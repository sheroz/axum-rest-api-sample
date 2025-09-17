#[derive(Clone, Debug)]
pub struct PostgresOptions {
    /// Database name.
    pub db: String,

    /// Database host.
    pub host: String,

    /// Database port.
    pub port: u16,

    /// Database user.
    pub user: String,

    /// Database password.
    pub password: String,

    /// Max connections (connection pool).
    pub max_connections: u32,
}

impl PostgresOptions {
    pub fn connection_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db
        )
    }

    pub fn set_db(&mut self, postgres_db: &str) {
        self.db = postgres_db.to_owned()
    }

    pub fn db(&self) -> String {
        self.db.clone()
    }

    pub const fn set_max_connections(&mut self, max_connections: u32) {
        self.max_connections = max_connections
    }

    pub const fn max_connections(&self) -> u32 {
        self.max_connections
    }
}
