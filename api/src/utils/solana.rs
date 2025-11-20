use solana_sdk::pubkey::Pubkey;



pub fn derive_market_pda(market_id: &str, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"market", market_id.as_bytes()], program_id)
}