use axum::http::StatusCode;
use thiserror::Error;

use crate::application::api_error::{ApiError, ApiErrorCode, ApiErrorKind, ErrorDetail};

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
}

impl From<AuthError> for ApiError {
    fn from(auth_error: AuthError) -> Self {
        let status = match auth_error {
            AuthError::WrongCredentials => StatusCode::UNAUTHORIZED,
            AuthError::MissingCredentials => StatusCode::BAD_REQUEST,
            AuthError::TokenCreationError => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::InvalidToken => StatusCode::BAD_REQUEST,
        };
        let error = ErrorDetail::new(&auth_error.to_string())
            .code(ApiErrorCode::AuthenticationError)
            .kind(ApiErrorKind::AuthenticationError);

        Self {
            status,
            errors: vec![error],
        }
    }
}
