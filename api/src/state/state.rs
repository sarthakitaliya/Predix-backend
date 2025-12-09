use std::{collections::HashMap, sync::Arc};

use anchor_client_sdk::PredixSdk;
use aws_sdk_s3::Client;

use solana_client::nonblocking::rpc_client::RpcClient;
use tokio::sync::{RwLock, mpsc};


use crate::engine::engine::EngineMsg;

pub struct AppState {
    pub markets: RwLock<HashMap<u64, mpsc::Sender<EngineMsg>>>,
    pub rpc_client: Arc<RpcClient>,
    pub predix_sdk: Arc<PredixSdk>,
    pub s3: Arc<Client>,
    pub db_pool: Arc<sqlx::PgPool>,
}

pub type Shared = Arc<AppState>;
