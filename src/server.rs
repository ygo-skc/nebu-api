use crate::routes::status;
use axum::{Router, routing::get, serve};
use tokio::net::TcpListener;

pub async fn run() {
    let port = 9030;

    tracing::info!("API starting on port {}", port);
    let app = Router::new().route("/status", get(status::get));
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    serve(listener, app).await.unwrap();
}
