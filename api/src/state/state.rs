use std::{collections::HashMap, sync::Arc};

use privy_rs::PrivyClient;
use solana_client::nonblocking::rpc_client::RpcClient;
use tokio::sync::{RwLock, mpsc};

use crate::engine::engine::EngineMsg;

pub struct AppState {
    pub markets: RwLock<HashMap<String, mpsc::Sender<EngineMsg>>>,
    pub privy_client: Arc<PrivyClient>,
    pub rpc_client: Arc<RpcClient>,
}

pub type Shared = Arc<AppState>;
