use axum::{extract::State, http::StatusCode};

use crate::state::state::Shared;


//TODO: Implement all markets related handlers

pub async fn get_all_markets(
    State(state): State<Shared>,
) -> Result<String, (StatusCode, String)> {
    Ok("List of all markets".to_string())
}

pub async fn resolve_market(
    State(state): State<Shared>,
) -> Result<String, (StatusCode, String)> {
    Ok("Market resolved".to_string())
}