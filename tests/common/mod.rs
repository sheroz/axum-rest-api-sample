pub mod accounts;
pub mod auth;
pub mod constants;
pub mod fetch;
pub mod helpers;
pub mod route;
pub mod test_app;
pub mod transactions;
pub mod users;

type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
