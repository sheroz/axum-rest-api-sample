use sqlx::postgres::PgPoolOptions;

use crate::{application::config::Config, infrastructure::types::DatabasePool};

pub async fn pgpool(config: &Config) -> DatabasePool {
    match PgPoolOptions::new()
        .max_connections(config.postgres_connection_pool)
        .connect(&config.postgres_url())
        .await
    {
        Ok(pool) => {
            tracing::info!("Connected to postgres");
            pool
        }
        Err(e) => {
            tracing::error!("Could not connect to postgres: {}", e);
            std::process::exit(1);
        }
    }
}
