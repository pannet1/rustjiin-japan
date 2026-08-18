use rusqlite::{params, Connection, functions::FunctionFlags};
use reqwest::Client;
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::convert::TryInto;

pub struct MemoryDB {
    conn: Arc<Mutex<Connection>>,
    client: Client,
}

impl MemoryDB {
    pub fn new() -> Self {
        let conn = Connection::open("itachi_memory.db").unwrap();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS memories (
                id INTEGER PRIMARY KEY,
                content TEXT NOT NULL,
                embedding BLOB NOT NULL
            )",
            [],
        ).unwrap();

        // Create cosine similarity function
        conn.create_scalar_function(
            "cosine_similarity",
            2,
            FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
            |ctx| {
                let v1: Vec<u8> = ctx.get(0)?;
                let v2: Vec<u8> = ctx.get(1)?;
                
                if v1.len() != v2.len() || v1.is_empty() {
                    return Ok(0.0_f64);
                }

                let mut dot_product = 0.0_f32;
                let mut norm1 = 0.0_f32;
                let mut norm2 = 0.0_f32;
                
                for i in (0..v1.len()).step_by(4) {
                    let f1 = f32::from_le_bytes(v1[i..i+4].try_into().unwrap());
                    let f2 = f32::from_le_bytes(v2[i..i+4].try_into().unwrap());
                    dot_product += f1 * f2;
                    norm1 += f1 * f1;
                    norm2 += f2 * f2;
                }
                
                if norm1 == 0.0 || norm2 == 0.0 {
                    return Ok(0.0_f64);
                }
                
                Ok((dot_product / (norm1.sqrt() * norm2.sqrt())) as f64)
            },
        ).unwrap();

        Self {
            conn: Arc::new(Mutex::new(conn)),
            client: Client::new(),
        }
    }

    async fn get_embedding(&self, text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
        let payload = json!({
            "model": "nomic-embed-text", // Common generic name, llama-swap might ignore it if it's default
            "input": text
        });

        // Use the local embedding endpoint
        let response = self.client.post("http://localhost:8080/v1/embeddings")
            .json(&payload)
            .send()
            .await?;

        let data: serde_json::Value = response.json().await?;
        
        if let Some(err) = data.get("error") {
            return Err(format!("LLM API Error: {}", err).into());
        }
        
        let embedding = data["data"][0]["embedding"]
            .as_array()
            .ok_or("Failed to parse embedding array")?
            .iter()
            .map(|v| v.as_f64().unwrap() as f32)
            .collect();
            
        Ok(embedding)
    }

    fn vec_to_bytes(vec: &[f32]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(vec.len() * 4);
        for &f in vec {
            bytes.extend_from_slice(&f.to_le_bytes());
        }
        bytes
    }

    pub async fn remember(&self, content: &str) {
        if content.trim().is_empty() { return; }
        
        match self.get_embedding(content).await {
            Ok(emb) => {
                let bytes = Self::vec_to_bytes(&emb);
                let conn = self.conn.lock().unwrap();
                let _ = conn.execute(
                    "INSERT INTO memories (content, embedding) VALUES (?1, ?2)",
                    params![content, bytes],
                );
            },
            Err(e) => println!("Warning: Memory embedding failed. {}", e),
        }
    }

    pub async fn recall(&self, query: &str) -> String {
        if query.trim().is_empty() { return "".to_string(); }

        let emb = match self.get_embedding(query).await {
            Ok(e) => e,
            Err(_) => return "".to_string(), // Silently fail if no embedding model is loaded
        };

        let bytes = Self::vec_to_bytes(&emb);
        let conn = self.conn.lock().unwrap();
        
        let mut stmt = match conn.prepare(
            "SELECT content FROM memories 
             ORDER BY cosine_similarity(embedding, ?1) DESC 
             LIMIT 3"
        ) {
            Ok(s) => s,
            Err(_) => return "".to_string(),
        };

        let mut rows = match stmt.query(params![bytes]) {
            Ok(r) => r,
            Err(_) => return "".to_string(),
        };

        let mut context = Vec::new();
        while let Ok(Some(row)) = rows.next() {
            let content: String = row.get(0).unwrap();
            context.push(content);
        }

        context.join("\n---\n")
    }
}
