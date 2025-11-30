use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ApproveRequest {
    pub market_id: u64,
    pub mint: String,
    pub user_ata: String,
    pub program_id: String,
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
