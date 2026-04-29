use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CardPriceResponse {
    pub prices: Vec<CardPrice>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CardPrice {
    pub set: String,
    pub rarity: String,
    pub market_price: f32,
}
