use std::str::FromStr;

use axum::{Extension, Json, extract::State, http::StatusCode};
use base64;
use solana_sdk::{
    message::Message, pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction,
};
use spl_token::instruction::approve_checked;
use std::env;

use crate::{
    models::{
        auth::AuthUser,
        delegate::{ApproveRequest, ApproveRes, CheckRequest, CheckResponse},
    },
    state::state::Shared,
    utils::solana::{derive_market_pda, verify_delegation},
};

pub async fn delegate_approval(
    State(state): State<Shared>,
    Extension(user): Extension<AuthUser>,
    Json(payload): Json<ApproveRequest>,
) -> Result<Json<ApproveRes>, (StatusCode, String)> {
    dbg!("Delegate approval payload: {:?}", &payload);
    let rpc_client = &state.rpc_client;
    let program_id = Pubkey::from_str(&payload.program_id).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid program ID: {}", e),
        )
    })?;
    let payer_pub_key = env::var("FEE_PAYER_PUBLIC_KEY").map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("FEE_PAYER_PUBLIC_KEY not set: {}", e),
        )
    })?;
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
    let mint_pubkey = Pubkey::from_str(&payload.mint).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid mint address: {}", e),
        )
    })?;
    let user_ata_pubkey = Pubkey::from_str(&payload.user_ata).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid user ATA address: {}", e),
        )
    })?;
    let (market_pda, bump) = derive_market_pda(payload.market_id);
    dbg!("Market PDA: {:?}", &market_pda);
    let approve_ix = approve_checked(
        &spl_token::id(),
        &user_ata_pubkey,
        &mint_pubkey,
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
    // we have to sign partially with fee payer and send transaction to frontend for user to sign
    let message = Message::new(&[approve_ix], Some((&fee_payer.pubkey())));
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

pub async fn check(
    State(state): State<Shared>,
    Extension(user): Extension<AuthUser>,
    Json(payload): Json<CheckRequest>,
) -> Result<Json<CheckResponse>, (StatusCode, String)> {
    dbg!("Check payload: {:?}", &payload);
    let (market_pda, bump) = derive_market_pda(payload.market_id);
    dbg!("Market PDA: {:?}", &market_pda);
    let a = verify_delegation(
        &state.rpc_client,
        &user.solana_address,
        &payload.collateral_mint,
    )
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Verification error: {}", e),
        )
    })?;
    Ok(Json(CheckResponse {
        message: "Check successful".to_string(),
    }))
}
