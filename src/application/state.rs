use std::sync::Arc;
use tokio::sync::Mutex;

use crate::infrastructure::types::DatabasePool;

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub db_pool: DatabasePool,
    pub redis: Mutex<redis::aio::MultiplexedConnection>,
}
