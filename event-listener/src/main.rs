use anchor_lang::prelude::*;
use std::str::FromStr;

use anyhow::Result;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
};
use solana_sdk::pubkey::Pubkey;

use tokio_stream::StreamExt;

use crate::types::MarketInitialized;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let program_id = std::env::var("PROGRAM_ID")?;
    let rpc_url =
        std::env::var("SOLANA_WS_RPC_URL").unwrap_or("wss://api.devnet.solana.com/".to_string());

    let program_id =
        Pubkey::from_str(&program_id).map_err(|e| anyhow::anyhow!("Invalid program id: {}", e))?;

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
                            }
                            Err(e) => {
                                println!("Failed to decode MarketInitialized event: {}", e);
                            }
                        }
                    }else if data.starts_with(&match_executed_discriminator) {
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
                                println!("Decoded MarketSettled event: {:?}", event);
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
