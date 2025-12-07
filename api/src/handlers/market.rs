use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use db::queries::market::{self, list_markets_by_status};

use crate::{
    models::market::{MarketByIdResponse, MarketsByStatusQuery, MarketsByStatusResponse},
    state::state::Shared,
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
