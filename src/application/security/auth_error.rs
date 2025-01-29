use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("wrong credentials")]
    WrongCredentials,
    #[error("missing credentials")]
    MissingCredentials,
    #[error("token creation error")]
    TokenCreationError,
    #[error("invalid token")]
    InvalidToken,
    #[error("forbidden")]
    Forbidden,
}
