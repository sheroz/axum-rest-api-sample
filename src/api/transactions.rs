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
        api_error::{DetailedError, DetailedErrorCode, DetailedErrorKind, DetailedErrorResponse},
        api_version::{self, ApiVersion},
        repository::transaction_repo,
        security::jwt_claims::{AccessClaims, ClaimsMethods},
        service::transaction_service::{self, TransactionError},
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
) -> Result<Json<Transaction>, DetailedErrorResponse> {
    let api_version: ApiVersion = api_version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("id: {}", id);
    access_claims.validate_role_admin()?;
    let transaction = transaction_repo::get_by_id(id, &state)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => TransactionError::TransactionNotFound(id),
            _ => e.into(),
        })?;

    Ok(Json(transaction))
}

async fn transfer_handler(
    api_version: ApiVersion,
    access_claims: AccessClaims,
    State(state): State<SharedState>,
    Json(transfer_order): Json<TransferOrder>,
) -> Result<Json<Transaction>, DetailedErrorResponse> {
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

impl From<TransactionError> for DetailedErrorResponse {
    fn from(transaction_error: TransactionError) -> Self {
        let mut error = DetailedError::new(&transaction_error.to_string());
        match transaction_error {
            TransactionError::TransactionNotFound(transaction_id) => {
                error.code = serde_json::to_string(&DetailedErrorCode::TransactionNotFound).ok();
                error.kind = Some(DetailedErrorKind::ResourceNotFound);
                error.detail = Some(serde_json::json!({"transaction_id": transaction_id}));
                Self::from(StatusCode::NOT_FOUND, vec![error])
            }
            TransactionError::InsufficientFunds => {
                error.code = serde_json::to_string(&DetailedErrorCode::SourceAccountNotFound).ok();
                error.kind = Some(DetailedErrorKind::ValidationError);
                error.description = Some(
                    "there are insufficient funds in the source account for the transfer".into(),
                );
                Self::from(StatusCode::UNPROCESSABLE_ENTITY, vec![error])
            }
            TransactionError::SourceAccountNotFound(account_id) => {
                error.code = serde_json::to_string(&DetailedErrorCode::SourceAccountNotFound).ok();
                error.kind = Some(DetailedErrorKind::ValidationError);
                error.detail = Some(serde_json::json!({"account_id": account_id}));
                Self::from(StatusCode::UNPROCESSABLE_ENTITY, vec![error])
            }
            TransactionError::DestinationAccountNotFound(account_id) => {
                error.code =
                    serde_json::to_string(&DetailedErrorCode::DestinationAccountNotFound).ok();
                error.kind = Some(DetailedErrorKind::ValidationError);
                error.detail = Some(serde_json::json!({"account_id": account_id}));
                Self::from(StatusCode::UNPROCESSABLE_ENTITY, vec![error])
            }
            TransactionError::SQLxError(e) => {
                error.code = serde_json::to_string(&DetailedErrorCode::DatabaseError).ok();
                error.kind = Some(DetailedErrorKind::DatabaseError);
                error.description = Some(format!("Database error occured: {}", e));
                Self::from(StatusCode::INTERNAL_SERVER_ERROR, vec![error])
            }
        }
    }
}
