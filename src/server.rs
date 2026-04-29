use axum::{Router, body::Body, http::Request, middleware, middleware::Next, response::Response, routing::get, serve};
use reqwest::Method;
use tokio::net::TcpListener;
use tower_http::compression::{CompressionLayer, predicate::SizeAbove};
use tower_http::cors::{Any, CorsLayer};

use crate::routes::price;
use crate::routes::status;

pub async fn run() {
    let port = 9030;
    tracing::info!("API starting on port {}", port);

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse().unwrap(),
            "https://dev.thesupremekingscastle.com"
                .parse()
                .unwrap(),
            "https://thesupremekingscastle.com"
                .parse()
                .unwrap(),
            "https://www.thesupremekingscastle.com"
                .parse()
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
        .layer(middleware::from_fn(common_res_headers))
        .layer(cors)
        .layer(CompressionLayer::new().compress_when(SizeAbove::new(1024)));

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    serve(listener, app).await.unwrap();
}

async fn common_res_headers(req: Request<Body>, next: Next) -> Response {
    let mut res = next.run(req).await;

    let headers = res.headers_mut();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Cache-Control", "max-age=300".parse().unwrap());

    res
}
