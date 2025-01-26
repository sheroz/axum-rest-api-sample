use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use thiserror::Error;

use crate::{
    api::{
        api_error::{ApiError, ApiErrorCode, ApiErrorEntry, ApiErrorKind},
        api_version::{self, ApiVersion},
    },
    application::{
        repository::transaction_repo,
        security::jwt_claims::{AccessClaims, ClaimsMethods},
        service::transaction_service::{self, TransferError, TransferValidationError},
        state::SharedState,
    },
    domain::models::transaction::Transaction,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferOrder {
    pub source_account_id: Uuid,
    pub destination_account_id: Uuid,
    pub amount_cents: i64,
}

pub async fn get_transaction_handler(
    access_claims: AccessClaims,
    Path((version, id)): Path<(String, Uuid)>,
    State(state): State<SharedState>,
) -> Result<Json<Transaction>, ApiError> {
    let api_version: ApiVersion = api_version::parse_version(&version)?;
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("id: {}", id);

    access_claims.validate_role_admin()?;

    let transaction = transaction_repo::get_by_id(id, &state)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::from(TransactionError::TransactionNotFound(id)),
            _ => e.into(),
        })?;

    Ok(Json(transaction))
}

pub async fn transfer_handler(
    api_version: ApiVersion,
    access_claims: AccessClaims,
    State(state): State<SharedState>,
    Json(transfer_order): Json<TransferOrder>,
) -> Result<Json<Transaction>, ApiError> {
    tracing::trace!("api version: {}", api_version);
    tracing::trace!("authentication details: {:#?}", access_claims);
    tracing::trace!("transfer: {:?}", transfer_order);

    access_claims.validate_role_admin()?;

    let transaction = transaction_service::transfer(
        transfer_order.source_account_id,
        transfer_order.destination_account_id,
        transfer_order.amount_cents,
        &state,
    )
    .await?;

    Ok(Json(transaction))
}

#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("transaction not found: {0}")]
    TransactionNotFound(Uuid),
}

impl From<TransactionError> for ApiError {
    fn from(error: TransactionError) -> Self {
        (error.status_code(), vec![ApiErrorEntry::from(error)]).into()
    }
}

impl TransactionError {
    const fn status_code(&self) -> StatusCode {
        match self {
            Self::TransactionNotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}

impl From<TransactionError> for ApiErrorEntry {
    fn from(transaction_error: TransactionError) -> Self {
        let error = Self::new(&transaction_error.to_string());
        match transaction_error {
            TransactionError::TransactionNotFound(transaction_id) => error
                .code(ApiErrorCode::TransactionNotFound)
                .kind(ApiErrorKind::ResourceNotFound)
                .detail(serde_json::json!({"transaction_id": transaction_id}))
                .trace_id(),
        }
    }
}

impl From<TransferError> for ApiError {
    fn from(transfer_error: TransferError) -> Self {
        match transfer_error {
            TransferError::TransferValidationErrors(validation_errors) => {
                let errors: Vec<_> = validation_errors
                    .into_iter()
                    .map(ApiErrorEntry::from)
                    .collect();
                (StatusCode::UNPROCESSABLE_ENTITY, errors).into()
            }
            TransferError::SQLxError(e) => e.into(),
        }
    }
}

impl From<TransferValidationError> for ApiErrorEntry {
    fn from(transfer_validation_error: TransferValidationError) -> Self {
        let error = Self::new(&transfer_validation_error.to_string());
        match transfer_validation_error {
            TransferValidationError::InsufficientFunds => error
                .code(ApiErrorCode::TransactionInsufficientFunds)
                .kind(ApiErrorKind::ValidationError)
                .description(
                    "there are insufficient funds in the source account for the transfer".into(),
                )
                .trace_id(),
            TransferValidationError::SourceAccountNotFound(source_account_id) => error
                .code(ApiErrorCode::TransactionSourceAccountNotFound)
                .kind(ApiErrorKind::ValidationError)
                .detail(serde_json::json!({"source_account_id": source_account_id}))
                .trace_id(),
            TransferValidationError::DestinationAccountNotFound(destination_account_id) => error
                .code(ApiErrorCode::TransactionDestinationAccountNotFound)
                .kind(ApiErrorKind::ValidationError)
                .detail(serde_json::json!({"destination_account_id": destination_account_id}))
                .trace_id(),
        }
    }
}
