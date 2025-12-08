use std::str::FromStr;

use axum::{Extension, Json, extract::State, http::StatusCode};
use db::{models::market::MarketOutcome};
use solana_sdk::pubkey::Pubkey;
use uuid::Uuid;

use crate::{
    models::{
        admin::{
            CreateMarketRequest, CreateMarketResponse, GetAllMarketsResponse, ResolveMarketRequest,
            ResolveMarketResponse,
        },
        auth::AuthUser,
    },
    state::state::Shared,
    utils::s3::upload_market_metadata_to_do,
};

pub async fn create_market(
    State(state): State<Shared>,
    Extension(_user): Extension<AuthUser>,
    Json(payload): Json<CreateMarketRequest>,
) -> Result<Json<CreateMarketResponse>, (StatusCode, String)> {
    dbg!("payload:", &payload);
    let bucket = std::env::var("DO_SPACES_BUCKET").map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("DO_SPACES_BUCKET environment variable not set: {}", e),
        )
    })?;
    dbg!("Using bucket:", &bucket);
    let collateral_mint = Pubkey::from_str(&payload.collateral_mint).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid collateral mint address: {}", e),
        )
    })?;
    let uuid = Uuid::new_v4();
    let bytes = uuid.as_bytes();
    let market_id = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
    dbg!("Generated market ID:", market_id);
    let metadata_url =
        upload_market_metadata_to_do(&state.s3, &bucket, market_id, &payload.metadata)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to upload market metadata: {}", e),
                )
            })?;
    dbg!("Metadata URL:", &metadata_url);
    state
        .predix_sdk
        .create_market(
            market_id,
            collateral_mint,
            metadata_url,
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
        market_id: market_id,
        message: "Market created successfully".to_string(),
    }))
}

pub async fn resolve_market(
    State(state): State<Shared>,
    Extension(_user): Extension<AuthUser>,
    Json(payload): Json<ResolveMarketRequest>,
) -> Result<Json<ResolveMarketResponse>, (StatusCode, String)> {
    dbg!("Resolving market with payload:", &payload);

    let market_id = payload
        .market_id
        .parse::<u64>()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid market id: {}", e)))?;
    let outcome = match payload.outcome {
        MarketOutcome::Yes => anchor_client_sdk::predix_program::types::MarketOutcome::Yes,
        MarketOutcome::No => anchor_client_sdk::predix_program::types::MarketOutcome::No,
        MarketOutcome::NotDecided => {
            anchor_client_sdk::predix_program::types::MarketOutcome::Undecided
        }
    };
    let tx = state
        .predix_sdk
        .set_winner(market_id, outcome)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create set winner instruction: {}", e),
            )
        })?;
    Ok(Json(ResolveMarketResponse {
        tx_message: tx,
        message: "Market resolved successfully".into(),
    }))
}

pub async fn get_all_markets(
    State(state): State<Shared>,
    Extension(_user): Extension<AuthUser>,
) -> Result<Json<GetAllMarketsResponse>, (StatusCode, String)> {
    let markets = db::queries::market::list_all_markets(&state.db_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch markets: {}", e),
            )
        })?;
    Ok(Json(GetAllMarketsResponse { markets }))
}
