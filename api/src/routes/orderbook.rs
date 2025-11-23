use std::sync::Arc;

use axum::{Router, middleware::from_fn, routing::get};

use crate::{
    auth::{
        auth::auth_middleware,
    },
    handlers::orderbook::{cancel_order, place_order, snapshot},
    state::state::AppState,
};
use axum::routing::{delete, post};

pub fn router() -> Router<Arc<AppState>> {
    let public = Router::new()
        .route("/snapshot/{market_id}", get(snapshot));

    let protected = Router::new()
        .route("/place", post(place_order))
        .route("/cancel", delete(cancel_order))
        .route_layer(from_fn(auth_middleware));

    public.merge(protected)
}
