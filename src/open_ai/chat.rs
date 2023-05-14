use axum::{extract::Query, response::Json};
use chatgpt::prelude::ChatGPT;
use chatgpt::types::CompletionResponse;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize)]
pub struct FetchGptChatParams {
    content: String,
}

pub async fn handler_chat(Query(params): Query<HashMap<String, String>>) -> Json<Value> {
    let open_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");
    let content = match params.get("content") {
        Some(content) => content,
        None => panic!("content is not set"),
    };
    let response = chat(&open_api_key, &content).await.unwrap();
    Json(json!({ "response": response }))
}

async fn chat(open_api_key: &str, content: &str) -> Result<String, anyhow::Error> {
    let client = ChatGPT::new(open_api_key)?;
    let response: CompletionResponse = client.send_message(content).await?;
    Ok(response.message().content.to_string())
}
