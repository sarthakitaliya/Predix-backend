use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{handlers::market::{get_all_markets_by_status, get_market_by_id}, state::state::AppState};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_markets_by_status))
        .route("/{id}", get(get_market_by_id))
}
