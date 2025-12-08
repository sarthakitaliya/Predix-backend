use std::sync::Arc;

use axum::{Router, middleware::from_fn};

use crate::{ auth::auth::auth_middleware, handlers::orders::{cancel_order, merge_order, place_order, split_order}, state::state::AppState};

use axum::routing::{delete, post};



pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/place", post(place_order))
        .route("/split", post(split_order))
        .route("/merge", post(merge_order))
        .route("/cancel/{order_id}", delete(cancel_order))
        .route_layer(from_fn(auth_middleware))
}