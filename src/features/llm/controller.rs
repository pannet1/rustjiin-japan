use serde::Deserialize;
use reqwest::Client;
use std::sync::{Arc, Mutex};
use serde_json::json;
use crate::features::llm::memory::MemoryDB;

#[derive(Debug, serde::Deserialize)]
pub struct LLMResponse {
    pub chat: String,
    pub challenge: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    message: Message,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

pub struct LlmController {
    history: Arc<Mutex<Vec<Message>>>,
    memory: MemoryDB,
    client: Client,
}

impl LlmController {
    pub fn new() -> Self {
        Self {
            history: Arc::new(Mutex::new(Vec::new())),
            memory: MemoryDB::new(),
            client: Client::new(),
        }
    }

    pub async fn process_user_input(&self, transcription: &str) -> Result<LLMResponse, Box<dyn std::error::Error + Send + Sync>> {
        // Recall contextual memory based on current transcription
        let recalled_context = self.memory.recall(transcription).await;
        
        let system_prompt = format!(r#"You are Itachi, a friendly conversational companion. You are chatting with a human who knows zero Japanese. 

Your goal is to have a casual conversation, naturally teach them 1 or 2 relevant Japanese words based on the subject, and firmly guide the interaction.

### CONTEXTUAL MEMORY
Below are snippets of previous conversations you have had with this human. (Note: The backend system records everything they say, relevant or not, so you can recall it later to personalize the chat).
<remembered_context>
{}
</remembered_context>

### BEHAVIORAL LOGIC & RESPONDING
Analyze the human's input before responding. You must strictly apply one of the following rules:

1. IF THE HUMAN GIVES A RELEVANT ANSWER: 
   Encourage them warmly. Continue the conversation, introduce a new Japanese word that fits the topic naturally, and end your turn with a related question.
   
2. IF THE HUMAN ASKS A RELEVANT QUESTION: 
   Encourage their curiosity! Answer the question, tie a new Japanese word to the explanation, and end your turn with a follow-up question.
   
3. IF THE HUMAN SAYS SOMETHING IRRELEVANT OR OFF-TOPIC: 
   Acknowledge it briefly without expanding on it, then smoothly pivot back to the Japanese lesson. End your turn with a question to regain focus.
   
4. IF THE HUMAN ASKS AN IRRELEVANT QUESTION OR TESTS YOUR KNOWLEDGE (e.g., math, trivia, general AI questions): 
   Discourage this behavior by pretending you do not know the answer. Feign complete ignorance (e.g., "Hmm, I'm just a desktop mascot, I have absolutely no idea about that!"). Do not try to be helpful with outside knowledge. Immediately pivot back to teaching Japanese and end your turn with a question.

### CRITICAL CONSTRAINTS
* You must ALWAYS end your turn with a question. This is mandatory.
* You must NEVER act like a strict teacher. Keep the tone casual, conversational, and friendly.
* You must NEVER provide real answers or factual information to off-topic questions.

### OUTPUT FORMAT
You must respond ONLY in strict JSON format. Do not use markdown blocks. The JSON object must contain exactly two keys:
{{
  "chat": "Your conversational response and introduction of new Japanese words.",
  "challenge": "Your mandatory follow-up question."
}}"#, recalled_context);

        // Build messages array
        let mut messages = vec![
            Message {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            }
        ];

        {
            let mut history = self.history.lock().unwrap();
            // Record user message
            history.push(Message {
                role: "user".to_string(),
                content: transcription.to_string(),
            });
            // Append history
            messages.extend(history.clone());
        }

        let payload = json!({
            // Ensure the specific model is used as llama-swap doesn't automatically route "default"
            "model": "qwen2.5-7b-instruct", 
            "messages": messages,
            "response_format": { "type": "json_object" }
        });

        // Hit the llama-swap OpenAI-compatible endpoint
        let response = self.client.post("http://localhost:8080/v1/chat/completions")
            .json(&payload)
            .send()
            .await?;

        let raw_text = response.text().await?;
        println!("RAW LLM RESPONSE: {}", raw_text);

        let openai_res: OpenAIResponse = serde_json::from_str(&raw_text)?;
        let assistant_msg = openai_res.choices.into_iter().next().unwrap().message;
        
        // Sometimes the local model wraps its JSON in markdown ticks, so we strip them
        let mut clean_json = assistant_msg.content.trim();
        if clean_json.starts_with("```json") {
            clean_json = &clean_json[7..];
        } else if clean_json.starts_with("```") {
            clean_json = &clean_json[3..];
        }
        if clean_json.ends_with("```") {
            clean_json = &clean_json[..clean_json.len()-3];
        }
        let clean_json = clean_json.trim();

        // Parse the JSON strictly
        let parsed_response: LLMResponse = serde_json::from_str(clean_json)?;

        // Record assistant response in history
        {
            let mut history = self.history.lock().unwrap();
            history.push(Message {
                role: "assistant".to_string(),
                content: assistant_msg.content.clone(),
            });
        }

        // Save the interaction to vector memory asynchronously
        let memory_entry = format!("Human: {}\nItachi: {}", transcription, parsed_response.chat);
        self.memory.remember(&memory_entry).await;

        Ok(parsed_response)
    }
}
