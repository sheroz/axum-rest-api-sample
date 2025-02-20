use std::fmt::{Display, Formatter, Result};

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub const API_DOCUMENT_URL: &str =
    "https://github.com/sheroz/axum-rest-api-sample/blob/main/docs/api-docs.md";

// API error response samples:
//
// {
//   "status": 404,
//   "errors": [
//     {
//         "code": "user_not_found",
//         "kind": "resource_not_found",
//         "message": "user not found: 12345",
//         "description": "user with the ID '12345' does not exist in our records",
//         "detail": { "user_id": "12345" },
//         "reason": "must be an existing user",
//         "instance": "/api/v1/users/12345",
//         "trace_id": "3d2b4f2d00694354a00522fe3bb86158",
//         "timestamp": "2024-01-19T16:58:34.123+0000",
//         "help": "please check if the user ID is correct or refer to our documentation at https://github.com/sheroz/axum-rest-api-sample/blob/main/docs/api-docs.md#errors for more information",
//         "doc_url": "https://github.com/sheroz/axum-rest-api-sample/blob/main/docs/api-docs.md"
//     }
//   ]
// }
//
// ---
//
// {
//   "status": 422,
//   "errors": [
//     {
//         "code": "transfer_insufficient_funds",
//         "kind": "validation_error",
//         "message": "source account does not have sufficient funds for the transfer",
//         "reason": "source account balance must be sufficient to cover the transfer amount",
//         "instance": "/api/v1/transactions/transfer",
//         "trace_id": "fbb9fdf5394d4abe8e42b49c3246310b",
//         "timestamp": "2024-01-19T16:58:35.225+0000",
//         "help": "please check the source account balance or refer to our documentation at https://github.com/sheroz/axum-rest-api-sample/blob/main/docs/api-docs.md#errors for more information",
//         "doc_url": "https://github.com/sheroz/axum-rest-api-sample/blob/main/docs/api-docs.md"
//     },
//     {
//         "code": "transfer_destination_account_not_found",
//         "kind": "validation_error",
//         "message": "destination account not found: d424cfe9-c042-41db-9a8e-8da5715fea10",
//         "detail": { "destination_account_id": "d424cfe9-c042-41db-9a8e-8da5715fea10" },
//         "reason": "must be an existing account",
//         "instance": "/api/v1/transactions/transfer",
//         "trace_id": "8a250eaa650943b085934771fb35ba54",
//         "timestamp": "2024-01-19T16:59:03.124+0000",
//         "help": "please check if the destination account ID is correct or refer to our documentation at https://github.com/sheroz/axum-rest-api-sample/blob/main/docs/api-docs.md#errors for more information.",
//         "doc_url": "https://github.com/sheroz/axum-rest-api-sample/blob/main/docs/api-docs.md"
//     },
//   ]
// }
//
// ---
//
// (Users endpoint can be extended to handle these validations)
//
// {
//   "status": 422,
//   "errors": [
//     {
//         "code": "invalid_birthdate",
//         "kind": "validation_error",
//         "message": "user birthdate is not correct",
//         "description": "validation error in your request",
//         "detail": { "birthdate": "2050.02.30" },
//         "reason": "must be a valid calendar date in the past",
//         "instance": "/api/v1/users/12345",
//         "trace_id": "8a250eaa650943b085934771fb35ba54",
//         "timestamp": "2024-01-19T16:59:03.124+0000",
//         "help": "please check if the user birthdate is correct or refer to our documentation at https://github.com/sheroz/axum-rest-api-sample/blob/main/docs/api-docs.md#errors for more information.",
//         "doc_url": "https://github.com/sheroz/axum-rest-api-sample/blob/main/docs/api-docs.md"
//     },
//     {
//         "code": "invalid_role",
//         "kind": "validation_error",
//         "message": "role not valid",
//         "description": "validation error in your request",
//         "detail": { role: "superadmin" },
//         "reason": "allowed roles: ['customer', 'guest']",
//         "instance": "/api/v1/users/12345",
//         "trace_id": "e023ebc3ab3e4c02b08247d9c5f03aa8",
//         "timestamp": "2024-01-19T16:59:03.124+0000",
//         "help": "please check if the user role is correct or refer to our documentation at https://github.com/sheroz/axum-rest-api-sample/blob/main/docs/api-docs.md#errors for more information",
//         "doc_url": "https://github.com/sheroz/axum-rest-api-sample/blob/main/docs/api-docs.md"
//     },

#[derive(Debug, Serialize, Deserialize)]
pub struct APIError {
    pub status: u16,
    pub errors: Vec<APIErrorEntry>,
}

impl Display for APIError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let api_error = serde_json::to_string_pretty(&self).unwrap_or_default();
        write!(f, "{}", api_error)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum APIErrorCode {
    AuthenticationWrongCredentials,
    AuthenticationMissingCredentials,
    AuthenticationTokenCreationError,
    AuthenticationInvalidToken,
    AuthenticationRevokedTokensInactive,
    AuthenticationForbidden,
    UserNotFound,
    TransactionNotFound,
    TransferInsufficientFunds,
    TransferSourceAccountNotFound,
    TransferDestinationAccountNotFound,
    TransferAccountsAreSame,
    ResourceNotFound,
    ApiVersionError,
    DatabaseError,
    RedisError,
}

impl Display for APIErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            serde_json::json!(self).as_str().unwrap_or_default()
        )
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum APIErrorKind {
    AuthenticationError,
    ResourceNotFound,
    ValidationError,
    DatabaseError,
    RedisError,
}

impl Display for APIErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            serde_json::json!(self).as_str().unwrap_or_default()
        )
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct APIErrorEntry {
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
    pub doc_url: Option<String>,
}

impl APIErrorEntry {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            timestamp: Utc::now(),
            ..Default::default()
        }
    }

    pub fn code<S: ToString>(mut self, code: S) -> Self {
        self.code = Some(code.to_string());
        self
    }

    pub fn kind<S: ToString>(mut self, kind: S) -> Self {
        self.kind = Some(kind.to_string());
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned());
        self
    }

    pub fn detail(mut self, detail: serde_json::Value) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn reason(mut self, reason: &str) -> Self {
        self.reason = Some(reason.to_owned());
        self
    }

    pub fn instance(mut self, instance: &str) -> Self {
        self.instance = Some(instance.to_owned());
        self
    }

    pub fn trace_id(mut self) -> Self {
        // Generate a new trace id.
        let mut trace_id = uuid::Uuid::new_v4().to_string();
        trace_id.retain(|c| c != '-');
        self.trace_id = Some(trace_id);
        self
    }

    pub fn help(mut self, help: &str) -> Self {
        self.help = Some(help.to_owned());
        self
    }

    pub fn doc_url(mut self) -> Self {
        self.doc_url = Some(API_DOCUMENT_URL.to_owned());
        self
    }
}

impl From<StatusCode> for APIErrorEntry {
    fn from(status_code: StatusCode) -> Self {
        let error_message = status_code.to_string();
        let error_code = error_message.replace(' ', "_").to_lowercase();
        Self::new(&error_message).code(error_code)
    }
}

impl From<sqlx::Error> for APIErrorEntry {
    fn from(e: sqlx::Error) -> Self {
        // Do not disclose database-related internal specifics, except for debug builds.
        if cfg!(debug_assertions) {
            let (code, kind) = match e {
                sqlx::Error::RowNotFound => (
                    APIErrorCode::ResourceNotFound,
                    APIErrorKind::ResourceNotFound,
                ),
                _ => (APIErrorCode::DatabaseError, APIErrorKind::DatabaseError),
            };
            Self::new(&e.to_string()).code(code).kind(kind).trace_id()
        } else {
            // Build the entry with a trace id to find the exact error in the log when needed.
            let error_entry = Self::from(StatusCode::INTERNAL_SERVER_ERROR).trace_id();
            let trace_id = error_entry.trace_id.as_deref().unwrap_or("");
            // The error must be logged here. Otherwise, we would lose it.
            tracing::error!("SQLx error: {}, trace id: {}", e.to_string(), trace_id);
            error_entry
        }
    }
}

impl From<redis::RedisError> for APIErrorEntry {
    fn from(e: redis::RedisError) -> Self {
        // Do not disclose Redis-related internal specifics, except for debug builds.
        if cfg!(debug_assertions) {
            Self::new(&e.to_string())
                .code(APIErrorCode::RedisError)
                .kind(APIErrorKind::RedisError)
                .description(&format!("Redis error: {}", e))
                .trace_id()
        } else {
            // Build the entry with a trace id to find the exact error in the log when needed.
            let error_entry = Self::from(StatusCode::INTERNAL_SERVER_ERROR).trace_id();
            let trace_id = error_entry.trace_id.as_deref().unwrap_or("");
            // The error must be logged here. Otherwise, we would lose it.
            tracing::error!("Redis error: {}, trace id: {}", e.to_string(), trace_id);
            error_entry
        }
    }
}

impl From<(StatusCode, Vec<APIErrorEntry>)> for APIError {
    fn from(error_from: (StatusCode, Vec<APIErrorEntry>)) -> Self {
        let (status_code, errors) = error_from;
        Self {
            status: status_code.as_u16(),
            errors,
        }
    }
}

impl From<(StatusCode, APIErrorEntry)> for APIError {
    fn from(error_from: (StatusCode, APIErrorEntry)) -> Self {
        let (status_code, error_entry) = error_from;
        Self {
            status: status_code.as_u16(),
            errors: vec![error_entry],
        }
    }
}

impl From<StatusCode> for APIError {
    fn from(status_code: StatusCode) -> Self {
        Self {
            status: status_code.as_u16(),
            errors: vec![status_code.into()],
        }
    }
}

impl From<sqlx::Error> for APIError {
    fn from(error: sqlx::Error) -> Self {
        let status_code = match error {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self {
            status: status_code.as_u16(),
            errors: vec![APIErrorEntry::from(error)],
        }
    }
}

impl From<redis::RedisError> for APIError {
    fn from(error: redis::RedisError) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            errors: vec![APIErrorEntry::from(error)],
        }
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        tracing::error!("Error response: {:?}", self);
        let status_code =
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status_code, Json(self)).into_response()
    }
}
