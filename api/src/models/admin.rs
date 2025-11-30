use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug)]
pub struct CreateMarketRequest {
    pub market_id: u64,
    pub metadata: String,
    pub collateral_mint: String,
    pub expiration_timestamp: i64,
}

#[derive(Serialize, Debug)]
pub struct CreateMarketResponse {
    pub market_id: u64,
    pub message: String,
}