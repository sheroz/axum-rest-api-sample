use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    api::server,
    application::{config, state::AppState},
    infrastructure::{postgres, redis},
};

pub async fn run() {
    // Load configuration.
    let config = config::load();

    // Connect to Redis.
    let redis = redis::open(&config).await;

    // Connect to PostgreSQL.
    let db_pool = postgres::pgpool(&config).await;

    // Run migrations.
    sqlx::migrate!("src/infrastructure/postgres/migrations")
        .run(&db_pool)
        .await
        .unwrap();

    // Build the application state.
    let shared_state = Arc::new(AppState {
        config,
        db_pool,
        redis: Mutex::new(redis),
    });

    server::start(shared_state).await;
}
