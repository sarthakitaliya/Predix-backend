use std::sync::Arc;

use axum::{
    Router,
    extract::State,
    http::{
        HeaderName, HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
    routing::get,
};
use tower_http::cors::CorsLayer;

use crate::{
    routes,
    state::state::{AppState, Shared},
};

pub async fn health_check(State(_state): State<Shared>) -> &'static str {
    dbg!("Health check called");
    "OK"
}

pub fn build_app(state: Arc<AppState>) -> Router {
    let privy_header = HeaderName::from_static("privy-id-token");
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE, privy_header])
        .allow_credentials(true);

    let app = Router::new()
        .route("/health-check", get(health_check))
        .nest("/admin", routes::admin::router())
        .nest("/markets", routes::markets::router())
        .nest("/orderbook", routes::orderbook::router())
        .layer(cors)
        .with_state(state);

    app
}
