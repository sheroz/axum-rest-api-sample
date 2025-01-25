use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use thiserror::Error;

use crate::{
    application::{
        api_error::{ApiError, ApiErrorCode, ApiErrorEntry, ApiErrorKind},
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
    pub source_account_id: Uuid,
    pub destination_account_id: Uuid,
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

    let transaction = transaction_repo::get_by_id(id, &state)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::from(TransactionError::TransactionNotFound(id)),
            _ => e.into(),
        })?;

    Ok(Json(transaction))
}

async fn transfer_handler(
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
    #[error("insufficient funds")]
    InsufficientFunds,
    #[error("source account not found: {0}")]
    SourceAccountNotFound(Uuid),
    #[error("destination account not found: {0}")]
    DestinationAccountNotFound(Uuid),
}

impl From<TransactionError> for ApiError {
    fn from(error: TransactionError) -> Self {
        let status = StatusCode::from(&error);
        let errors = vec![error.into()];
        (status, errors).into()
    }
}

impl From<(StatusCode, Vec<TransactionError>)> for ApiError {
    fn from(error_from: (StatusCode, Vec<TransactionError>)) -> Self {
        let (status, errors) = error_from;
        Self {
            status,
            errors: errors.into_iter().map(ApiErrorEntry::from).collect(),
        }
    }
}

impl From<&TransactionError> for StatusCode {
    fn from(error: &TransactionError) -> Self {
        match error {
            TransactionError::TransactionNotFound(_) => Self::NOT_FOUND,
            TransactionError::InsufficientFunds
            | TransactionError::SourceAccountNotFound(_)
            | TransactionError::DestinationAccountNotFound(_) => Self::UNPROCESSABLE_ENTITY,
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
                .detail(serde_json::json!({"transaction_id": transaction_id})),
            TransactionError::InsufficientFunds => error
                .code(ApiErrorCode::TransactionInsufficientFunds)
                .kind(ApiErrorKind::ValidationError)
                .description(
                    "there are insufficient funds in the source account for the transfer".into(),
                ),
            TransactionError::SourceAccountNotFound(source_account_id) => error
                .code(ApiErrorCode::TransactionSourceAccountNotFound)
                .kind(ApiErrorKind::ValidationError)
                .detail(serde_json::json!({"source_account_id": source_account_id})),
            TransactionError::DestinationAccountNotFound(destination_account_id) => error
                .code(ApiErrorCode::TransactionDestinationAccountNotFound)
                .kind(ApiErrorKind::ValidationError)
                .detail(serde_json::json!({"destination_account_id": destination_account_id})),
        }
    }
}

#[derive(Debug, Default)]
pub struct TransferValidationErrors {
    errors: Vec<TransactionError>,
}

impl TransferValidationErrors {
    pub fn add(&mut self, error: TransactionError) {
        self.errors.push(error);
    }
    pub fn exists(&self) -> bool {
        self.errors.len() > 0
    }
}

impl From<TransferValidationErrors> for ApiError {
    fn from(validation_errors: TransferValidationErrors) -> Self {
        let status = StatusCode::UNPROCESSABLE_ENTITY;
        let errors = validation_errors.errors;
        (status, errors).into()
    }
}
