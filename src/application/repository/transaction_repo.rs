use sqlx::query_as;
use uuid::Uuid;

use crate::{
    application::{repository::RepositoryResult, state::SharedState},
    domain::models::transaction::Transaction,
    infrastructure::types::DatabaseConnection,
};

pub async fn get_by_id(id: Uuid, state: &SharedState) -> RepositoryResult<Transaction> {
    let transaction = sqlx::query_as::<_, Transaction>("SELECT * FROM transactions WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db_pool)
        .await?;

    Ok(transaction)
}

pub async fn add(
    from_account_id: Uuid,
    to_account_id: Uuid,
    amount_cents: i64,
    connection: &mut DatabaseConnection,
) -> RepositoryResult<Transaction> {
    let transaction = query_as::<_, Transaction>(
        r#"INSERT INTO transactions (from_account_id, to_account_id, amount_cents)
         VALUES ($1, $2, $3)
         RETURNING transactions.*"#,
    )
    .bind(from_account_id)
    .bind(to_account_id)
    .bind(amount_cents)
    .fetch_one(connection)
    .await?;

    Ok(transaction)
}
