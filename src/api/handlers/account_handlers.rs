use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::types::Uuid;

use crate::{
    api::{
        APIError,
        version::{self, APIVersion},
    },
    application::{
        repository::account_repo,
        security::jwt::{AccessClaims, ClaimsMethods},
        state::SharedState,
    },
    domain::models::account::Account,
};

pub async fn list_accounts_handler(
    api_version: APIVersion,
    access_claims: AccessClaims,
    State(state): State<SharedState>,
) -> Result<Json<Vec<Account>>, APIError> {
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);

    access_claims.validate_role_admin()?;

    let mut connection = state.db_pool.acquire().await?;
    let accounts = account_repo::list(&mut connection).await?;
    Ok(Json(accounts))
}

pub async fn add_account_handler(
    api_version: APIVersion,
    access_claims: AccessClaims,
    State(state): State<SharedState>,
    Json(account): Json<Account>,
) -> Result<impl IntoResponse, APIError> {
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);

    access_claims.validate_role_admin()?;

    let mut connection = state.db_pool.acquire().await?;
    let account = account_repo::add(account, &mut connection).await?;
    Ok((StatusCode::CREATED, Json(account)))
}

pub async fn get_account_handler(
    access_claims: AccessClaims,
    Path((version, id)): Path<(String, Uuid)>,
    State(state): State<SharedState>,
) -> Result<Json<Account>, APIError> {
    let api_version: APIVersion = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("id: {}", id);

    access_claims.validate_role_admin()?;

    let mut connection = state.db_pool.acquire().await?;
    let account = account_repo::get_by_id(id, &mut connection).await?;
    Ok(Json(account))
}

pub async fn update_account_handler(
    access_claims: AccessClaims,
    Path((version, id)): Path<(String, Uuid)>,
    State(state): State<SharedState>,
    Json(account): Json<Account>,
) -> Result<Json<Account>, APIError> {
    let api_version: APIVersion = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("id: {}", id);
    tracing::trace!("account: {:?}", account);
    access_claims.validate_role_admin()?;

    let mut connection = state.db_pool.acquire().await?;
    let account = account_repo::update(account, &mut connection).await?;
    Ok(Json(account))
}

pub async fn delete_account_handler(
    access_claims: AccessClaims,
    Path((version, id)): Path<(String, Uuid)>,
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, APIError> {
    let api_version: APIVersion = version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("id: {}", id);
    access_claims.validate_role_admin()?;

    let mut connection = state.db_pool.acquire().await.unwrap();
    if account_repo::delete(id, &mut connection).await? {
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::NOT_FOUND)?
    }
}
