use uuid::Uuid;

use crate::{
    application::{
        repository::{account_repo, transaction_repo, RepositoryResult},
        state::SharedState,
    },
    domain::models::transaction::TransactionResult,
};

pub async fn transfer(
    from_account_id: Uuid,
    to_account_id: Uuid,
    amount_cents: i64,
    state: &SharedState,
) -> RepositoryResult<TransactionResult> {
    tracing::trace!(
        "transfer: from_account_id: {}, to_account_id: {}, amount_cents: {} ",
        from_account_id,
        to_account_id,
        amount_cents
    );

    // Start transaction.
    let mut tx = state.db_pool.begin().await?;

    let mut from_account = account_repo::get_by_id(from_account_id, &mut tx).await?;
    if from_account.balance_cents < amount_cents {
        return Ok(TransactionResult::InsufficientFunds);
    }

    let mut to_account = account_repo::get_by_id(from_account_id, &mut tx).await?;

    from_account.balance_cents -= amount_cents;
    to_account.balance_cents += amount_cents;

    account_repo::update(from_account, &mut tx).await?;
    account_repo::update(to_account, &mut tx).await?;
    let transaction =
        transaction_repo::add(from_account_id, to_account_id, amount_cents, &mut tx).await?;

    // End transaction.
    tx.commit().await?;

    Ok(TransactionResult::Success(transaction))
}
