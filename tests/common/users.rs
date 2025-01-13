use axum_web::domain::models::user::User;
use uuid::Uuid;

use crate::common::{
    constants::{API_USERS_PATH, API_V1},
    utils, GenericResult,
};

pub async fn list(access_token: &str) -> GenericResult<(reqwest::StatusCode, Option<Vec<User>>)> {
    let url = utils::build_path(API_V1, API_USERS_PATH);

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
        let users: Vec<User> = serde_json::from_str(&body).unwrap();
        return Ok((status, Some(users)));
    }
    Ok((status, None))
}

pub async fn get(
    user_id: Uuid,
    access_token: &str,
) -> GenericResult<(reqwest::StatusCode, Option<User>)> {
    let url = utils::build_url(API_V1, API_USERS_PATH, &user_id.to_string());

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
        let user: User = serde_json::from_str(&body).unwrap();
        return Ok((status, Some(user)));
    }
    Ok((status, None))
}

pub async fn add(
    user: User,
    access_token: &str,
) -> GenericResult<(reqwest::StatusCode, Option<User>)> {
    let url = utils::build_path(API_V1, API_USERS_PATH);
    let json_param = serde_json::json!(user);
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
        let user: User = serde_json::from_str(&body).unwrap();
        return Ok((status, Some(user)));
    }
    Ok((status, None))
}

pub async fn update(
    user: User,
    access_token: &str,
) -> GenericResult<(reqwest::StatusCode, Option<User>)> {
    let url = utils::build_url(API_V1, API_USERS_PATH, &user.id.to_string());
    let json_param = serde_json::json!(user);
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
        let user: User = serde_json::from_str(&body).unwrap();
        return Ok((status, Some(user)));
    }
    Ok((status, None))
}

pub async fn delete(user_id: Uuid, access_token: &str) -> GenericResult<reqwest::StatusCode> {
    let url = utils::build_url(API_V1, API_USERS_PATH, &user_id.to_string());
    let authorization = format!("Bearer {}", access_token);
    let response = reqwest::Client::new()
        .delete(url.as_str())
        .header("Accept", "application/json")
        .header("Authorization", authorization)
        .send()
        .await?;
    Ok(response.status())
}
