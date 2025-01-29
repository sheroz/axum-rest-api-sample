use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{application::config::Config, infrastructure::types::DatabasePool};

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub config: Config,
    pub db_pool: DatabasePool,
    pub redis: Mutex<redis::aio::MultiplexedConnection>,
}
