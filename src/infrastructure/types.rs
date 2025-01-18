use sqlx::{PgConnection, Pool, Postgres};

pub type DatabasePool = Pool<Postgres>;
pub type DatabaseConnection = PgConnection;
