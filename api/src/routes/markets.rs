use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{handlers::{market::get_all_markets}, state::state::AppState};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/market", get(get_all_markets))
}
