use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use sqlx::types::Uuid;

use crate::{
    application::{
        api_error::ApiError,
        api_version::{self, ApiVersion},
        repository::user_repo,
        security::jwt_claims::{AccessClaims, ClaimsMethods},
        state::SharedState,
    },
    domain::models::user::User,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(list_users_handler))
        .route("/", post(add_user_handler))
        .route("/{id}", get(get_user_handler))
        .route("/{id}", put(update_user_handler))
        .route("/{id}", delete(delete_user_handler))
}

async fn list_users_handler(
    api_version: ApiVersion,
    access_claims: AccessClaims,
    State(state): State<SharedState>,
) -> Result<Json<Vec<User>>, ApiError> {
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    access_claims.validate_role_admin()?;
    let users = user_repo::list(&state).await?;
    Ok(Json(users))
}

async fn add_user_handler(
    api_version: ApiVersion,
    access_claims: AccessClaims,
    State(state): State<SharedState>,
    Json(user): Json<User>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    access_claims.validate_role_admin()?;
    let user = user_repo::add(user, &state).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

async fn get_user_handler(
    access_claims: AccessClaims,
    Path((version, id)): Path<(String, Uuid)>,
    State(state): State<SharedState>,
) -> Result<Json<User>, ApiError> {
    let api_version: ApiVersion = api_version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("id: {}", id);
    access_claims.validate_role_admin()?;
    let user = user_repo::get_by_id(id, &state).await?;
    Ok(Json(user))
}

async fn update_user_handler(
    access_claims: AccessClaims,
    Path((version, id)): Path<(String, Uuid)>,
    State(state): State<SharedState>,
    Json(user): Json<User>,
) -> Result<Json<User>, ApiError> {
    let api_version: ApiVersion = api_version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("id: {}", id);
    access_claims.validate_role_admin()?;
    let user = user_repo::update(user, &state).await?;
    Ok(Json(user))
}

async fn delete_user_handler(
    access_claims: AccessClaims,
    Path((version, id)): Path<(String, Uuid)>,
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, ApiError> {
    let api_version: ApiVersion = api_version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("id: {}", id);
    access_claims.validate_role_admin()?;
    if user_repo::delete(id, &state).await? {
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::NOT_FOUND)?
    }
}
