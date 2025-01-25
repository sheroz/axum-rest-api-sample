use uuid::Uuid;

use crate::{
    api::transactions::{TransactionError, TransferValidationErrors},
    application::{
        api_error::ApiError,
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
) -> Result<Transaction, ApiError> {
    tracing::trace!(
        "transfer: source_account_id: {}, destination_account_id: {}, amount_cents: {} ",
        source_account_id,
        destination_account_id,
        amount_cents
    );

    // Start transaction.
    let mut tx = state.db_pool.begin().await?;

    let mut validation_errors = TransferValidationErrors::default();

    // Find the source account.
    let mut source_account = match account_repo::get_by_id(source_account_id, &mut tx).await {
        Ok(account) => Some(account),
        Err(e) => {
            let error = match e {
                sqlx::Error::RowNotFound => {
                    TransactionError::SourceAccountNotFound(source_account_id)
                }
                _ => Err(e)?,
            };
            validation_errors.add(error);
            None
        }
    };

    // Find the destination account.
    let mut destination_account =
        match account_repo::get_by_id(destination_account_id, &mut tx).await {
            Ok(account) => Some(account),
            Err(e) => {
                let error = match e {
                    sqlx::Error::RowNotFound => {
                        TransactionError::DestinationAccountNotFound(destination_account_id)
                    }
                    _ => Err(e)?,
                };
                validation_errors.add(error);
                None
            }
        };

    if validation_errors.exists() {
        Err(validation_errors)?
    }

    let mut source_account = source_account.take().unwrap();
    // Check the source balance.
    if source_account.balance_cents < amount_cents {
        Err(TransactionError::InsufficientFunds)?
    }

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
