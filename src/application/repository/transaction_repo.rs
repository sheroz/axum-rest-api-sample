use sqlx::query_as;
use uuid::Uuid;

use crate::{
    application::{repository::RepositoryResult, state::SharedState},
    domain::models::transaction::Transaction,
    infrastructure::database::DatabaseConnection,
};

pub async fn get_by_id(id: Uuid, state: &SharedState) -> RepositoryResult<Transaction> {
    let transaction = sqlx::query_as::<_, Transaction>("SELECT * FROM transactions WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db_pool)
        .await?;

    Ok(transaction)
}

pub async fn add(
    source_account_id: Uuid,
    destination_account_id: Uuid,
    amount_cents: i64,
    connection: &mut DatabaseConnection,
) -> RepositoryResult<Transaction> {
    let transaction = query_as::<_, Transaction>(
        r#"INSERT INTO transactions (source_account_id, destination_account_id, amount_cents)
         VALUES ($1, $2, $3)
         RETURNING transactions.*"#,
    )
    .bind(source_account_id)
    .bind(destination_account_id)
    .bind(amount_cents)
    .fetch_one(connection)
    .await?;

    Ok(transaction)
}
