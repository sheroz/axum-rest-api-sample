use axum::http::StatusCode;

use crate::application::api_error::{ApiError, DetailedError};

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl From<AuthError> for ApiError {
    fn from(err: AuthError) -> Self {
        let (status_code, error_message) = match err {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };

        Self {
            status_code,
            error_message: error_message.to_owned(),
        }
    }
}

impl From<AuthError> for DetailedError {
    fn from(auth_errors: AuthError) -> Self {
        let api_error = ApiError::from(auth_errors);
        Self::from(api_error)
    }
}
