use reqwest::Response;
use thiserror::Error;
use uuid::Uuid;

use axum_web::{api::handlers::transaction_handlers::TransferOrder, domain::models::transaction::Transaction};

use crate::common::{
    constants::{API_TRANSACTIONS_PATH, API_V1},
    utils,
};

#[derive(Debug, Error)]
pub enum TransactionResponseError {
    #[error("unexpected response")]
    UnexpectedResponse(Response),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

pub async fn get(
    transaction_id: Uuid,
    access_token: &str,
) -> Result<Transaction, TransactionResponseError> {
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
        return Ok(transaction);
    }

    Err(TransactionResponseError::UnexpectedResponse(response))
}

pub async fn transfer(
    source_account_id: Uuid,
    destination_account_id: Uuid,
    amount_cents: i64,
    access_token: &str,
) -> Result<Transaction, TransactionResponseError> {
    let url = utils::build_url(API_V1, API_TRANSACTIONS_PATH, "transfer");

    let transfer_order = TransferOrder {
        source_account_id,
        destination_account_id,
        amount_cents,
    };

    let json_param = serde_json::json!(transfer_order);
    let authorization = format!("Bearer {}", access_token);
    let response = reqwest::Client::new()
        .post(url.as_str())
        .header("Accept", "application/json")
        .header("Authorization", authorization)
        .json(&json_param)
        .send()
        .await?;

    let status = response.status();
    if status == reqwest::StatusCode::OK {
        let body = response.text().await.unwrap();
        let transaction: Transaction = serde_json::from_str(&body).unwrap();
        return Ok(transaction);
    }

    Err(TransactionResponseError::UnexpectedResponse(response))
}
