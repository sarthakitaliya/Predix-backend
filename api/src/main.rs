use std::{collections::HashMap, sync::Arc};

use axum::{
    Router,
    routing::{get, post},
};
use tokio::sync::RwLock;

use crate::{handlers::{orderbook::{place_order, cancel_order}, snapshot::snapshot}, state::AppState};

mod engine;
mod handlers;
mod models;
mod state;

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        markets: RwLock::new(HashMap::new()),
    });

    let app = Router::new()
        .route("/", get(|| async { "hello" }))
        .route("/orderbook", post(place_order).delete(cancel_order))
        .route("/snapshot/{market_id}", get(snapshot))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
