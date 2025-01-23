use chrono::{DateTime, Utc};
use serde::Serialize;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::application::service::transaction_service::TransactionError;

// TODO: support for structured (detailed) API errors
// TODO: existing boilerplate error handlers need to be refactored
//
// API error response samples:
//
// {
//   "errors": [
//     {
//         "status": 404,
//         "code": "user_not_found",
//         "kind": "resource_not_found",
//         "message": "The user does not exist",
//         "description": "Ghe user with the ID '12345' does not exist",
//         "detail": { "user_id": "12345" },
//         "reason": "resource must exist",
//         "instance": "/api/v1/users/12345",
//         "trace_id": "b8fe4d093d5bd6df",
//         "timestamp": "2024-01-19T16:58:34.123+0000",
//         "help": "Please check if the user ID is correct or refer to our documentation at https://api.example.com/docs/errors#user_not_found for more information",
//         "info_url": "https://api.example.com/docs/errors"
//     }
//   ]
// }
//
// {
//   "errors": [
//     {
//         "status": 422,
//         "code": "invalid_email",
//         "kind": "validation_error",
//         "message": "The user email is not valid",
//         "description": "Validation error in your request",
//         "detail": { "email": "xyz@12345" },
//         "reason": "must be a valid email address",
//         "instance": "/api/v1/users/12345",
//         "trace_id": "a97563baf79bb8fe",
//         "timestamp": "2024-01-19T16:58:35.225+0000",
//         "help": "Please check if the user email is correct or refer to our documentation at https://api.example.com/docs/errors#invalid_email for more information",
//         "info_url": "https://api.example.com/docs/errors"
//     },
//     {
//         "status": 422,
//         "code": "invalid_birthdate",
//         "kind": "validation_error",
//         "message": "The user birthdate is not correct",
//         "description": "Validation error in your request",
//         "detail": { "birthdate": "2050.02.30" },
//         "reason": "must be a valid calendar date in the past",
//         "instance": "/api/v1/users/12345",
//         "trace_id": "7563baf79b46c9a9",
//         "timestamp": "2024-01-19T16:59:03.124+0000",
//         "help": "Please check if the user birthdate is correct or refer to our documentation at https://api.example.com/docs/errors#invalid_birthdate for more information."
//         "info_url": "https://api.example.com/docs/errors"
//     },
//     {
//         "status": 422
//         "code": "invalid_role",
//         "kind": "validation_error",
//         "message": "The user birthdate is not correct",
//         "description": "Validation error in your request",
//         "detail": { role: "superadmin" },
//         "reason": "allowed roles: ['customer', 'guest']",
//         "instance": "/api/v1/users/12345",
//         "trace_id": "7563baf79b46c9a9",
//         "timestamp": "2024-01-19T16:59:03.124+0000",
//         "help": "Please check if the user role is correct or refer to our documentation at https://api.example.com/docs/errors#invalid_birthdate for more information",
//         "info_url": "https://api.example.com/docs/errors"
//     },
//   ]
// }
#[derive(Debug, Default, Serialize)]
pub struct DetailedError {
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<DetailedErrorKind>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info_url: Option<String>,
}

impl From<StatusCode> for DetailedError {
    fn from(status_code: StatusCode) -> Self {
        let status = status_code.into();
        let timestamp = Utc::now();
        Self {
            status,
            timestamp,
            ..Default::default()
        }
    }
}

impl From<ApiError> for DetailedError {
    fn from(api_error: ApiError) -> Self {
        let mut error = DetailedError::from(api_error.status_code);
        error.message = api_error.error_message;
        error
    }
}

impl IntoResponse for DetailedError {
    fn into_response(self) -> Response {
        tracing::error!("Error response: {:?}", self);
        let status_code =
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = serde_json::to_string(&self).unwrap_or_else(|_| "".to_string());
        (status_code, body).into_response()
    }
}

impl From<TransactionError> for DetailedError {
    fn from(transaction_error: TransactionError) -> Self {
        let message = format!("{}", transaction_error);
        let mut error = match transaction_error {
            TransactionError::InsufficientFunds => {
                let mut error = DetailedError::from(StatusCode::UNPROCESSABLE_ENTITY);
                error.code = Some("insufficient_funds".to_string());
                error.kind = Some(DetailedErrorKind::ValidationError);
                error.description = Some(
                    "there are insufficient funds in the source account for the transfer".into(),
                );
                error
            }
            TransactionError::SourceAccountNotFound(uuid) => {
                let mut error = DetailedError::from(StatusCode::UNPROCESSABLE_ENTITY);
                error.code = Some("source_account_not_found".to_string());
                error.kind = Some(DetailedErrorKind::ValidationError);
                error.detail = Some(format!("{{account_id: {}}}", uuid));
                error
            }
            TransactionError::DestinationAccountNotFound(uuid) => {
                let mut error = DetailedError::from(StatusCode::UNPROCESSABLE_ENTITY);
                error.code = Some("destination_account_not_found".to_string());
                error.kind = Some(DetailedErrorKind::ValidationError);
                error.detail = Some(format!("{{account_id: {}}}", uuid));
                error
            }
            TransactionError::SQLxError(e) => {
                let mut error = DetailedError::from(StatusCode::INTERNAL_SERVER_ERROR);
                error.code = serde_json::to_string(&DetailedErrorKind::DatabaseError).ok();
                error.kind = Some(DetailedErrorKind::DatabaseError);
                error.description =
                    Some(format!("Database error occured during transaction: {}", e));
                error
            }
        };
        error.message = message;
        error
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DetailedErrorKind {
    ResourceNotFound,
    ValidationError,
    DatabaseError,
}

pub struct ApiError {
    pub status_code: StatusCode,
    pub error_message: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{status_code: {}, error_message: {}}}",
            self.status_code, self.error_message
        )
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        tracing::error!("Error response: {}", self.to_string());
        (self.status_code, self.error_message).into_response()
    }
}

impl From<StatusCode> for ApiError {
    fn from(status_code: StatusCode) -> Self {
        Self {
            status_code,
            error_message: status_code.to_string(),
        }
    }
}
