use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug)]
pub struct CreateMarketRequest {
    pub metadata: MarketMetadata,
    pub collateral_mint: String,
    pub expiration_timestamp: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MarketMetadata {
    pub title: String,
    pub description: Option<String>,
    pub category: String,
    pub image_url: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct CreateMarketResponse {
    pub market_id: u64,
    pub message: String,
}