use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PriceParams {
    pub subject: String,
    pub rarity: Option<String>,
    pub product: Option<String>,
}

#[derive(Serialize)]
pub struct TCGPriceRequest {
    algorithm: String,
    from: u8,
    size: u8,
}

impl TCGPriceRequest {
    pub fn new() -> Self {
        Self {
            algorithm: "sales_dismax".to_string(),
            from: 0,
            size: 30,
        }
    }
}
