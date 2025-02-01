use chrono::Utc;
use sqlx::query_as;
use uuid::Uuid;

use crate::{
    application::repository::RepositoryResult, domain::models::account::Account,
    infrastructure::database::DatabaseConnection,
};

pub async fn list(connection: &mut DatabaseConnection) -> RepositoryResult<Vec<Account>> {
    let accounts = query_as::<_, Account>("SELECT * FROM accounts")
        .fetch_all(connection)
        .await?;

    Ok(accounts)
}

pub async fn add(
    account: Account,
    connection: &mut DatabaseConnection,
) -> RepositoryResult<Account> {
    let time_now = Utc::now().naive_utc();
    tracing::trace!("account: {:#?}", account);
    let account = sqlx::query_as::<_, Account>(
        r#"INSERT INTO accounts (id,
         user_id,
         balance_cents,
         created_at,
         updated_at)
         VALUES ($1,$2,$3,$4,$5)
         RETURNING accounts.*"#,
    )
    .bind(account.id)
    .bind(account.user_id)
    .bind(account.balance_cents)
    .bind(time_now)
    .bind(time_now)
    .fetch_one(connection)
    .await?;

    Ok(account)
}

pub async fn get_by_id(id: Uuid, connection: &mut DatabaseConnection) -> RepositoryResult<Account> {
    let account = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE id = $1")
        .bind(id)
        .fetch_one(connection)
        .await?;

    Ok(account)
}

pub async fn get_by_user_id(
    user_id: Uuid,
    connection: &mut DatabaseConnection,
) -> RepositoryResult<Vec<Account>> {
    let accounts = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(connection)
        .await?;

    Ok(accounts)
}

pub async fn update(
    account: Account,
    connection: &mut DatabaseConnection,
) -> RepositoryResult<Account> {
    tracing::trace!("account: {:#?}", account);
    let time_now = Utc::now().naive_utc();
    let account = sqlx::query_as::<_, Account>(
        r#"UPDATE accounts
         SET
         user_id = $1,
         balance_cents = $2,
         updated_at = $3
         WHERE id = $4
         RETURNING accounts.*"#,
    )
    .bind(account.user_id)
    .bind(account.balance_cents)
    .bind(time_now)
    .bind(account.id)
    .fetch_one(connection)
    .await?;

    Ok(account)
}

pub async fn delete(id: Uuid, connection: &mut DatabaseConnection) -> RepositoryResult<bool> {
    let query_result = sqlx::query("DELETE FROM accounts WHERE id = $1")
        .bind(id)
        .execute(connection)
        .await?;

    Ok(query_result.rows_affected() == 1)
}
