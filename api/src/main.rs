use anchor_client::{
    Client, Cluster,
    solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair},
};
use anchor_client_sdk::PredixSdk;
use anchor_lang::declare_program;
use dotenvy::dotenv;
use privy_rs::PrivyClient;
use solana_client::nonblocking::rpc_client::RpcClient;
use std::{collections::HashMap, env, rc::Rc, sync::Arc};
use tower_http::cors::CorsLayer;

use axum::{
    Router,
    http::{
        HeaderName, HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
    middleware::from_fn,
    routing::{get, post},
};
use tokio::sync::RwLock;

use crate::{
    auth::{auth::auth_middleware, require_admin::require_admin},
    handlers::delegate::delegate_approval,
    state::state::AppState,
};
// use anchor_lang::prelude::*;

mod app;
mod auth;
mod engine;
mod handlers;
mod models;
mod routes;
mod state;
mod utils;

declare_program!(predix_program);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let app_id = env::var("PRIVY_APP_ID").expect("PRIVY_APP_ID environment variable not set");
    let app_secret =
        env::var("PRIVY_APP_SECRET").expect("PRIVY_APP_SECRET environment variable not set");

    let client = PrivyClient::new(app_id, app_secret)?;
    let rpc_url = env::var("SOLANA_RPC_URL")?;
    let rpc = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let payer_private_key =
        env::var("FEE_PAYER_PRIVATE_KEY").expect("FEE_PAYER_PRIVATE_KEY must be set");
    let predix_sdk = PredixSdk::new(&payer_private_key)?;
    let state = Arc::new(AppState {
        markets: RwLock::new(HashMap::new()),
        privy_client: Arc::new(client),
        rpc_client: Arc::new(rpc),
        predix_sdk: Arc::new(predix_sdk),
    });

    let app = app::build_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
