use std::{collections::HashMap, sync::Arc};

use tokio::sync::{RwLock, mpsc};

use crate::engine::engine::EngineMsg;

pub struct AppState {
    pub markets: RwLock<HashMap<String, mpsc::Sender<EngineMsg>>>,
}

pub type Shared = Arc<AppState>;

