use axum::http::StatusCode;

use crate::application::api_error::{ApiError, ErrorDetail};

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl From<AuthError> for ApiError {
    fn from(err: AuthError) -> Self {
        let (status, message) = match err {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };

        Self {
            status,
            errors: vec![ErrorDetail::new(&message)],
        }
    }
}
