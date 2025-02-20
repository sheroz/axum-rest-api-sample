use std::sync::Arc;

use axum::{
    RequestPartsExt,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use crate::{
    api::APIError,
    application::{
        security::{
            auth::{self, AuthError},
            jwt::{AccessClaims, ClaimsMethods, RefreshClaims, decode_token},
        },
        state::SharedState,
    },
};

impl<S> FromRequestParts<S> for AccessClaims
where
    SharedState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = APIError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        decode_token_from_request_part(parts, state).await
    }
}

impl<S> FromRequestParts<S> for RefreshClaims
where
    SharedState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = APIError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        decode_token_from_request_part(parts, state).await
    }
}

async fn decode_token_from_request_part<S, T>(parts: &mut Parts, state: &S) -> Result<T, APIError>
where
    SharedState: FromRef<S>,
    S: Send + Sync,
    T: for<'de> serde::Deserialize<'de> + std::fmt::Debug + ClaimsMethods + Sync + Send,
{
    // Extract the token from the authorization header.
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| {
            tracing::error!("Invalid authorization header");
            AuthError::WrongCredentials
        })?;

    // Take the state from a reference.
    let state = Arc::from_ref(state);

    // Decode the token.
    let claims = decode_token::<T>(bearer.token(), &state.config)?;

    // Check for revoked tokens if enabled by configuration.
    if state.config.jwt_enable_revoked_tokens {
        auth::validate_revoked(&claims, &state).await?
    }
    Ok(claims)
}
