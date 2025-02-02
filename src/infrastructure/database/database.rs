use sqlx::{PgConnection, PgPool};
use thiserror::Error;

use crate::infrastructure::database::postgres::{PostgresDatabase, PostgresOptions};

pub type DatabasePool = PgPool;
pub type DatabaseConnection = PgConnection;
pub type TestDatabase = PostgresDatabase;

#[derive(Clone, Debug)]
pub struct DatabaseOptions {
    pub postgres: PostgresOptions,
}

pub struct Database;

impl Database {
    pub async fn connect(options: DatabaseOptions) -> Result<DatabasePool, DatabaseError> {
        let db = PostgresDatabase::connect(options).await?;
        Ok(db.pool().clone())
    }

    pub async fn open_test_database(
        options: DatabaseOptions,
    ) -> Result<TestDatabase, DatabaseError> {
        // Create a test database.
        let db = PostgresDatabase::connect_test(options).await?;

        // Run database migrations.
        Self::migrate(db.pool()).await?;

        Ok(db)
    }

    pub async fn migrate(pool: &DatabasePool) -> Result<(), DatabaseError> {
        sqlx::migrate!("src/infrastructure/database/postgres/migrations")
            .run(pool)
            .await?;

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error(transparent)]
    SQLxError(#[from] sqlx::Error),
    #[error(transparent)]
    SQLxMigrateError(#[from] sqlx::migrate::MigrateError),
}
