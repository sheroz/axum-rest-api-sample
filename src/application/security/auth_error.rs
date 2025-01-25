use axum::http::StatusCode;

use crate::application::api_error::{ApiError, ApiErrorSimple};

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl From<AuthError> for ApiErrorSimple {
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

impl From<AuthError> for ApiError {
    fn from(auth_errors: AuthError) -> Self {
        ApiErrorSimple::from(auth_errors).into()
    }
}
