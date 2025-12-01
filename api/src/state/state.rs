use std::{collections::HashMap, sync::Arc};

use anchor_client_sdk::PredixSdk;
use privy_rs::PrivyClient;
use solana_client::nonblocking::rpc_client::RpcClient;
use tokio::sync::{RwLock, mpsc};

use crate::engine::engine::EngineMsg;

pub struct AppState {
    pub markets: RwLock<HashMap<u64, mpsc::Sender<EngineMsg>>>,
    pub privy_client: Arc<PrivyClient>,
    pub rpc_client: Arc<RpcClient>,
    pub predix_sdk: Arc<PredixSdk>,
}

pub type Shared = Arc<AppState>;
