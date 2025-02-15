use thiserror::Error;
use uuid::Uuid;

use crate::{
    application::{
        repository::{account_repo, transaction_repo},
        state::SharedState,
    },
    domain::models::transaction::Transaction,
};

pub async fn transfer(
    source_account_id: Uuid,
    destination_account_id: Uuid,
    amount_cents: i64,
    state: &SharedState,
) -> Result<Transaction, TransferError> {
    tracing::trace!(
        "transfer: source_account_id: {}, destination_account_id: {}, amount_cents: {} ",
        source_account_id,
        destination_account_id,
        amount_cents
    );

    // Start transaction.
    let mut tx = state.db_pool.begin().await?;

    let mut validation_errors = vec![];

    // Find the source account.
    let mut source_account = match account_repo::get_by_id(source_account_id, &mut tx).await {
        Ok(account) => {
            // Check the balance of the source account.
            if account.balance_cents < amount_cents {
                validation_errors.push(TransferValidationError::InsufficientFunds);
            }
            Some(account)
        }
        Err(e) => {
            let error = match e {
                sqlx::Error::RowNotFound => {
                    TransferValidationError::SourceAccountNotFound(source_account_id)
                }
                _ => Err(e)?,
            };
            validation_errors.push(error);
            None
        }
    };

    // Check if accounts are distinct.
    if source_account_id == destination_account_id {
        validation_errors.push(TransferValidationError::AccountsAreSame);

        // No need for futher validations.
        return Err(TransferError::TransferValidationErrors(validation_errors))?;
    }

    // Find the destination account.
    let mut destination_account =
        match account_repo::get_by_id(destination_account_id, &mut tx).await {
            Ok(account) => Some(account),
            Err(e) => {
                let error = match e {
                    sqlx::Error::RowNotFound => {
                        TransferValidationError::DestinationAccountNotFound(destination_account_id)
                    }
                    _ => Err(e)?,
                };
                validation_errors.push(error);
                None
            }
        };

    if !validation_errors.is_empty() {
        Err(TransferError::TransferValidationErrors(validation_errors))?
    }

    let mut source_account = source_account.take().unwrap();
    let mut destination_account = destination_account.take().unwrap();

    // Transfer money.
    source_account.balance_cents -= amount_cents;
    destination_account.balance_cents += amount_cents;

    // Update accounts.
    account_repo::update(source_account, &mut tx).await?;
    account_repo::update(destination_account, &mut tx).await?;

    // Add transaction.
    let transaction = transaction_repo::add(
        source_account_id,
        destination_account_id,
        amount_cents,
        &mut tx,
    )
    .await?;

    // Commit transaction.
    tx.commit().await?;

    Ok(transaction)
}

#[derive(Debug, Error)]
pub enum TransferError {
    #[error("transfer validation errors")]
    TransferValidationErrors(Vec<TransferValidationError>),
    #[error(transparent)]
    SQLxError(#[from] sqlx::Error),
}

#[derive(Debug, Error)]
pub enum TransferValidationError {
    #[error("source account does not have sufficient funds for the transfer")]
    InsufficientFunds,
    #[error("source account not found: {0}")]
    SourceAccountNotFound(Uuid),
    #[error("destination account not found: {0}")]
    DestinationAccountNotFound(Uuid),
    #[error("source and destination accounts are the same")]
    AccountsAreSame,
}
