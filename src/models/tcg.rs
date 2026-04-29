use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PriceParams {
    pub subject: String,
    pub rarities: Option<String>,
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

#[allow(dead_code)]
impl TCGPriceRequest {
    pub fn with_yugioh_product_line(mut self) -> Self {
        self.filters
            .term
            .insert("productLineName".to_string(), vec!["yugioh".to_string()]);
        self
    }

    pub fn with_rarities(mut self, rarities: Vec<String>) -> Self {
        self.filters.term.insert("rarityName".to_string(), rarities);
        self
    }

    pub fn with_sets(mut self, sets: Vec<String>) -> Self {
        self.filters.term.insert("setName".to_string(), sets);
        self
    }
}

impl Default for TCGPriceRequest {
    fn default() -> Self {
        Self {
            algorithm: "sales_dismax".to_string(),
            from: 0,
            size: 30,
            filters: TCGPriceFilters::default(),
            settings: TCGPriceSettings::default(),
            sort: TCGPriceSortOptions::default(),
            context: TCGPriceContext::default(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TCGPriceFilters {
    term: HashMap<String, Vec<String>>,
}

impl Default for TCGPriceFilters {
    fn default() -> Self {
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

impl Default for TCGPriceSettings {
    fn default() -> Self {
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

impl Default for TCGPriceSortOptions {
    fn default() -> Self {
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

impl Default for TCGPriceContext {
    fn default() -> Self {
        Self {
            shipping_country: "US".to_string(),
        }
    }
}

/*
 * Response
 */

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct TCGPriceResponse {
    #[serde(rename = "results")]
    data: Vec<TCGPriceData>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct TCGPriceData {
    results: Vec<TCGPriceResults>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TCGPriceResults {
    lowest_price: Option<f32>,
    lowest_price_with_shipping: Option<f32>,
    market_price: Option<f32>,
}
