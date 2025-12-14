use anchor_client_sdk::{MatchFill, predix_program::types::MarketOutcome};
use anchor_lang::prelude::*;

#[derive(Debug, AnchorDeserialize)]
pub struct MarketInitialized {
    pub market_id: u64,
    pub market_pda: Pubkey,
    pub authority: Pubkey,
    pub collateral_mint: Pubkey,
    pub collateral_vault: Pubkey,
    pub metadata_url: String,
    pub yes_mint: Pubkey,
    pub no_mint: Pubkey,
    pub expiration_timestamp: i64,
}

#[derive(Debug, AnchorDeserialize)]
pub struct MatchExecuted {
    pub market_id: u64,
    pub market_pda: Pubkey,
    pub admin: Pubkey,
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub fills_executed: Vec<MatchFill>,
}
#[derive(Debug, AnchorDeserialize)]
pub struct TokensSplit {
    pub market_id: u64,
    pub market_pda: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
}
#[derive(Debug, AnchorDeserialize)]
pub struct TokensMerged {
    pub market_id: u64,
    pub market_pda: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
}
#[derive(Debug, AnchorDeserialize)]
pub struct RewardsClaimed {
    pub market_id: u64,
    pub market_pda: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
}
#[derive(Debug, AnchorDeserialize)]
pub struct MarketSettled {
    pub market_id: u64,
    pub market_pda: Pubkey,
    pub outcome: MarketOutcome,
}