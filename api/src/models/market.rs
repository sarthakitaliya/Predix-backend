
use db::models::market::{Market, MarketStatus};
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