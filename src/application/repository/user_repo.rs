use chrono::Utc;
use sqlx::query_as;
use uuid::Uuid;

use crate::{
    application::{repository::RepositoryResult, state::SharedState},
    domain::models::user::User,
};

pub async fn list(state: &SharedState) -> RepositoryResult<Vec<User>> {
    let users = query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&state.db_pool)
        .await?;

    Ok(users)
}

pub async fn add(user: User, state: &SharedState) -> RepositoryResult<User> {
    let time_now = Utc::now().naive_utc();
    tracing::trace!("user: {:#?}", user);
    let user = sqlx::query_as::<_, User>(
        r#"INSERT INTO users (id,
         username,
         email,
         password_hash,
         password_salt,
         active,
         roles,
         created_at,
         updated_at)
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
         RETURNING users.*"#,
    )
    .bind(user.id)
    .bind(user.username)
    .bind(user.email)
    .bind(user.password_hash)
    .bind(user.password_salt)
    .bind(true)
    .bind(user.roles)
    .bind(time_now)
    .bind(time_now)
    .fetch_one(&state.db_pool)
    .await?;

    Ok(user)
}

pub async fn get_by_id(id: Uuid, state: &SharedState) -> RepositoryResult<User> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db_pool)
        .await?;
    Ok(user)
}

pub async fn get_by_username(username: &str, state: &SharedState) -> RepositoryResult<User> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_one(&state.db_pool)
        .await?;

    Ok(user)
}

pub async fn update(user: User, state: &SharedState) -> RepositoryResult<User> {
    tracing::trace!("user: {:#?}", user);
    let time_now = Utc::now().naive_utc();
    let user = sqlx::query_as::<_, User>(
        r#"UPDATE users
         SET 
         username = $1,
         email = $2,
         password_hash = $3,
         password_salt = $4,
         active = $5,
         roles = $6,
         updated_at = $7
         WHERE id = $8
         RETURNING users.*"#,
    )
    .bind(user.username)
    .bind(user.email)
    .bind(user.password_hash)
    .bind(user.password_salt)
    .bind(user.active)
    .bind(user.roles)
    .bind(time_now)
    .bind(user.id)
    .fetch_one(&state.db_pool)
    .await?;

    Ok(user)
}

pub async fn delete(id: Uuid, state: &SharedState) -> RepositoryResult<bool> {
    let query_result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&state.db_pool)
        .await?;

    Ok(query_result.rows_affected() == 1)
}
