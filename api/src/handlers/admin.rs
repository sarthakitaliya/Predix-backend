use axum::{extract::State, http::StatusCode};

use crate::state::state::Shared;


//TODO: Implement all admin related handlers

pub async fn create_market(
    State(state): State<Shared>,
) -> Result<String, (StatusCode, String)> {
    Ok("Market created".to_string())
}