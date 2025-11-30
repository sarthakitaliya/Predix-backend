use std::env;
use std::str::FromStr;

use anchor_client_sdk::PredixSdk;
use axum::{Extension, Json, extract::State, http::StatusCode};
use solana_sdk::pubkey::Pubkey;

use crate::{
    models::{
        admin::{CreateMarketRequest, CreateMarketResponse},
        auth::AuthUser,
    },
    state::state::Shared,
};

pub async fn create_market(
    State(state): State<Shared>,
    Extension(user): Extension<AuthUser>,
    Json(payload): Json<CreateMarketRequest>,
) -> Result<Json<CreateMarketResponse>, (StatusCode, String)> {
    let collateral_mint = Pubkey::from_str(&payload.collateral_mint).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid collateral mint address: {}", e),
        )
    })?;

    // SDK handles program creation internally
    state.predix_sdk
        .create_market(
            payload.market_id,
            collateral_mint,
            payload.metadata,
            payload.expiration_timestamp,
        )
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create market: {}", e),
            )
        })?;

    Ok(Json(CreateMarketResponse {
        market_id: payload.market_id,
        message: "Market created successfully".to_string(),
    }))
}