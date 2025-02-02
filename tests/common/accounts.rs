use uuid::Uuid;

use axum_web::domain::models::account::Account;
use reqwest::StatusCode;

use crate::common::{
    constants::{API_PATH_ACCOUNTS, API_V1},
    helpers, TestResult,
};

pub async fn list(access_token: &str) -> TestResult<Vec<Account>> {
    let url = helpers::build_path(API_V1, API_PATH_ACCOUNTS);

    let authorization = format!("Bearer {}", access_token);
    let response = reqwest::Client::new()
        .get(url.as_str())
        .header("Accept", "application/json")
        .header("Authorization", authorization)
        .send()
        .await?;

    helpers::dispatch_reqwest_response::<Vec<Account>>(response, StatusCode::OK)
        .await
        .map(|v| v.unwrap())
}

pub async fn get(account_id: Uuid, access_token: &str) -> TestResult<Account> {
    let url = helpers::build_url(API_V1, API_PATH_ACCOUNTS, &account_id.to_string());

    let authorization = format!("Bearer {}", access_token);
    let response = reqwest::Client::new()
        .get(url.as_str())
        .header("Accept", "application/json")
        .header("Authorization", authorization)
        .send()
        .await?;

    helpers::dispatch_reqwest_response::<Account>(response, StatusCode::OK)
        .await
        .map(|v| v.unwrap())
}

pub async fn add(account: Account, access_token: &str) -> TestResult<Account> {
    let url = helpers::build_path(API_V1, API_PATH_ACCOUNTS);
    let json_param = serde_json::json!(account);
    let authorization = format!("Bearer {}", access_token);
    let response = reqwest::Client::new()
        .post(url.as_str())
        .header("Accept", "application/json")
        .header("Authorization", authorization)
        .json(&json_param)
        .send()
        .await?;

    helpers::dispatch_reqwest_response::<Account>(response, StatusCode::CREATED)
        .await
        .map(|v| v.unwrap())
}

pub async fn update(account: Account, access_token: &str) -> TestResult<Account> {
    let url = helpers::build_url(API_V1, API_PATH_ACCOUNTS, &account.id.to_string());
    let json_param = serde_json::json!(account);
    let authorization = format!("Bearer {}", access_token);
    let response = reqwest::Client::new()
        .put(url.as_str())
        .header("Accept", "application/json")
        .header("Authorization", authorization)
        .json(&json_param)
        .send()
        .await?;

    helpers::dispatch_reqwest_response::<Account>(response, StatusCode::OK)
        .await
        .map(|v| v.unwrap())
}
