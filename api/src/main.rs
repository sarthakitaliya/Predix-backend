use dotenvy::dotenv;
use privy_rs::PrivyClient;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::CommitmentConfig};
use std::{collections::HashMap, env, sync::Arc};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

use axum::{
    Json, Router,
    http::{
        HeaderName, HeaderValue, Method,
        header::{ACCEPT, ACCESS_CONTROL_ALLOW_CREDENTIALS, AUTHORIZATION, CONTENT_TYPE},
    },
    middleware::{self, from_fn},
    routing::{get, post},
};
use tokio::sync::RwLock;

use crate::{
    auth::{middleware::auth_middleware, privy::PClient},
    handlers::{
        delegate::delegate_approval, orderbook::{cancel_order, health_check, place_order}, snapshot::snapshot
    },
    state::AppState,
};

mod auth;
mod engine;
mod handlers;
mod models;
mod state;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("PRIVY_APP_ID").expect("PRIVY_APP_ID environment variable not set");
    let app_secret =
        env::var("PRIVY_APP_SECRET").expect("PRIVY_APP_SECRET environment variable not set");

    let client = PrivyClient::new(app_id, app_secret)?;
    let rpc_url = env::var("SOLANA_RPC_URL").unwrap_or("https://api.devnet.solana.com".to_string());
    let rpc = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let state = Arc::new(AppState {
        markets: RwLock::new(HashMap::new()),
        privy_client: Arc::new(client),
        rpc_client: Arc::new(rpc),
    });

    // let res = client.sign_message("Hello, Privy!").await?;
    // println!("Signed message response: {:?}", res);
    let privy_header = HeaderName::from_static("privy-id-token");
    //TODO: update cors policy
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>()?)
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE, privy_header])
        .allow_credentials(true);

    let health_route = Router::new()
        .route("/", get(health_check))
        .route("/approve", post(delegate_approval))
        .route_layer(from_fn(auth_middleware));
    let app = Router::new()
        // .route("/", get(health_check))
        .route("/orderbook", post(place_order).delete(cancel_order))
        .route("/snapshot/{market_id}", get(snapshot))
        .merge(health_route)
        .with_state(state)
        .layer(cors);
         
    // .layer(middleware::from_fn(auth_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
