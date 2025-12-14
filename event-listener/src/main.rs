use anchor_client_sdk::predix_program::types::MarketOutcome;
use anchor_lang::prelude::*;
use chrono::Utc;
use db::{
    Db,
    models::market::{self, MarketStatus},
    queries::market::{create_market, update_market_resolution},
};
use std::{path::Path, str::FromStr};

use anyhow::Result;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
};
use solana_sdk::pubkey::Pubkey;

use tokio_stream::StreamExt;

use crate::types::MarketInitialized;
use dotenvy::from_path;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    from_path(Path::new("../.env")).ok();

    let database_url = std::env::var("DATABASE_URL")?;
    let program_id = std::env::var("PROGRAM_ID")?;
    let rpc_url =
        std::env::var("SOLANA_WS_RPC_URL").unwrap_or("wss://api.devnet.solana.com/".to_string());

    let program_id =
        Pubkey::from_str(&program_id).map_err(|e| anyhow::anyhow!("Invalid program id: {}", e))?;
    let pool = Db::new(&database_url).await?.pool;
    let client = PubsubClient::new(&rpc_url).await?;
    let filter = RpcTransactionLogsFilter::Mentions(vec![program_id.to_string()]);
    let config = RpcTransactionLogsConfig { commitment: None };
    let (mut log_stream, _unsubscribe) = client.logs_subscribe(filter, config).await?;

    let initialized_discriminator =
        solana_program::hash::hashv(&[b"event:MarketInitialized"]).to_bytes()[..8].to_vec();
    let match_executed_discriminator =
        solana_program::hash::hashv(&[b"event:MatchExecuted"]).to_bytes()[..8].to_vec();
    let tokens_split_discriminator =
        solana_program::hash::hashv(&[b"event:TokensSplit"]).to_bytes()[..8].to_vec();
    let tokens_merged_discriminator =
        solana_program::hash::hashv(&[b"event:TokensMerged"]).to_bytes()[..8].to_vec();
    let rewards_claimed_discriminator =
        solana_program::hash::hashv(&[b"event:RewardsClaimed"]).to_bytes()[..8].to_vec();
    let market_settled_discriminator =
        solana_program::hash::hashv(&[b"event:MarketSettled"]).to_bytes()[..8].to_vec();

    while let Some(msg) = log_stream.next().await {
        for log in msg.value.logs {
            println!("Log: {:?}", log);
            if let Some(stripped) = log.strip_prefix("Program data: ") {
                #[allow(deprecated)]
                if let Some(data) = base64::decode(stripped).ok() {
                    println!("{:?}", data);
                    if data.starts_with(&initialized_discriminator) {
                        let payload = &data[8..];
                        println!("MarketInitialized event payload: {:?}", payload);
                        // println!("Decoded MarketInitialized event: {:?}", event);
                        match MarketInitialized::try_from_slice(payload) {
                            Ok(event) => {
                                println!("Decoded MarketInitialized event: {:?}", event);
                                let market_id = event.market_id.to_string();
                                let market_pda = event.market_pda.to_string();
                                let metadata_url = event.metadata_url;
                                let yes_mint = event.yes_mint.to_string();
                                let no_mint = event.no_mint.to_string();
                                let usdc_vault = event.collateral_vault.to_string();
                                let status = MarketStatus::Open;
                                let outcome = market::MarketOutcome::NotDecided;
                                let close_time = chrono::DateTime::<Utc>::from_timestamp(
                                    event.expiration_timestamp as i64,
                                    0,
                                )
                                .unwrap();
                                let updated_at = chrono::Utc::now();
                                let market = create_market(
                                    &pool,
                                    &market_id,
                                    &market_pda,
                                    &metadata_url,
                                    &yes_mint,
                                    &no_mint,
                                    &usdc_vault,
                                    status,
                                    outcome,
                                    close_time,
                                    updated_at,
                                )
                                .await?;
                                println!("Inserted market into DB: {:?}", market);
                            }
                            Err(e) => {
                                println!("Failed to decode MarketInitialized event: {}", e);
                            }
                        }
                    } else if data.starts_with(&match_executed_discriminator) {
                        let payload = &data[8..];
                        println!("MatchExecuted event payload: {:?}", payload);
                        match crate::types::MatchExecuted::try_from_slice(payload) {
                            Ok(event) => {
                                println!("Decoded MatchExecuted event: {:?}", event);
                            }
                            Err(e) => {
                                println!("Failed to decode MatchExecuted event: {}", e);
                            }
                        }
                    } else if data.starts_with(&tokens_split_discriminator) {
                        let payload = &data[8..];
                        println!("TokensSplit event payload: {:?}", payload);
                        match crate::types::TokensSplit::try_from_slice(payload) {
                            Ok(event) => {
                                println!("Decoded TokensSplit event: {:?}", event);
                            }
                            Err(e) => {
                                println!("Failed to decode TokensSplit event: {}", e);
                            }
                        }
                    } else if data.starts_with(&tokens_merged_discriminator) {
                        let payload = &data[8..];
                        println!("TokensMerged event payload: {:?}", payload);
                        match crate::types::TokensMerged::try_from_slice(payload) {
                            Ok(event) => {
                                println!("Decoded TokensMerged event: {:?}", event);
                            }
                            Err(e) => {
                                println!("Failed to decode TokensMerged event: {}", e);
                            }
                        }
                    } else if data.starts_with(&rewards_claimed_discriminator) {
                        let payload = &data[8..];
                        println!("RewardsClaimed event payload: {:?}", payload);
                        match crate::types::RewardsClaimed::try_from_slice(payload) {
                            Ok(event) => {
                                println!("Decoded RewardsClaimed event: {:?}", event);
                            }
                            Err(e) => {
                                println!("Failed to decode RewardsClaimed event: {}", e);
                            }
                        }
                    } else if data.starts_with(&market_settled_discriminator) {
                        let payload = &data[8..];
                        println!("MarketSettled event payload: {:?}", payload);
                        match crate::types::MarketSettled::try_from_slice(payload) {
                            Ok(event) => {
                                dbg!("Processing MarketSettled event: {}", &event);
                                let resolve_time = chrono::Utc::now();
                                let market_id = event.market_id.to_string();
                                let outcome = match event.outcome {
                                    MarketOutcome::Yes => market::MarketOutcome::Yes,
                                    MarketOutcome::No => market::MarketOutcome::No,
                                    MarketOutcome::Undecided => market::MarketOutcome::NotDecided,
                                };
                                let market = update_market_resolution(
                                    &pool,
                                    market_id,
                                    MarketStatus::Resolved,
                                    outcome,
                                    resolve_time,
                                )
                                .await?;
                                println!("Decoded MarketSettled event: {:?}", event);
                                dbg!("market settled event processed: {}", market);
                            }
                            Err(e) => {
                                println!("Failed to decode MarketSettled event: {}", e);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
