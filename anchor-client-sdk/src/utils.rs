use anchor_lang::prelude::Pubkey;

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