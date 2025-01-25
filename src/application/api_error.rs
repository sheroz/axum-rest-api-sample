use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// TODO: support for structured (detailed) API errors
// TODO: existing boilerplate error handlers need to be refactored
//
// API error response samples:
//
// {
//   "errors": [
//     {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    #[serde(skip)]
    pub status: StatusCode,
    pub errors: Vec<ErrorDetail>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorCode {
    AuthenticationError,
    TransactionNotFound,
    InsufficientFunds,
    SourceAccountNotFound,
    DestinationAccountNotFound,
    DatabaseError,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorKind {
    AuthenticationError,
    ResourceNotFound,
    ValidationError,
    DatabaseError,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<serde_json::Value>,
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

impl ErrorDetail {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            timestamp: Utc::now(),
            ..Default::default()
        }
    }
    pub fn code(mut self, code: ApiErrorCode) -> Self {
        self.code = serde_json::to_string(&code).ok();
        self
    }

    pub fn kind(mut self, kind: ApiErrorKind) -> Self {
        self.kind = serde_json::to_string(&kind).ok();
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn detail(mut self, detail: serde_json::Value) -> Self {
        self.detail = Some(detail);
        self
    }
}

impl From<StatusCode> for ApiError {
    fn from(status: StatusCode) -> Self {
        Self {
            status,
            errors: vec![ErrorDetail::new(&status.to_string())],
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        tracing::error!("Error response: {:?}", self);
        (self.status, Json(self)).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        let status = match error {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self {
            status,
            errors: vec![ErrorDetail::from(error)],
        }
    }
}

impl From<sqlx::Error> for ErrorDetail {
    fn from(e: sqlx::Error) -> Self {
        Self::new(&e.to_string())
            .code(ApiErrorCode::DatabaseError)
            .kind(ApiErrorKind::DatabaseError)
            .description(format!("Database error occured: {}", e))
    }
}
