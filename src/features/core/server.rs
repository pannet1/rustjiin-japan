use askama::Template;
use axum::{
    extract::State,
    response::Html,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

use crate::features::llm::controller::LlmController;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[derive(Template)]
#[template(path = "mascot.html")]
struct MascotTemplate {
    chat: String,
    challenge: String,
}

#[derive(Template)]
#[template(path = "idle.html")]
struct IdleTemplate {}

struct AppState {
    llm: LlmController,
}

pub async fn start_axum_server() {
    let state = Arc::new(AppState {
        llm: LlmController::new(),
    });

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("icons"))
        .route("/", get(index))
        .route("/interact", post(interact))
        .route("/idle", get(idle))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Axum Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<String> {
    let template = IndexTemplate {};
    Html(template.render().unwrap())
}

async fn idle() -> Html<String> {
    let template = IdleTemplate {};
    Html(template.render().unwrap())
}

#[derive(serde::Deserialize)]
struct InteractPayload {
    transcription: Option<String>,
}

async fn interact(
    State(state): State<Arc<AppState>>,
    axum::extract::Form(payload): axum::extract::Form<InteractPayload>,
) -> Html<String> {
    let transcription = payload.transcription.unwrap_or_default();
    println!("Received transcription: '{}'", transcription);
    
    match state.llm.process_user_input(&transcription).await {
        Ok(llm_resp) => {
            let template = MascotTemplate {
                chat: llm_resp.chat,
                challenge: llm_resp.challenge,
            };
            Html(template.render().unwrap())
        },
        Err(e) => {
            println!("LLM Error: {}", e);
            let template = MascotTemplate {
                chat: format!("Error: {}", e),
                challenge: "".to_string(),
            };
            Html(template.render().unwrap())
        }
    }
}




