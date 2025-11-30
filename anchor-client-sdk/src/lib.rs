use std::sync::Arc;

use anchor_client::{
    Client, Cluster, Program,
    solana_sdk::{
        commitment_config::CommitmentConfig, instruction::Instruction, signature::Keypair,
        signer::Signer,
    },
};
use anchor_lang::{
    declare_program,
    prelude::{Pubkey, system_program},
};
use anyhow::Result;
use solana_sdk::{message::Message, transaction::Transaction};

use crate::{
    predix_program::client::{accounts, args},
    utils::{
        derive_market_pda, derive_user_collateral_ata_pda, derive_yes_and_no_ata_pdas,
        derive_yes_and_no_mint_pdas, vault_pda,
    },
};

mod utils;

declare_program!(predix_program);

pub struct PredixSdk {
    keypair: Arc<Keypair>,
    program: Program<Arc<Keypair>>,
}

impl PredixSdk {
    pub fn new(payer_private_key: &str) -> Result<Self> {
        let key_pair = Keypair::from_base58_string(payer_private_key);
        let keypair_arc = Arc::new(key_pair);

        let provider = Client::new_with_options(
            Cluster::Devnet,
            keypair_arc.clone(),
            CommitmentConfig::confirmed(),
        );

        let program = provider.program(predix_program::ID)?;

        Ok(Self {
            keypair: keypair_arc,
            program,
        })
    }

    pub async fn create_market(
        &self,
        market_id: u64,
        collateral_mint: Pubkey,
        metadata: String,
        expiration_timestamp: i64,
    ) -> Result<()> {
        let (market_pda, _bump) = derive_market_pda(market_id, &predix_program::ID);
        let (vault_pda, _bump) = vault_pda(market_id, &predix_program::ID);
        let ((yes_mint_pda, _), (no_mint_pda, _)) =
            derive_yes_and_no_mint_pdas(market_id, &predix_program::ID);
        dbg!("Market PDA:", market_pda);
        dbg!("Vault PDA:", vault_pda);
        dbg!("Yes Mint PDA:", yes_mint_pda);
        dbg!("No Mint PDA:", no_mint_pda);

        let accounts = accounts::InitializeMarket {
            market: market_pda,
            vault: vault_pda,
            collateral_mint,
            yes_mint: yes_mint_pda,
            no_mint: no_mint_pda,
            admin: self.keypair.pubkey(),
            system_program: system_program::ID,
            token_program: spl_token::ID,
            associated_token_program: spl_associated_token_account::ID,
        };

        let args = args::InitializeMarket {
            market_id,
            metadata,
            expiration_timestamp,
        };
        dbg!("Creating market with ID: {}", market_id);

        let tx = self
            .program
            .request()
            .accounts(accounts)
            .args(args)
            .send()
            .await?;

        println!("Transaction signature: {}", tx);
        Ok(())
    }

    pub async fn split_order(
        &self,
        market_id: u64,
        user_wallet: &Pubkey,
        collateral_mint: &Pubkey,
        amount: u64,
    ) -> Result<String> {
        let (market_pda, _bump) = derive_market_pda(market_id, &predix_program::ID);
        let (vault_pda, _bump) = vault_pda(market_id, &predix_program::ID);
        let ((yes_mint_pda, _), (no_mint_pda, _)) =
            derive_yes_and_no_mint_pdas(market_id, &predix_program::ID);
        let user_collateral_ata = derive_user_collateral_ata_pda(user_wallet, collateral_mint);
        let (user_yes_ata, user_no_ata) =
            derive_yes_and_no_ata_pdas(user_wallet, &yes_mint_pda, &no_mint_pda);
        dbg!("Market PDA:", market_pda);
        dbg!("Vault PDA:", vault_pda);
        dbg!("Yes Mint PDA:", yes_mint_pda);
        dbg!("No Mint PDA:", no_mint_pda);
        dbg!("User Collateral ATA:", user_collateral_ata);
        dbg!("User Yes ATA:", user_yes_ata);
        dbg!("User No ATA:", user_no_ata);
        let accounts = accounts::SplitToken {
            market: market_pda,
            user_collateral: user_collateral_ata,
            collateral_vault: vault_pda,
            yes_ata: user_yes_ata,
            no_ata: user_no_ata,
            yes_mint: yes_mint_pda,
            no_mint: no_mint_pda,
            user: *user_wallet,
            token_program: spl_token::ID,
            associated_token_program: spl_associated_token_account::ID,
            system_program: system_program::ID,
        };
        let payer = self.keypair.clone();

        let args = args::SplitToken { market_id, amount };

        let mut ix_vec = self
            .program
            .request()
            .accounts(accounts)
            .args(args)
            .instructions()?;
        dbg!("Split order ix: {:?}", &ix_vec);
        let recent_blockhash = self.program.rpc().get_latest_blockhash().await?;

        let message = Message::new(&[ix_vec.remove(0)], Some(&self.keypair.pubkey()));
        let mut tx = Transaction::new_unsigned(message);
        dbg!("Recent blockhash: {:?}", &recent_blockhash);
        tx.try_partial_sign(&[self.keypair.as_ref()], recent_blockhash)?;
        dbg!("Split order tx: {:?}", &tx);

        let serialized = bincode::serialize(&tx)?;
        #[allow(deprecated)]
        let tx_base64 = base64::encode(serialized);

        Ok(tx_base64)
    }
}
