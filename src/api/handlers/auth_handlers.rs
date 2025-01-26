use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::types::Uuid;

use crate::{
    api::{api_error::ApiError, api_version::ApiVersion},
    application::{
        repository::user_repo,
        security::{
            auth_error::AuthError,
            jwt_auth::{self, JwtTokens},
            jwt_claims::{AccessClaims, ClaimsMethods, RefreshClaims},
        },
        service::token_service,
        state::SharedState,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    username: String,
    password_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevokeUser {
    user_id: Uuid,
}

#[tracing::instrument(level = tracing::Level::TRACE, name = "login", skip_all, fields(username=login.username))]
pub async fn login_handler(
    api_version: ApiVersion,
    State(state): State<SharedState>,
    Json(login): Json<LoginUser>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::trace!("api version: {}", api_version);
    let user = user_repo::get_by_username(&login.username, &state).await?;
    if user.active && user.password_hash == login.password_hash {
        tracing::trace!("access granted, user: {}", user.id);
        let tokens = jwt_auth::generate_tokens(user, &state.config);
        let response = tokens_to_response(tokens);
        return Ok(response);
    }

    tracing::error!("access denied: {:#?}", login);
    Err(AuthError::WrongCredentials)?
}

pub async fn logout_handler(
    api_version: ApiVersion,
    State(state): State<SharedState>,
    refresh_claims: RefreshClaims,
) -> Result<impl IntoResponse, ApiError> {
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("refresh_claims: {:?}", refresh_claims);
    jwt_auth::logout(refresh_claims, state).await
}

pub async fn refresh_handler(
    api_version: ApiVersion,
    State(state): State<SharedState>,
    refresh_claims: RefreshClaims,
) -> Result<impl IntoResponse, ApiError> {
    tracing::trace!("api version: {}", api_version);
    let new_tokens = jwt_auth::refresh(refresh_claims, state).await?;
    Ok(tokens_to_response(new_tokens))
}

// Revoke all issued tokens until now.
pub async fn revoke_all_handler(
    api_version: ApiVersion,
    State(state): State<SharedState>,
    access_claims: AccessClaims,
) -> Result<impl IntoResponse, ApiError> {
    tracing::trace!("api version: {}", api_version);
    access_claims.validate_role_admin()?;
    if !token_service::revoke_global(&state).await {
        Err(StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    Ok(())
}

// Revoke tokens issued to user until now.
pub async fn revoke_user_handler(
    api_version: ApiVersion,
    State(state): State<SharedState>,
    access_claims: AccessClaims,
    Json(revoke_user): Json<RevokeUser>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::trace!("api version: {}", api_version);
    if access_claims.sub != revoke_user.user_id.to_string() {
        // Only admin can revoke tokens of other users.
        access_claims.validate_role_admin()?;
    }
    tracing::trace!("revoke_user: {:?}", revoke_user);
    if !token_service::revoke_user_tokens(&revoke_user.user_id.to_string(), &state).await {
        Err(StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    Ok(())
}

pub async fn cleanup_handler(
    api_version: ApiVersion,
    State(state): State<SharedState>,
    access_claims: AccessClaims,
) -> Result<impl IntoResponse, ApiError> {
    tracing::trace!("api version: {}", api_version);
    access_claims.validate_role_admin()?;
    tracing::trace!("authentication details: {:#?}", access_claims);
    let deleted = jwt_auth::cleanup_revoked_and_expired(&access_claims, &state).await?;
    let json = json!({
        "deleted_tokens": deleted,
    });
    Ok(Json(json))
}

fn tokens_to_response(jwt_tokens: JwtTokens) -> impl IntoResponse {
    let json = json!({
        "access_token": jwt_tokens.access_token,
        "refresh_token": jwt_tokens.refresh_token,
        "token_type": "Bearer"
    });

    tracing::trace!("JWT: generated response {:#?}", json);
    Json(json)
}
