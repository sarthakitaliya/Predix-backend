use std::sync::Arc;

use axum::{Router, middleware::from_fn, routing::get};

use crate::{
    auth::auth::auth_middleware,
    handlers::{
        orderbook::{get_orderbook, get_open_orders},
    },
    state::state::AppState,
};
use axum::routing::{delete, post};

pub fn router() -> Router<Arc<AppState>> {
    let public = Router::new()
        .route("/snapshot/{market_id}", get(get_orderbook));
    
        let protected = Router::new()
        //TODO: add handler for open orders
        .route("/open/{market_id}", get(get_open_orders))
        .route_layer(from_fn(auth_middleware));

    public.merge(protected)
}
