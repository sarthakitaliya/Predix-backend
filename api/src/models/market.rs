use db::models::{
    close_order::ShareType,
    market::{Market, MarketStatus},
};
use matching::types::Side;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct MarketByIdResponse {
    pub market: Market,
}

#[derive(Deserialize, Debug)]
pub struct MarketsByStatusQuery {
    pub status: MarketStatus,
}

#[derive(Serialize, Debug)]
pub struct MarketsByStatusResponse {
    pub markets: Vec<Market>,
}
#[derive(Deserialize, Debug)]
pub struct ApproveRequest {
    pub market_id: String,
    pub side: Side, // "bid" or "ask"
    pub share: ShareType,
    pub amount: u64,
    pub decimals: u8,
}

#[derive(Serialize, Debug)]
pub struct ApproveRes {
    pub tx_message: String,
    pub recent_blockhash: String,
}
