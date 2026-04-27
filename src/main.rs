mod models;
mod routes;
mod server;
use tikv_jemallocator::Jemalloc;
use tracing_subscriber;

use crate::models::Config;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

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
