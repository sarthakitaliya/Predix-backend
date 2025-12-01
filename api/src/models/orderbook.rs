use matching::types::{Side, Trade};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Clone, Copy, Debug)]
pub enum ShareType {
    Yes,
    No,
}

#[derive(Deserialize)]
pub struct PlaceOrderReq {
    pub market_id: u64,
    pub collateral_mint: String,
    pub side: Side, // "bid" or "ask"
    pub share: ShareType,
    pub price: Decimal,
    pub qty: Decimal,
}

#[derive(Serialize)]
pub struct PlaceOrderRes {
    pub order_id: Uuid,
    pub trades: Vec<Trade>,
    pub remaining_qty: Decimal,
    pub message: String,
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
    pub market_id: u64,
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
