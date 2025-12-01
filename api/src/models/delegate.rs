use matching::types::Side;
use serde::{Deserialize, Serialize};

use crate::models::orderbook::ShareType;

#[derive(Deserialize, Debug)]
pub struct ApproveRequest {
    pub market_id: u64,
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

#[derive(Deserialize, Debug)]
pub struct CheckRequest {
    pub collateral_mint: String,
    pub market_id: u64,
}
#[derive(Serialize, Debug)]
pub struct CheckResponse {
    pub message: String,
}
