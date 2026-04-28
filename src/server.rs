use axum::http::HeaderValue;
use axum::{Router, routing::get, serve};
use reqwest::Method;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use crate::routes::price;
use crate::routes::status;

pub async fn run() {
    let port = 9030;
    tracing::info!("API starting on port {}", port);

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "https://dev.thesupremekingscastle.com"
                .parse::<HeaderValue>()
                .unwrap(),
            "https://thesupremekingscastle.com"
                .parse::<HeaderValue>()
                .unwrap(),
            "https://www.thesupremekingscastle.com"
                .parse::<HeaderValue>()
                .unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    let app = Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .route("/status", get(status::get))
                .route("/prices", get(price::get)),
        )
        .layer(cors);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    serve(listener, app).await.unwrap();
}
