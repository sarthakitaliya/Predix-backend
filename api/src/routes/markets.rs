use std::sync::Arc;

use axum::{
    Router, middleware::from_fn, routing::{get, post}
};

use crate::{
    auth::auth::auth_middleware, handlers::market::{delegate_approval, get_all_markets_by_status, get_market_by_id}, state::state::AppState
};

pub fn router() -> Router<Arc<AppState>> {
    let public = Router::new()
        .route("/", get(get_all_markets_by_status))
        .route("/{id}", get(get_market_by_id));
    
    let protected = Router::new()
        .route("/delegate", post(delegate_approval))
        .route_layer(from_fn(auth_middleware));

    public.merge(protected)
}
