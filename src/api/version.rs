use std::collections::HashMap;

use axum::{
    RequestPartsExt,
    extract::{FromRequestParts, Path},
    http::{StatusCode, request::Parts},
};
use thiserror::Error;

use crate::api::error::{APIError, APIErrorCode, APIErrorEntry, APIErrorKind};

#[derive(Debug, Clone, Copy)]
pub enum APIVersion {
    V1,
    V2,
}

impl std::str::FromStr for APIVersion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "v1" => Ok(Self::V1),
            "v2" => Ok(Self::V2),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for APIVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Self::V1 => "v1",
            Self::V2 => "v2",
        };
        write!(f, "{}", v)
    }
}

pub fn parse_version(version: &str) -> Result<APIVersion, APIError> {
    version.parse().map_or_else(
        |_| Err(ApiVersionError::InvalidVersion(version.to_owned()).into()),
        |v| Ok(v),
    )
}

impl<S> FromRequestParts<S> for APIVersion
where
    S: Send + Sync,
{
    type Rejection = APIError;

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

impl From<ApiVersionError> for APIError {
    fn from(err: ApiVersionError) -> Self {
        let error_entry = APIErrorEntry::new(&err.to_string())
            .code(APIErrorCode::ApiVersionError)
            .kind(APIErrorKind::ValidationError);

        (StatusCode::BAD_REQUEST, error_entry).into()
    }
}
