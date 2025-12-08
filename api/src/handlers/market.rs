use anchor_client_sdk::{derive_yes_and_no_mint_pdas, predix_program};
use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use db::{models::close_order::ShareType, queries::market::{self, list_markets_by_status}};
use matching::types::Side;
use solana_sdk::{
    instruction::Instruction, message::Message, pubkey::Pubkey, signature::Keypair, signer::Signer,
    transaction::Transaction,
};
use spl_associated_token_account::{get_associated_token_address, instruction::create_associated_token_account_idempotent};
use spl_token::instruction::approve_checked;
use std::{env, str::FromStr};

use crate::{
    models::{
        auth::AuthUser,
        market::{ApproveRequest, ApproveRes, MarketByIdResponse, MarketsByStatusQuery, MarketsByStatusResponse},

    },
    state::state::Shared,
    utils::solana::derive_market_pda,
};

pub async fn get_all_markets_by_status(
    State(state): State<Shared>,
    Query(query): Query<MarketsByStatusQuery>,
) -> Result<Json<MarketsByStatusResponse>, (StatusCode, String)> {
    let markets = list_markets_by_status(&state.db_pool, query.status)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch markets by status: {}", e),
            )
        })?;
    Ok(Json(MarketsByStatusResponse { markets }))
}

pub async fn get_market_by_id(
    State(state): State<Shared>,
    Path(market_id): Path<String>,
) -> Result<Json<MarketByIdResponse>, (StatusCode, String)> {
    let market = market::get_market_by_id(&state.db_pool, market_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch market by id: {}", e),
            )
        })?;
    Ok(Json(MarketByIdResponse { market }))
}

pub async fn delegate_approval(
    State(state): State<Shared>,
    Extension(user): Extension<AuthUser>,
    Json(payload): Json<ApproveRequest>,
) -> Result<Json<ApproveRes>, (StatusCode, String)> {
    dbg!("Delegate approval payload: {:?}", &payload);
    let rpc_client = &state.rpc_client;
    let payer_private_key = env::var("FEE_PAYER_PRIVATE_KEY").map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("FEE_PAYER_PRIVATE_KEY not set: {}", e),
        )
    })?;

    let fee_payer = Keypair::from_base58_string(&payer_private_key);
    let recent_blockhash = rpc_client.get_latest_blockhash().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("RPC error: {}", e),
        )
    })?;
    dbg!("user wallet: {:?}", &user.solana_address);
    let wallet_pubkey = Pubkey::from_str(&user.solana_address).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid wallet address: {}", e),
        )
    })?;
    let usdc_mint_pubkey = Pubkey::from_str("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU")
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Invalid USDC mint address: {}", e),
            )
        })?;
    let market_id_str = payload.market_id.clone();
    let market_id = market_id_str
        .parse::<u64>()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid market id: {}", e)))?;
    let (market_pda, _bump) = derive_market_pda(market_id);
    dbg!("Market PDA: {:?}", &market_pda);
    let mut ixs: Vec<Instruction> = Vec::new();
    let token_mint;
    let (yes_mint_pda, no_mint_pda) = derive_yes_and_no_mint_pdas(market_id, &predix_program::ID);
    if payload.side == Side::Bid {
        match payload.share {
            ShareType::Yes => {
                token_mint = yes_mint_pda.0;
            }
            ShareType::No => {
                token_mint = no_mint_pda.0;
            }
        }
        let ix = create_associated_token_account_idempotent(
            &fee_payer.pubkey(),
            &wallet_pubkey,
            &token_mint,
            &spl_token::id(),
        );
        dbg!("Create ATA instruction: {:?}", &ix);
        ixs.push(ix);
        // delegate approval
        let user_ata_pubkey = spl_associated_token_account::get_associated_token_address(
            &wallet_pubkey,
            &usdc_mint_pubkey,
        );
        let approve_ix = approve_checked(
            &spl_token::id(),
            &user_ata_pubkey,
            &usdc_mint_pubkey,
            &market_pda,
            &wallet_pubkey,
            &[],
            payload.amount,
            payload.decimals,
        )
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create approve instruction: {}", e),
            )
        })?;
        dbg!("Approve instruction: {:?}", &approve_ix);
        ixs.push(approve_ix);
    } else if payload.side == Side::Ask {
        match payload.share {
            ShareType::Yes => {
                token_mint = yes_mint_pda.0;
            }
            ShareType::No => {
                token_mint = no_mint_pda.0;
            }
        }
        let token_ata = get_associated_token_address(&wallet_pubkey, &token_mint);
        let approve_ix = approve_checked(
            &spl_token::id(),
            &token_ata,
            &token_mint,
            &market_pda,
            &wallet_pubkey,
            &[],
            payload.amount,
            payload.decimals,
        )
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create approve instruction: {}", e),
            )
        })?;
        dbg!("Approve instruction: {:?}", &approve_ix);
        ixs.push(approve_ix);
    }

    // we have to sign partially with fee payer and send transaction to frontend for user to sign
    let message = Message::new(&ixs, Some(&fee_payer.pubkey()));
    let mut tx = Transaction::new_unsigned(message);
    dbg!("Partial transaction before signing: {:?}", &tx);
    tx.try_partial_sign(&[fee_payer], recent_blockhash)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to partially sign transaction: {}", e),
            )
        })?;
    let serialized = bincode::serialize(&tx).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to serialize transaction: {}", e),
        )
    })?;
    #[allow(deprecated)]
    let tx_base64 = base64::encode(&serialized);
    // let tx_base64 = base64::encode(&serialized);
    Ok(Json(ApproveRes {
        tx_message: tx_base64,
        recent_blockhash: recent_blockhash.to_string(),
    }))
}
