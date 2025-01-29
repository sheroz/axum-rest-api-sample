pub mod account_repo;
pub mod transaction_repo;
pub mod user_repo;

pub type RepositoryResult<T> = Result<T, sqlx::Error>;
