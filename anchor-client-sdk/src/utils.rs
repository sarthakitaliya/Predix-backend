use anchor_lang::declare_program;
use anchor_lang::prelude::AccountMeta;
use anchor_lang::prelude::Pubkey;
use matching::types::Trade;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;


use crate::predix_program::types::TradeSide;
use crate::predix_program::types::{MatchFill};

declare_program!(predix_program);


pub fn derive_market_pda(market_id: u64, program_id: &Pubkey) -> (Pubkey, u8) {
    let market_id_bytes = market_id.to_le_bytes();
    let seeds = &[b"market", market_id_bytes.as_ref()];
    Pubkey::find_program_address(seeds, program_id)
}
pub fn vault_pda(market_id: u64, program_id: &Pubkey) -> (Pubkey, u8) {
    let market_id_bytes = market_id.to_le_bytes();
    let seeds = &[b"collateral_vault", market_id_bytes.as_ref()];
    Pubkey::find_program_address(seeds, program_id)
}
pub fn derive_yes_and_no_mint_pdas(market_id: u64, program_id: &Pubkey) -> ((Pubkey, u8), (Pubkey, u8)) {
    let market_id_bytes = market_id.to_le_bytes();
    let yes_seeds = &[b"yes_mint", market_id_bytes.as_ref()];
    let no_seeds = &[b"no_mint", market_id_bytes.as_ref()];
    let yes_mint_pda = Pubkey::find_program_address(yes_seeds, program_id);
    let no_mint_pda = Pubkey::find_program_address(no_seeds, program_id);
    (yes_mint_pda, no_mint_pda)
}

pub fn derive_user_collateral_ata_pda(
    user_wallet: &Pubkey,
    collateral_mint: &Pubkey,
) -> Pubkey {
    spl_associated_token_account::get_associated_token_address(user_wallet, collateral_mint)
}

pub fn derive_yes_and_no_ata_pdas(
    user_wallet: &Pubkey,
    yes_mint: &Pubkey,
    no_mint: &Pubkey,
) -> (Pubkey, Pubkey) {
    let yes_ata = spl_associated_token_account::get_associated_token_address(user_wallet, yes_mint);
    let no_ata = spl_associated_token_account::get_associated_token_address(user_wallet, no_mint);
    (yes_ata, no_ata)
}

pub fn derive_yes_ata(
    user_wallet: &Pubkey,
    yes_mint: &Pubkey,
) -> Pubkey {
    spl_associated_token_account::get_associated_token_address(user_wallet, yes_mint)
}

pub fn derive_no_ata(
    user_wallet: &Pubkey,
    no_mint: &Pubkey,
) -> Pubkey {
    spl_associated_token_account::get_associated_token_address(user_wallet, no_mint)
}   

fn to_u64_amount(dec: Decimal) -> u64{
    (dec * Decimal::new(1_000_000, 0)).to_u64().unwrap()
}

pub fn get_match_fills(trade: &Vec<Trade>, side: TradeSide) -> Vec<MatchFill> {
    let mut match_fills: Vec<MatchFill> = Vec::new();
    for t in trade.iter() {
        match_fills.push(MatchFill {
            shares: to_u64_amount(t.quantity),
            price: to_u64_amount(t.price),
            side,
        });
    }
    match_fills
}

pub fn get_remaining_accounts(trade: &Vec<Trade>, side: TradeSide, market_id: u64) -> Vec<AccountMeta> {
    let mut remaining_accounts: Vec<AccountMeta> = Vec::new();
    let market_pda = derive_market_pda(market_id, &predix_program::ID);
    dbg!("Market PDA in remaining accounts:", market_pda.0);
    for t in trade.iter() {
        let (yes_mint_pda, no_mint_pda) = derive_yes_and_no_mint_pdas(market_id, &predix_program::ID);
        let buyer_pubkey = t.buyer_address.parse::<Pubkey>().unwrap();
        let seller_pubkey = t.seller_address.parse::<Pubkey>().unwrap();
        let collateral_mint = Pubkey::from_str("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU").unwrap();
        let buyer_collateral = derive_user_collateral_ata_pda(&buyer_pubkey, &collateral_mint);
        let seller_collateral = derive_user_collateral_ata_pda(&seller_pubkey, &collateral_mint);
        let buyer_ata;
        let seller_ata;
        match side {
            TradeSide::Yes => {
                buyer_ata = derive_yes_ata(&buyer_pubkey, &yes_mint_pda.0);
                seller_ata = derive_yes_ata(&seller_pubkey, &yes_mint_pda.0);
            }
            TradeSide::No => {
                buyer_ata = derive_no_ata(&buyer_pubkey, &no_mint_pda.0);
                seller_ata = derive_no_ata(&seller_pubkey, &no_mint_pda.0);
            }
        }
        let buyer_account = t.buyer_address.parse::<Pubkey>().unwrap();
        let seller_account = t.seller_address.parse::<Pubkey>().unwrap();
        remaining_accounts.push(AccountMeta {
            pubkey: buyer_collateral,
            is_signer: false,
            is_writable: true,
        });
        remaining_accounts.push(AccountMeta {
            pubkey: seller_collateral,
            is_signer: false,
            is_writable: true,
        });
        remaining_accounts.push(AccountMeta {
            pubkey: buyer_ata,
            is_signer: false,
            is_writable: true,
        });
        remaining_accounts.push(AccountMeta {
            pubkey: seller_ata,
            is_signer: false,
            is_writable: true,
        });
        remaining_accounts.push(AccountMeta {
            pubkey: buyer_account,
            is_signer: false,
            is_writable: false,
        });
        remaining_accounts.push(AccountMeta {
            pubkey: seller_account,
            is_signer: false,
            is_writable: false,
        });
    }
    remaining_accounts
}