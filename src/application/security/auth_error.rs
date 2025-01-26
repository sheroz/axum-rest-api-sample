use axum::http::StatusCode;
use thiserror::Error;

use crate::api::api_error::{ApiError, ApiErrorCode, ApiErrorEntry, ApiErrorKind};

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

impl From<AuthError> for ApiError {
    fn from(auth_error: AuthError) -> Self {
        let (status, code) = match auth_error {
            AuthError::WrongCredentials => {
                (StatusCode::UNAUTHORIZED, ApiErrorCode::AuthWrongCredentials)
            }
            AuthError::MissingCredentials => (
                StatusCode::BAD_REQUEST,
                ApiErrorCode::AuthMissingCredentials,
            ),
            AuthError::TokenCreationError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorCode::AuthTokenCreationError,
            ),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, ApiErrorCode::AuthInvalidToken),
            AuthError::Forbidden => (StatusCode::FORBIDDEN, ApiErrorCode::AuthForbidden),
        };

        let error = ApiErrorEntry::new(&auth_error.to_string())
            .code(code)
            .kind(ApiErrorKind::AuthenticationError);

        Self {
            status,
            errors: vec![error],
        }
    }
}
