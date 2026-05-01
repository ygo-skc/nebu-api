use axum::{Json, extract::Query};
use reqwest::{Client, Response};
use serde::Deserialize;
use std::time::Instant;
use tracing::{error, info, warn};

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

    let tcg_price = {
        let tcg_req = {
            let mut tcg_req = TCGPriceRequest::default().with_yugioh_product_line();

            if let Some(rarities) = params.rarities {
                tcg_req = tcg_req.with_rarities(rarities.split(",").map(String::from).collect());
            }
            if let Some(sets) = params.sets {
                tcg_req = tcg_req.with_sets(sets.split(",").map(String::from).collect())
            }
            tcg_req.as_card()
        };

        let timer = Instant::now();
        let res = Client::new()
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
            })?;

        let tcg_price = handle_errors(res).await?;
        info!(duration_ms = timer.elapsed().as_millis(), "TCG request completed");
        tcg_price
    };

    Ok(Json(CardPriceResponse {
        prices: parse_tcg_card_prices(tcg_price, params.subject.as_str()),
    }))
}

async fn handle_errors(tcg_res: Response) -> Result<TCGPriceResponse, APIError> {
    if tcg_res.status() != 200 {
        error!(status_code = %tcg_res.status(), "Unexpected http code");
        return Err(APIError {
            message: "Error retrieving prices".to_string(),
        });
    }

    let body = tcg_res.json::<TCGPriceResponse>().await.map_err(|e| {
        error!(error = %e, "Failed to de-searialize response");
        APIError {
            message: "Error de-searializing price response".to_string(),
        }
    })?;

    let num_data_elements = body.data.len();
    if num_data_elements != 1 {
        error!(
            num_date_elements = num_data_elements,
            "Number of data elements isn't expected value 1"
        );
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
            let matches =
                price.product_name.to_lowercase().starts_with(subject_prefix) || price.product_name.to_lowercase().starts_with(subject);
            if !matches {
                warn!(name = price.product_name, "Item skipped");
            }
            matches
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
