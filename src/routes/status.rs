use crate::models::APIStatus;
use axum::Json;

pub async fn get() -> Json<APIStatus> {
    tracing::info!("API status requested");
    Json(APIStatus {
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
