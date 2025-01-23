use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::{
    application::{
        api_error::{ApiError, DetailedError},
        api_version::{self, ApiVersion},
        repository::transaction_repo,
        security::jwt_claims::{AccessClaims, ClaimsMethods},
        service::transaction_service,
        state::SharedState,
    },
    domain::models::transaction::Transaction,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferOrder {
    pub from_account_id: Uuid,
    pub to_account_id: Uuid,
    pub amount_cents: i64,
}

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/transfer", post(transfer_handler))
        .route("/{id}", get(get_transaction_handler))
}

async fn get_transaction_handler(
    access_claims: AccessClaims,
    Path((version, id)): Path<(String, Uuid)>,
    State(state): State<SharedState>,
) -> Result<Json<Transaction>, ApiError> {
    let api_version: ApiVersion = api_version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("id: {}", id);
    access_claims.validate_role_admin()?;
    match transaction_repo::get_by_id(id, &state).await {
        Ok(account) => Ok(Json(account)),
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::NOT_FOUND.into())
        }
    }
}

async fn transfer_handler(
    api_version: ApiVersion,
    access_claims: AccessClaims,
    State(state): State<SharedState>,
    Json(transfer_order): Json<TransferOrder>,
) -> Result<Json<Transaction>, DetailedError> {
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("transfer: {:?}", transfer_order);

    access_claims.validate_role_admin()?;

    let transaction = transaction_service::transfer(
        transfer_order.from_account_id,
        transfer_order.to_account_id,
        transfer_order.amount_cents,
        &state,
    )
    .await?;

    Ok(Json(transaction))
}
