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
        api_error::ApiError,
        api_version::{self, ApiVersion},
        repository::transaction_repo,
        security::jwt_claims::{AccessClaims, ClaimsMethods},
        service::transaction_service,
        state::SharedState,
    },
    domain::models::transaction::{Transaction, TransactionResult},
};

#[derive(Debug, Serialize, Deserialize)]
struct TransferOrder {
    from_account_id: Uuid,
    to_account_id: Uuid,
    amount_cents: i64,
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
) -> Result<Json<TransactionResult>, ApiError> {
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("transfer: {:?}", transfer_order);

    access_claims.validate_role_admin()?;

    match transaction_service::transfer(
        transfer_order.from_account_id,
        transfer_order.to_account_id,
        transfer_order.amount_cents,
        &state,
    )
    .await
    {
        Ok(transaction_result) => Ok(Json(transaction_result)),
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into())
        }
    }
}
