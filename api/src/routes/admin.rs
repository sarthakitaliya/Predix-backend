use std::sync::Arc;

use axum::{Router, middleware::from_fn, routing::{get, post}};

use crate::{
    auth::{auth::auth_middleware, require_admin::require_admin},
    handlers::{admin::create_market, market::resolve_market},
    state::state::AppState,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/market/create", post(create_market))
        .route("/market/resolve", get(resolve_market))
        .route_layer(from_fn(require_admin))
        .route_layer(from_fn(auth_middleware))
}
