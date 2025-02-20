use reqwest::StatusCode;
use serde::Deserialize;

use crate::common::{
    TestResult,
    constants::{API_PATH_AUTH, API_V1},
    helpers,
};

#[derive(Debug, Deserialize)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn login(username: &str, password_hash: &str) -> TestResult<AuthTokens> {
    let url = helpers::build_url(API_V1, API_PATH_AUTH, "login");

    let params = format!(
        "{{\"username\":\"{}\", \"password_hash\":\"{}\"}}",
        username, password_hash
    );

    let response = reqwest::Client::new()
        .post(url.as_str())
        .header("Accept", "application/json")
        .header("Content-type", "application/json; charset=utf8")
        .body(params)
        .send()
        .await?;

    helpers::dispatch_reqwest_response::<AuthTokens>(response, StatusCode::OK)
        .await
        .map(|v| v.unwrap())
}

pub async fn refresh(refresh_token: &str) -> TestResult<AuthTokens> {
    let url = helpers::build_url(API_V1, API_PATH_AUTH, "refresh");

    let authorization = format!("Bearer {}", refresh_token);
    let response = reqwest::Client::new()
        .post(url.as_str())
        .header("Accept", "application/json")
        .header("Authorization", authorization)
        .send()
        .await?;

    helpers::dispatch_reqwest_response::<AuthTokens>(response, StatusCode::OK)
        .await
        .map(|v| v.unwrap())
}

pub async fn logout(refresh_token: &str) -> TestResult<StatusCode> {
    let url = helpers::build_url(API_V1, API_PATH_AUTH, "logout");

    let authorization = format!("Bearer {}", refresh_token);
    let response = reqwest::Client::new()
        .post(url.as_str())
        .header("Accept", "application/json")
        .header("Authorization", authorization)
        .send()
        .await?;

    Ok(response.status())
}

pub async fn revoke_all(access_token: &str) -> TestResult<StatusCode> {
    let url = helpers::build_url(API_V1, API_PATH_AUTH, "revoke-all");
    let authorization = format!("Bearer {}", access_token);
    let response = reqwest::Client::new()
        .post(url.as_str())
        .header("Accept", "application/json")
        .header("Authorization", authorization)
        .send()
        .await?;
    Ok(response.status())
}

pub async fn revoke_user(access_token: &str, user_id: &str) -> TestResult<StatusCode> {
    let url = helpers::build_url(API_V1, API_PATH_AUTH, "revoke-user");
    let params = format!("{{\"user_id\":\"{}\"}}", user_id);
    let authorization = format!("Bearer {}", access_token);
    let response = reqwest::Client::new()
        .post(url.as_str())
        .header("Accept", "application/json")
        .header("Content-type", "application/json; charset=utf8")
        .header("Authorization", authorization)
        .body(params)
        .send()
        .await?;
    Ok(response.status())
}

pub async fn cleanup(access_token: &str) -> TestResult<u64> {
    let url = helpers::build_url(API_V1, API_PATH_AUTH, "cleanup");
    let authorization = format!("Bearer {}", access_token);
    let response = reqwest::Client::new()
        .post(url.as_str())
        .header("Accept", "application/json")
        .header("Authorization", authorization)
        .send()
        .await?;

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let json: serde_json::Value = response.json().await.unwrap();
    let deleted_tokens = json["deleted_tokens"].as_u64().unwrap();

    Ok(deleted_tokens)
}
