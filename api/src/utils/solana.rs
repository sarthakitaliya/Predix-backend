use std::str::FromStr;

use anchor_lang::{declare_program, prelude::Pubkey};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::program_pack::Pack;
use spl_associated_token_account::get_associated_token_address;
use spl_token::state::Account;

declare_program!(predix_program);

pub async fn verify_delegation(
    rpc: &RpcClient,
    wallet_address: &str,
    collateral_mint: &str,
) -> Result<(), anyhow::Error> {
    let program_id = predix_program::ID;
    let wallet_address = Pubkey::from_str(wallet_address).expect("Invalid wallet address");
    let collateral_mint =
        Pubkey::from_str(collateral_mint).expect("Invalid collateral mint address");

    let collateral_ata = get_associated_token_address(&wallet_address, &collateral_mint);

    let account_data = rpc
        .get_account_data(&collateral_ata)
        .await
        .expect("Failed to get account data");
    let account_data = Account::unpack(&account_data).expect("Failed to unpack account data");
    println!("Collateral ATA account data: {:?}", account_data);
    dbg!("Program ID: {:?}", program_id);
    dbg!("Wallet Address: {:?}", wallet_address);
    dbg!("Collateral Mint: {:?}", collateral_mint);
    dbg!("Collateral ATA: {:?}", collateral_ata);
    dbg!("Account Data: {:?}", account_data);

    Ok(())
}

pub fn derive_market_pda(market_id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"market", &market_id.to_le_bytes().as_ref()], &predix_program::ID)
}
