use matching::types::{Side, Trade};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub enum ShareType {
    Yes,
    No,
}

#[derive(Deserialize)]
pub struct PlaceOrderReq {
    pub user_id: String,
    pub market_id: String,
    pub side: Side, // "bid" or "ask"
    pub share: ShareType,
    pub price: String,
    pub qty: String,
}

#[derive(Serialize)]
pub struct PlaceOrderRes {
    pub order_id: Uuid,
    pub trades: Vec<Trade>,
    pub remaining_qty: Decimal,
}

#[derive(Deserialize)]
pub struct SplitOrderReq {
    pub market_id: u64,
    pub collateral_mint: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct SplitOrderRes {
    pub tx_message: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct MergeOrderReq {
    pub market_id: u64,
    pub collateral_mint: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct MergeOrderRes {
    pub tx_message: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct CancelReq {
    pub market_id: String,
    pub order_id: Uuid,
    pub side: Side, // "bid" or "ask"
    pub share: ShareType,
    pub price: Decimal,
}
#[derive(Serialize)]
pub struct CancelRes {
    pub success: bool,
    pub message: String,
}
