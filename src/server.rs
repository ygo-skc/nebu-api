use crate::routes::status;
use axum::{Router, routing::get, serve};
use tokio::net::TcpListener;

pub async fn run() {
    tracing::info!("API starting on port 8080");
    let app = Router::new().route("/status", get(status::get));
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    serve(listener, app).await.unwrap();
}
