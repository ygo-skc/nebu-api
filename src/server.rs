use axum::{routing::get, Router, serve};
use tokio::net::TcpListener;

pub async fn run() {
    let app = Router::new()
        .route("/status", get(status));
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    serve(listener, app).await.unwrap();
}

async fn status() -> &'static str {
    "OK"
}
