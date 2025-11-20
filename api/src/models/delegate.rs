use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ApproveRequest {
    pub market_id: String,
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
