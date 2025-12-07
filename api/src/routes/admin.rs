use std::sync::Arc;

use axum::{Router, middleware::from_fn, routing::{get, post}};

use crate::{
    auth::{auth::auth_middleware, require_admin::require_admin},
    handlers::{admin::{create_market, get_all_markets, resolve_market}},
    state::state::AppState,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/market/create", post(create_market))
        .route("/market/resolve", post(resolve_market))
        .route("/markets", get(get_all_markets))
        .route_layer(from_fn(require_admin))
        .route_layer(from_fn(auth_middleware))
}
