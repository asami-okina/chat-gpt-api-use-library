use axum::{routing::get, Router};

mod open_ai;
#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/chat/completions", get(open_ai::chat::handler_chat));
    // localhost:3000 で hyper と共に実行する
    axum::Server::bind(&"0.0.0.0:3015".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
