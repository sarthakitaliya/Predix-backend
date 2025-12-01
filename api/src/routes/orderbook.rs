use std::sync::Arc;

use axum::{Router, middleware::from_fn, routing::get};

use crate::{
    auth::auth::auth_middleware,
    handlers::{
        delegate::{check, delegate_approval},
        orderbook::{cancel_order, get_orderbook, merge_order, place_order, split_order},
    },
    state::state::AppState,
};
use axum::routing::{delete, post};

pub fn router() -> Router<Arc<AppState>> {
    let public = Router::new()
        .route("/snapshot/{market_id}", get(get_orderbook));
    
        let protected = Router::new()
        .route("/place-order", post(place_order))
        .route("/split-order", post(split_order))
        .route("/merge-order", post(merge_order))
        .route("/cancel-order", delete(cancel_order))
        .route("/approve", post(delegate_approval))
        .route("/check", post(check))
        .route_layer(from_fn(auth_middleware));

    public.merge(protected)
}
