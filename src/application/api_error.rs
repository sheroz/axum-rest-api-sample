use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// API error response samples:
//
// {
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
//         "help": "please check if the user ID is correct or refer to our documentation at https://api.example.com/docs/errors#user_not_found for more information",
//         "doc_url": "https://api.example.com/docs/errors"
//     }
//   ]
// }
//
// {
//   "errors": [
//     {
//         "code": "invalid_email",
//         "kind": "validation_error",
//         "message": "user email is not valid",
//         "description": "validation error in your request",
//         "detail": { "email": "xyz@12345" },
//         "reason": "must be a valid email address",
//         "instance": "/api/v1/users/12345",
//         "trace_id": "fbb9fdf5394d4abe8e42b49c3246310b",
//         "timestamp": "2024-01-19T16:58:35.225+0000",
//         "help": "please check if the user email is correct or refer to our documentation at https://api.example.com/docs/errors#invalid_email for more information",
//         "doc_url": "https://api.example.com/docs/errors"
//     },
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
//         "help": "please check if the user birthdate is correct or refer to our documentation at https://api.example.com/docs/errors#invalid_birthdate for more information."
//         "doc_url": "https://api.example.com/docs/errors"
//     },
//     {
//         "code": "invalid_role",
//         "kind": "validation_error",
//         "message": "user birthdate is not correct",
//         "description": "validation error in your request",
//         "detail": { role: "superadmin" },
//         "reason": "allowed roles: ['customer', 'guest']",
//         "instance": "/api/v1/users/12345",
//         "trace_id": "e023ebc3ab3e4c02b08247d9c5f03aa8",
//         "timestamp": "2024-01-19T16:59:03.124+0000",
//         "help": "please check if the user role is correct or refer to our documentation at https://api.example.com/docs/errors#invalid_birthdate for more information",
//         "doc_url": "https://api.example.com/docs/errors"
//     },
//   ]
// }
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    #[serde(skip)]
    pub status: StatusCode,
    pub errors: Vec<ApiErrorEntry>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorCode {
    AuthWrongCredentials,
    AuthMissingCredentials,
    AuthTokenCreationError,
    AuthInvalidToken,
    AuthForbidden,
    UserNotFound,
    TransactionNotFound,
    TransactionInsufficientFunds,
    TransactionSourceAccountNotFound,
    TransactionDestinationAccountNotFound,
    ResourceNotFound,
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
pub struct ApiErrorEntry {
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

impl ApiErrorEntry {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            timestamp: Utc::now(),
            ..Default::default()
        }
    }

    pub fn code<S: Serialize>(mut self, code: S) -> Self {
        self.code = serde_json::to_string(&code).ok();
        self
    }

    pub fn kind<S: Serialize>(mut self, kind: S) -> Self {
        self.kind = serde_json::to_string(&kind).ok();
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

    pub fn doc_url(mut self, doc_url: &str) -> Self {
        self.doc_url = Some(doc_url.to_owned());
        self
    }
}

impl From<sqlx::Error> for ApiErrorEntry {
    fn from(e: sqlx::Error) -> Self {
        Self::new(&e.to_string())
            .code(ApiErrorCode::DatabaseError)
            .kind(ApiErrorKind::DatabaseError)
            .description(&format!("Database error: {}", e))
    }
}

impl From<(StatusCode, Vec<ApiErrorEntry>)> for ApiError {
    fn from(error_from: (StatusCode, Vec<ApiErrorEntry>)) -> Self {
        let (status, errors) = error_from;
        Self { status, errors }
    }
}

impl From<(StatusCode, ApiErrorEntry)> for ApiError {
    fn from(error_from: (StatusCode, ApiErrorEntry)) -> Self {
        let (status, error_entry) = error_from;
        Self {
            status,
            errors: vec![error_entry],
        }
    }
}

impl From<StatusCode> for ApiError {
    fn from(status: StatusCode) -> Self {
        Self {
            status,
            errors: vec![],
        }
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
            errors: vec![ApiErrorEntry::from(error)],
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        tracing::error!("Error response: {:?}", self);
        (self.status, Json(self)).into_response()
    }
}
