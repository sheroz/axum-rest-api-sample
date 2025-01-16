use uuid::Uuid;

use axum_web::domain::models::transaction::{Transaction, TransactionResult};

use crate::common::{
    constants::{API_TRANSACTIONS_PATH, API_V1},
    utils, GenericResult,
};

pub async fn get(
    transaction_id: Uuid,
    access_token: &str,
) -> GenericResult<(reqwest::StatusCode, Option<Transaction>)> {
    let url = utils::build_url(API_V1, API_TRANSACTIONS_PATH, &transaction_id.to_string());

    let authorization = format!("Bearer {}", access_token);
    let response = reqwest::Client::new()
        .get(url.as_str())
        .header("Accept", "application/json")
        .header("Authorization", authorization)
        .send()
        .await?;

    let status = response.status();
    if status == reqwest::StatusCode::OK {
        let body = response.text().await.unwrap();
        let transaction: Transaction = serde_json::from_str(&body).unwrap();
        return Ok((status, Some(transaction)));
    }
    Ok((status, None))
}

pub async fn transfer(
    from_account_id: Uuid,
    to_account_id: Uuid,
    amount: i64,
    access_token: &str,
) -> GenericResult<(reqwest::StatusCode, Option<TransactionResult>)> {
    Ok((reqwest::StatusCode::NOT_IMPLEMENTED, None))
}
