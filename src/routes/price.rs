use axum::{Json, extract::Query};
use tracing::info;

use crate::models::{APIStatus, PriceParams, TCGPriceRequest};

pub async fn get(Query(params): Query<PriceParams>) -> Json<APIStatus> {
    info!(
        subject = %params.subject,
        rarity = ?params.rarity,
        product = ?params.product,
        "Fetching card prices"
    );

    let _price_request = TCGPriceRequest::new();

    Json(APIStatus {
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
