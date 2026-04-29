use crate::models::APIStatus;
use axum::Json;
use tracing::info;

pub async fn get_status() -> Json<APIStatus> {
    info!("API status requested");
    Json(APIStatus {
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
