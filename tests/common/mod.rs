pub mod auth;
pub mod constants;
pub mod fetch;
pub mod route;
pub mod users;
pub mod utils;

type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
