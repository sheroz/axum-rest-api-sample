use reqwest::StatusCode;
use uuid::Uuid;

use axum_web::{
    api::handlers::transaction_handlers::TransferOrder, domain::models::transaction::Transaction,
};

use crate::common::{
    TestResult,
    constants::{API_PATH_TRANSACTIONS, API_V1},
    helpers,
};

pub async fn get(transaction_id: Uuid, access_token: &str) -> TestResult<Transaction> {
    let url = helpers::build_url(API_V1, API_PATH_TRANSACTIONS, &transaction_id.to_string());

    let authorization = format!("Bearer {}", access_token);
    let response = reqwest::Client::new()
        .get(url.as_str())
        .header("Accept", "application/json")
        .header("Authorization", authorization)
        .send()
        .await?;

    helpers::dispatch_reqwest_response::<Transaction>(response, StatusCode::OK)
        .await
        .map(|v| v.unwrap())
}

pub async fn transfer(
    source_account_id: Uuid,
    destination_account_id: Uuid,
    amount_cents: i64,
    access_token: &str,
) -> TestResult<Transaction> {
    let url = helpers::build_url(API_V1, API_PATH_TRANSACTIONS, "transfer");

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

    helpers::dispatch_reqwest_response::<Transaction>(response, StatusCode::OK)
        .await
        .map(|v| v.unwrap())
}
