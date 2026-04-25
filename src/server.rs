use axum::{routing::get, Router, serve};
use tokio::net::TcpListener;
use crate::routes::status;

pub async fn run() {
    let app = Router::new()
        .route("/status", get(status::get));
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    serve(listener, app).await.unwrap();
}
