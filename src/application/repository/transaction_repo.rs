use chrono::Utc;
use sqlx::query_as;
use uuid::Uuid;

use crate::{
    application::{repository::RepositoryResult, state::SharedState},
    domain::models::{
        account::Account,
        transaction::{Transaction, TransactionResult},
    },
};

pub async fn get_by_id(id: Uuid, state: &SharedState) -> RepositoryResult<Transaction> {
    let transaction = sqlx::query_as::<_, Transaction>("SELECT * FROM transactions WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pgpool)
        .await?;

    Ok(transaction)
}

pub async fn transfer(
    from_account_id: Uuid,
    to_account_id: Uuid,
    amount_cents: i64,
    state: &SharedState,
) -> RepositoryResult<TransactionResult> {
    // tracing::trace!("account: {:#?}", account);
    // let time_now = Utc::now().naive_utc();
    // let account = sqlx::query_as::<_, Account>(
    //     r#"UPDATE accounts
    //      SET
    //      user_id = $1,
    //      balance_cents = $2,
    //      updated_at = $3
    //      WHERE id = $4
    //      RETURNING accounts.*"#,
    // )
    // .bind(account.user_id)
    // .bind(account.balance_cents)
    // .bind(time_now)
    // .bind(id)
    // .fetch_one(&state.pgpool)
    // .await?;

    Ok(TransactionResult::InsufficientFunds)
}
