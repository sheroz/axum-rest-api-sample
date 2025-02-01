use serial_test::serial;
use uuid::Uuid;

use axum_web::{
    application::security::jwt::{self, AccessClaims},
    domain::models::user::User,
};
use reqwest::StatusCode;

pub mod common;
use common::{
    auth,
    constants::{TEST_ADMIN_PASSWORD_HASH, TEST_ADMIN_USERNAME},
    helpers, test_app, users,
};

#[tokio::test]
#[serial]
async fn list_users_test() {
    // Start the api server.
    test_app::run().await;

    let config = helpers::config();

    // Try unauthorized access to the users handler.
    let (status, _) = users::list("xyz").await.unwrap();
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // Login as an admin.
    let (status, result) = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    let (access_token, _) = result.unwrap();

    let access_claims = jwt::decode_token::<AccessClaims>(&access_token, config).unwrap();
    let user_id: Uuid = access_claims.sub.parse().unwrap();

    // Try authorized access to the users handler.
    let (status, result) = users::list(&access_token).await.unwrap();
    assert_eq!(status, reqwest::StatusCode::OK);
    assert!(result.is_some());

    let users = result.unwrap();
    assert!(!users.is_empty());
    assert!(users.iter().any(|u| u.id == user_id));
}

#[tokio::test]
#[serial]
async fn get_user_test() {
    // Start the api server.
    test_app::run().await;

    let config = helpers::config();

    // Try unauthorized access to the get user handler
    let (status, _) = users::get(uuid::Uuid::new_v4(), "").await.unwrap();
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // Login as an admin.
    let (status, result) = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    let (access_token, _) = result.unwrap();

    let access_claims = jwt::decode_token::<AccessClaims>(&access_token, config).unwrap();
    let user_id = access_claims.sub.parse().unwrap();

    // Get the user.
    let (status, result) = users::get(user_id, &access_token).await.unwrap();
    assert_eq!(status, reqwest::StatusCode::OK);
    assert!(result.is_some());

    let user = result.unwrap();
    assert_eq!(user.id, user_id);
}

#[tokio::test]
#[serial]
async fn add_get_update_delete_user_test() {
    // Start the api server.
    test_app::run().await;

    let username = format!("test-{}", chrono::Utc::now().timestamp() as usize);
    let mut user = User {
        id: Uuid::new_v4(),
        username: username.clone(),
        email: format!("{}@email.com", username),
        password_hash: "xyz123".to_string(),
        password_salt: "xyz123".to_string(),
        active: true,
        roles: "guest".to_string(),
        created_at: None,
        updated_at: None,
    };

    // Try unauthorized access to user handlers.
    let access_token = "xyz".to_string();
    let (status, _) = users::get(user.id, &access_token).await.unwrap();
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let (status, _) = users::add(user.clone(), &access_token).await.unwrap();
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let (status, _) = users::update(user.clone(), &access_token).await.unwrap();
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let status = users::delete(user.id, &access_token).await.unwrap();
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // Login as an admin.
    let (status, result) = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    let (access_token, _) = result.unwrap();

    // Add a user.
    let (status, result) = users::add(user.clone(), &access_token).await.unwrap();
    assert_eq!(status, StatusCode::CREATED);
    assert!(result.is_some());
    let user_result = result.unwrap();
    assert!(user_result.updated_at.is_some());
    assert!(user_result.created_at.is_some());

    user.created_at = user_result.created_at;
    user.updated_at = user_result.updated_at;
    assert_eq!(user_result, user);

    // Get the added user.
    let (status, result) = users::get(user.id, &access_token).await.unwrap();
    assert_eq!(status, reqwest::StatusCode::OK);
    assert!(result.is_some());
    let user_result = result.unwrap();
    assert_eq!(user_result, user);

    // Update user.
    user.username = format!("test-{}", chrono::Utc::now().timestamp() as usize);
    let (status, result) = users::update(user.clone(), &access_token).await.unwrap();
    assert_eq!(status, reqwest::StatusCode::OK);
    assert!(result.is_some());
    let user_result = result.unwrap();
    assert_ne!(user_result.updated_at, user.updated_at);
    user.updated_at = user_result.updated_at;
    assert_eq!(user_result, user);

    // Delete user.
    let status = users::delete(user.id, &access_token).await.unwrap();
    assert_eq!(status, StatusCode::OK);

    // Check the user.
    let (status, _) = users::get(user.id, &access_token).await.unwrap();
    assert_eq!(status, reqwest::StatusCode::NOT_FOUND);
}
