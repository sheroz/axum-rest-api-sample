use thiserror::Error;
use uuid::Uuid;

use crate::{
    application::{
        repository::{account_repo, transaction_repo},
        state::SharedState,
    },
    domain::models::transaction::Transaction,
};

#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("insufficient funds")]
    InsufficientFunds,
    #[error("source account not found: {0}")]
    SourceAccountNotFound(Uuid),
    #[error("destination account not found: {0}")]
    DestinationAccountNotFound(Uuid),
    #[error(transparent)]
    SQLxError(#[from] sqlx::Error),
}

pub async fn transfer(
    from_account_id: Uuid,
    to_account_id: Uuid,
    amount_cents: i64,
    state: &SharedState,
) -> Result<Transaction, TransactionError> {
    tracing::trace!(
        "transfer: from_account_id: {}, to_account_id: {}, amount_cents: {} ",
        from_account_id,
        to_account_id,
        amount_cents
    );

    // Start transaction.
    let mut tx = state.db_pool.begin().await?;

    // Find the source account.
    let mut from_account = account_repo::get_by_id(from_account_id, &mut tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => TransactionError::SourceAccountNotFound(from_account_id),
            _ => e.into(),
        })?;

    // Find the destination account.
    let mut to_account = account_repo::get_by_id(to_account_id, &mut tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => TransactionError::DestinationAccountNotFound(to_account_id),
            _ => e.into(),
        })?;

    // Check the source balance.
    if from_account.balance_cents < amount_cents {
        return Err(TransactionError::InsufficientFunds.into());
    }

    // Transfer money.
    from_account.balance_cents -= amount_cents;
    to_account.balance_cents += amount_cents;

    // Update accounts.
    account_repo::update(from_account, &mut tx).await?;
    account_repo::update(to_account, &mut tx).await?;

    // Add transaction.
    let transaction =
        transaction_repo::add(from_account_id, to_account_id, amount_cents, &mut tx).await?;

    // Commit transaction.
    tx.commit().await?;

    Ok(transaction)
}
