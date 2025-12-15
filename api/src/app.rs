use std::sync::Arc;

use axum::{
    Router,
    extract::State,
    http::{
        HeaderName, HeaderValue, Method,
        header::{ACCEPT, ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE},
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
    //take from env variable
    let origin = std::env::var("CORS_ALLOW_ORIGIN")
    .map_err(|_| "CORS_ALLOW_ORIGIN not set")
    .unwrap();
    let allow_origin = HeaderValue::from_str(&origin).unwrap();
    let cors = CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE, privy_header, ACCESS_CONTROL_ALLOW_ORIGIN])
        .allow_credentials(true);

    let app = Router::new()
        .route("/health-check", get(health_check))
        .nest("/admin", routes::admin::router())
        .nest("/markets", routes::markets::router())
        .nest("/orders", routes::orders::router())
        .nest("/orderbook", routes::orderbook::router())
        .layer(cors)
        .with_state(state);

    app
}
