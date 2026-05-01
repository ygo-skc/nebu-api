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
            tcg_req = tcg_req.with_rarities(rarities.split(",").map(String::from).collect());
        }
        if let Some(sets) = params.sets {
            tcg_req = tcg_req.with_sets(sets.split(",").map(String::from).collect())
        }

        let tcg_req = tcg_req;
        Client::new()
            .post(format!("https://{}/v1/search/request", Config::load().tcg_price_api_host))
            .query(&[("q", params.subject.as_str()), ("isList", "false")])
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

    Ok(Json(CardPriceResponse {
        prices: parse_tcg_card_prices(handle_errors(res).await?, &params.subject),
    }))
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

fn parse_tcg_card_prices(tcg_prices: TCGPriceResponse, subject: &str) -> Vec<CardPrice> {
    let subject_prefix = &format!("{} (", subject.to_lowercase());

    tcg_prices.data[0]
        .results
        .iter()
        .filter(|price| {
            price.product_name.to_lowercase().starts_with(subject_prefix) || price.product_name.to_lowercase().starts_with(subject)
        })
        .filter_map(|price| {
            let lowest_price = price.lowest_price?;

            Some(CardPrice {
                set: price.set_name.clone(),
                rarity: price.rarity_name.clone()?,
                lowest_price: price.lowest_price_with_shipping.unwrap_or(lowest_price),
                market_price: price.market_price.unwrap_or(lowest_price),
            })
        })
        .collect()
}
