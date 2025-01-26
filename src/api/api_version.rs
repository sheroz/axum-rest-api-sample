use std::collections::HashMap;

use axum::{
    extract::{FromRequestParts, Path},
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use thiserror::Error;

use crate::api::api_error::{ApiError, ApiErrorCode, ApiErrorEntry, ApiErrorKind};

#[derive(Debug, Clone, Copy)]
pub enum ApiVersion {
    V1,
    V2,
}

impl std::str::FromStr for ApiVersion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "v1" => Ok(Self::V1),
            "v2" => Ok(Self::V2),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Self::V1 => "v1",
            Self::V2 => "v2",
        };
        write!(f, "{}", v)
    }
}

pub fn parse_version(version: &str) -> Result<ApiVersion, ApiError> {
    version.parse().map_or_else(
        |_| Err(ApiVersionError::InvalidVersion(version.to_owned()).into()),
        |v| Ok(v),
    )
}

impl<S> FromRequestParts<S> for ApiVersion
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let params: Path<HashMap<String, String>> = parts
            .extract()
            .await
            .map_err(|_| ApiVersionError::VersionExtractError)?;

        let version = params
            .get("version")
            .ok_or(ApiVersionError::ParameterMissing)?;

        parse_version(version)
    }
}

#[derive(Debug, Error)]
pub enum ApiVersionError {
    #[error("unknown version: {0}")]
    InvalidVersion(String),
    #[error("parameter is missing: version")]
    ParameterMissing,
    #[error("could not extract api version")]
    VersionExtractError,
}

impl From<ApiVersionError> for ApiError {
    fn from(err: ApiVersionError) -> Self {
        let error_entry = ApiErrorEntry::new(&err.to_string())
            .code(ApiErrorCode::ApiVersionError)
            .kind(ApiErrorKind::ValidationError);

        (StatusCode::BAD_REQUEST, error_entry).into()
    }
}
