use axum::{Json, extract::Query};
use reqwest::Client;
use reqwest::Response;
use serde::Deserialize;
use tracing::error;
use tracing::info;

use crate::models::{APIError, CardPrice, CardPriceResponse, Config, TCGPriceRequest, TCGPriceResponse};

#[derive(Deserialize)]
pub struct PriceParams {
    subject: String,
    rarities: Option<String>,
    sets: Option<String>,
}

pub async fn get_card_prices(Query(params): Query<PriceParams>) -> Result<Json<CardPriceResponse>, APIError> {
    info!(
        subject = %params.subject,
        rarities = params.rarities,
        sets = params.sets,
        "Fetching card prices"
    );

    let res = {
        let mut tcg_req = TCGPriceRequest::default().with_yugioh_product_line();

        if let Some(rarities) = params.rarities {
            tcg_req = tcg_req.with_rarities(vec![rarities.to_string()]);
        }

        Client::new()
            .post(format!("https://{}/v1/search/request", Config::load().tcg_price_api_host))
            .query(&[("q", params.subject), ("isList", "false".to_string())])
            .json(&tcg_req)
            .send()
            .await
            .map_err(|e| {
                error!("request failed: {e}");
                APIError {
                    message: "Error retrieving prices".to_string(),
                }
            })?
    };

    let tcg_price_res = handle_errors(res).await?;
    let card_prices: Vec<CardPrice> = tcg_price_res.data[0]
        .results
        .iter()
        .filter_map(|item| {
            Some(CardPrice {
                set: item.set_name.clone(),
                rarity: item.rarity_name.clone()?,
                market_price: item.market_price.unwrap_or(item.lowest_price_with_shipping.unwrap_or_default()),
            })
        })
        .collect();

    Ok(Json(CardPriceResponse { prices: card_prices }))
}

async fn handle_errors(tcg_res: Response) -> Result<TCGPriceResponse, APIError> {
    if tcg_res.status() != 200 {
        error!("Expected 200 code, but received {}", tcg_res.status());
        return Err(APIError {
            message: "Error retrieving prices".to_string(),
        });
    }

    let body = tcg_res.json::<TCGPriceResponse>().await.map_err(|e| {
        error!("Failed to de-searialize response: {e}");
        APIError {
            message: "Error de-searializing price response".to_string(),
        }
    })?;

    let num_data_elements = body.data.len();
    if num_data_elements != 1 {
        error!("Number of data elements isn't 1 as expected. It's {}", num_data_elements,);
        return Err(APIError {
            message: "Unexpected TCG price response state".to_string(),
        });
    }

    Ok(body)
}
