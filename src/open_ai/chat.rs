use axum::response::Json;
use chatgpt::prelude::ChatGPT;
use serde_json::{json, Value};
use std::env;

pub async fn handler_chat(body_json: Json<Value>) -> Json<Value> {
    let open_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");
    let content = body_json.0.get("content").unwrap().as_str().unwrap();
    let response = chat(&open_api_key, &content).await.unwrap();
    Json(json!({ "response": response }))
}

async fn chat(open_api_key: &str, content: &str) -> Result<String, anyhow::Error> {
    let client = ChatGPT::new(open_api_key)?;
    let mut conversation = client.new_conversation();
    let response = conversation.send_message(content).await?;
    let content = response.message().content.to_string();
    Ok(content)
}
