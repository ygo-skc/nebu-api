use axum::{Json, extract::Query};
use reqwest::Client;
use tracing::error;
use tracing::info;

use crate::models::Config;
use crate::models::{APIError, APIStatus, PriceParams, TCGPriceRequest};

pub async fn get(Query(params): Query<PriceParams>) -> Result<Json<APIStatus>, APIError> {
    info!(
        subject = %params.subject,
        rarity = ?params.rarity,
        product = ?params.product,
        "Fetching card prices"
    );

    let mut tcg_req =
        TCGPriceRequest::defaults().with_filter_term("productLineName", vec!["yugioh".to_string()]);

    if let Some(rarity) = params.rarity {
        tcg_req = tcg_req.with_filter_term("rarityName", vec![rarity.to_string()]);
    }

    let tcg_res = Client::new()
        .post(format!(
            "https://{}/v1/search/request",
            Config::load().tcg_price_api_host
        ))
        .query(&[("q", params.subject), ("isList", "false".to_string())])
        .json(&tcg_req)
        .send()
        .await
        .map_err(|e| {
            error!("request failed: {e}");
            APIError {
                message: "Error retrieving prices".to_string(),
            }
        })?;

    if tcg_res.status() != 200 {
        error!("Expected 200 code, but received {}", tcg_res.status());
        return Err(APIError {
            message: "Error retrieving prices".to_string(),
        });
    }

    let res_str = tcg_res.text().await.map_err(|e| {
        error!("failed to read response body: {e}");
        APIError {
            message: "Error reading response".to_string(),
        }
    })?;
    info!(res_str);

    Ok(Json(APIStatus {
        version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}
