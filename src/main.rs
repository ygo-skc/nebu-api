mod models;
mod routes;
mod server;
use tracing_subscriber;

use crate::models::Config;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_thread_ids(true)
        .json()
        .init();

    Config::load();
    server::run().await;
}
