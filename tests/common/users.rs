use axum_web::domain::models::user::User;
use reqwest::StatusCode;
use uuid::Uuid;

use crate::common::{
    TestResult,
    constants::{API_PATH_USERS, API_V1},
    helpers,
};

pub async fn list(access_token: &str) -> TestResult<Vec<User>> {
    let url = helpers::build_path(API_V1, API_PATH_USERS);

    let authorization = format!("Bearer {}", access_token);
    let response = reqwest::Client::new()
        .get(url.as_str())
        .header("Accept", "application/json")
        .header("Authorization", authorization)
        .send()
        .await?;

    helpers::dispatch_reqwest_response::<Vec<User>>(response, StatusCode::OK)
        .await
        .map(|v| v.unwrap())
}

pub async fn get(user_id: Uuid, access_token: &str) -> TestResult<User> {
    let url = helpers::build_url(API_V1, API_PATH_USERS, &user_id.to_string());

    let authorization = format!("Bearer {}", access_token);
    let response = reqwest::Client::new()
        .get(url.as_str())
        .header("Accept", "application/json")
        .header("Authorization", authorization)
        .send()
        .await?;

    helpers::dispatch_reqwest_response::<User>(response, StatusCode::OK)
        .await
        .map(|v| v.unwrap())
}

pub async fn add(user: User, access_token: &str) -> TestResult<User> {
    let url = helpers::build_path(API_V1, API_PATH_USERS);
    let json_param = serde_json::json!(user);
    let authorization = format!("Bearer {}", access_token);
    let response = reqwest::Client::new()
        .post(url.as_str())
        .header("Accept", "application/json")
        .header("Authorization", authorization)
        .json(&json_param)
        .send()
        .await?;

    helpers::dispatch_reqwest_response::<User>(response, StatusCode::CREATED)
        .await
        .map(|v| v.unwrap())
}

pub async fn update(user: User, access_token: &str) -> TestResult<User> {
    let url = helpers::build_url(API_V1, API_PATH_USERS, &user.id.to_string());
    let json_param = serde_json::json!(user);
    let authorization = format!("Bearer {}", access_token);
    let response = reqwest::Client::new()
        .put(url.as_str())
        .header("Accept", "application/json")
        .header("Authorization", authorization)
        .json(&json_param)
        .send()
        .await?;

    helpers::dispatch_reqwest_response::<User>(response, StatusCode::OK)
        .await
        .map(|v| v.unwrap())
}

pub async fn delete(user_id: Uuid, access_token: &str) -> TestResult<()> {
    let url = helpers::build_url(API_V1, API_PATH_USERS, &user_id.to_string());
    let authorization = format!("Bearer {}", access_token);
    let response = reqwest::Client::new()
        .delete(url.as_str())
        .header("Accept", "application/json")
        .header("Authorization", authorization)
        .send()
        .await?;

    helpers::dispatch_reqwest_response::<String>(response, StatusCode::OK).await?;
    Ok(())
}
