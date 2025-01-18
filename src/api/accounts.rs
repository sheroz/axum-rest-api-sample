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
        repository::account_repo,
        security::jwt_claims::{AccessClaims, ClaimsMethods},
        state::SharedState,
    },
    domain::models::account::Account,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(list_accounts_handler))
        .route("/", post(add_account_handler))
        .route("/{id}", get(get_account_handler))
        .route("/{id}", put(update_account_handler))
        .route("/{id}", delete(delete_account_handler))
}

async fn list_accounts_handler(
    api_version: ApiVersion,
    access_claims: AccessClaims,
    State(state): State<SharedState>,
) -> Result<Json<Vec<Account>>, ApiError> {
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);

    access_claims.validate_role_admin()?;

    let mut connection = state.db_pool.acquire().await.unwrap();
    match account_repo::list(&mut connection).await {
        Ok(accounts) => Ok(Json(accounts)),
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into())
        }
    }
}

async fn add_account_handler(
    api_version: ApiVersion,
    access_claims: AccessClaims,
    State(state): State<SharedState>,
    Json(account): Json<Account>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);

    access_claims.validate_role_admin()?;

    let mut connection = state.db_pool.acquire().await.unwrap();
    match account_repo::add(account, &mut connection).await {
        Ok(account) => Ok((StatusCode::CREATED, Json(account))),
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into())
        }
    }
}

async fn get_account_handler(
    access_claims: AccessClaims,
    Path((version, id)): Path<(String, Uuid)>,
    State(state): State<SharedState>,
) -> Result<Json<Account>, ApiError> {
    let api_version: ApiVersion = api_version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("id: {}", id);

    access_claims.validate_role_admin()?;

    let mut connection = state.db_pool.acquire().await.unwrap();
    match account_repo::get_by_id(id, &mut connection).await {
        Ok(account) => Ok(Json(account)),
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::NOT_FOUND.into())
        }
    }
}

async fn update_account_handler(
    access_claims: AccessClaims,
    Path((version, id)): Path<(String, Uuid)>,
    State(state): State<SharedState>,
    Json(account): Json<Account>,
) -> Result<Json<Account>, ApiError> {
    let api_version: ApiVersion = api_version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("id: {}", id);
    tracing::trace!("account: {:?}", account);
    access_claims.validate_role_admin()?;

    let mut connection = state.db_pool.acquire().await.unwrap();
    match account_repo::update(account, &mut connection).await {
        Ok(account) => Ok(Json(account)),
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into())
        }
    }
}

async fn delete_account_handler(
    access_claims: AccessClaims,
    Path((version, id)): Path<(String, Uuid)>,
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, ApiError> {
    let api_version: ApiVersion = api_version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("id: {}", id);
    access_claims.validate_role_admin()?;

    let mut connection = state.db_pool.acquire().await.unwrap();
    match account_repo::delete(id, &mut connection).await {
        Ok(true) => Ok(StatusCode::OK),
        Ok(false) => Err(ApiError {
            status_code: StatusCode::NOT_FOUND,
            error_message: format!("Account not found for deletion: {}", id),
        }),
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into())
        }
    }
}
