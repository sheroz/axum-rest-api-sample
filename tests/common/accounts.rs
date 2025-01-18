use uuid::Uuid;

use axum_web::domain::models::account::Account;

use crate::common::{
    constants::{API_ACCOUNTS_PATH, API_V1},
    utils, GenericResult,
};

pub async fn list(
    access_token: &str,
) -> GenericResult<(reqwest::StatusCode, Option<Vec<Account>>)> {
    let url = utils::build_path(API_V1, API_ACCOUNTS_PATH);

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
        let accounts: Vec<Account> = serde_json::from_str(&body).unwrap();
        return Ok((status, Some(accounts)));
    }
    Ok((status, None))
}

pub async fn get(
    account_id: Uuid,
    access_token: &str,
) -> GenericResult<(reqwest::StatusCode, Option<Account>)> {
    let url = utils::build_url(API_V1, API_ACCOUNTS_PATH, &account_id.to_string());

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
        let account: Account = serde_json::from_str(&body).unwrap();
        return Ok((status, Some(account)));
    }
    Ok((status, None))
}

pub async fn add(
    account: Account,
    access_token: &str,
) -> GenericResult<(reqwest::StatusCode, Option<Account>)> {
    let url = utils::build_path(API_V1, API_ACCOUNTS_PATH);
    let json_param = serde_json::json!(account);
    let authorization = format!("Bearer {}", access_token);
    let response = reqwest::Client::new()
        .post(url.as_str())
        .header("Accept", "application/json")
        .header("Authorization", authorization)
        .json(&json_param)
        .send()
        .await?;

    let status = response.status();
    if status == reqwest::StatusCode::CREATED {
        let body = response.text().await.unwrap();
        let account: Account = serde_json::from_str(&body).unwrap();
        return Ok((status, Some(account)));
    }
    Ok((status, None))
}

pub async fn update(
    account: Account,
    access_token: &str,
) -> GenericResult<(reqwest::StatusCode, Option<Account>)> {
    let url = utils::build_url(API_V1, API_ACCOUNTS_PATH, &account.id.to_string());
    let json_param = serde_json::json!(account);
    let authorization = format!("Bearer {}", access_token);
    let response = reqwest::Client::new()
        .put(url.as_str())
        .header("Accept", "application/json")
        .header("Authorization", authorization)
        .json(&json_param)
        .send()
        .await?;

    let status = response.status();
    if status == reqwest::StatusCode::OK {
        let body = response.text().await.unwrap();
        let account: Account = serde_json::from_str(&body).unwrap();
        return Ok((status, Some(account)));
    }
    Ok((status, None))
}
