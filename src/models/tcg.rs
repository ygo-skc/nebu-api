use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PriceParams {
    pub subject: String,
    pub rarity: Option<String>,
    pub product: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TCGPriceRequest {
    algorithm: String,
    from: u8,
    size: u8,
    filters: TCGPriceFilters,
    settings: TCGPriceSettings,
    sort: TCGPriceSortOptions,
    context: TCGPriceContext,
}

impl TCGPriceRequest {
    pub fn defaults() -> Self {
        Self {
            algorithm: "sales_dismax".to_string(),
            from: 0,
            size: 30,
            filters: TCGPriceFilters::defaults(),
            settings: TCGPriceSettings::defaults(),
            sort: TCGPriceSortOptions::market_price_defaults(),
            context: TCGPriceContext::us_defaults(),
        }
    }

    pub fn with_filter_term(mut self, filter_name: &str, filter_values: Vec<String>) -> Self {
        self.filters
            .term
            .insert(filter_name.to_string(), filter_values);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TCGPriceFilters {
    term: HashMap<String, Vec<String>>,
}

impl TCGPriceFilters {
    fn defaults() -> Self {
        Self {
            term: HashMap::new(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TCGPriceSettings {
    use_fuzzy_search: bool,
}

impl TCGPriceSettings {
    fn defaults() -> Self {
        Self {
            use_fuzzy_search: false,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TCGPriceSortOptions {
    field: String,
    order: String,
}

impl TCGPriceSortOptions {
    fn market_price_defaults() -> Self {
        Self {
            field: "market-price".to_string(),
            order: "desc".to_string(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TCGPriceContext {
    shipping_country: String,
}

impl TCGPriceContext {
    fn us_defaults() -> Self {
        Self {
            shipping_country: "US".to_string(),
        }
    }
}
