use chrono::Utc;
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
    helpers::{self},
    test_app, users,
};

fn test_user() -> User {
    let username = format!("test-{}", Utc::now().timestamp() as usize);
    User {
        id: Uuid::new_v4(),
        username: username.clone(),
        email: format!("{}@email.com", username),
        password_hash: "xyz123".to_string(),
        password_salt: "xyz123".to_string(),
        active: true,
        roles: "guest".to_string(),
        created_at: None,
        updated_at: None,
    }
}

#[tokio::test]
#[serial]
async fn user_unauthorized_test() {
    // Start API server.
    let test_db = test_app::run().await;

    // Try unauthorized access to user handlers.
    let wrong_access_token = "xyz";

    let user = test_user();

    let result = users::get(user.id, wrong_access_token).await;
    assert_api_error_status!(result, StatusCode::UNAUTHORIZED);

    let result = users::add(user.clone(), wrong_access_token).await;
    assert_api_error_status!(result, StatusCode::UNAUTHORIZED);

    let result = users::update(user.clone(), wrong_access_token).await;
    assert_api_error_status!(result, StatusCode::UNAUTHORIZED);

    let result = users::delete(user.id, wrong_access_token).await;
    assert_api_error_status!(result, StatusCode::UNAUTHORIZED);

    // Drop test database.
    test_db.drop().await.unwrap();
}

#[tokio::test]
#[serial]
async fn list_users_test() {
    // Start API server.
    let test_db = test_app::run().await;

    let config = helpers::config();

    // Try unauthorized access to the users handler.
    let result = users::list("xyz").await;
    assert_api_error_status!(result, StatusCode::UNAUTHORIZED);

    // Login as an admin.
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    let access_claims = jwt::decode_token::<AccessClaims>(&tokens.access_token, config).unwrap();
    let user_id: Uuid = access_claims.sub.parse().unwrap();

    // Try authorized access to the users handler.
    let users = users::list(&tokens.access_token)
        .await
        .expect("User list fetch error.");
    assert!(!users.is_empty());
    assert!(users.iter().any(|u| u.id == user_id));

    // Drop test database.
    test_db.drop().await.unwrap();
}

#[tokio::test]
#[serial]
async fn get_user_test() {
    // Start API server.
    let test_db = test_app::run().await;

    let config = helpers::config();

    // Try unauthorized access to the get user handler
    let result = users::get(uuid::Uuid::new_v4(), "").await;
    assert_api_error_status!(result, StatusCode::UNAUTHORIZED);

    // Login as an admin.
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    let access_claims = jwt::decode_token::<AccessClaims>(&tokens.access_token, config).unwrap();
    let user_id = access_claims.sub.parse().unwrap();

    // Get the user.
    let user = users::get(user_id, &tokens.access_token)
        .await
        .expect("User fetch error.");
    assert_eq!(user.id, user_id);

    // Drop test database.
    test_db.drop().await.unwrap();
}

#[tokio::test]
#[serial]
async fn add_get_update_delete_user_test() {
    // Start API server.
    let test_db = test_app::run().await;

    let mut user = test_user();

    // Login as an admin.
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    // Add a user.
    let user_result = users::add(user.clone(), &tokens.access_token)
        .await
        .expect("User creation error.");
    assert!(user_result.updated_at.is_some());
    assert!(user_result.created_at.is_some());

    user.created_at = user_result.created_at;
    user.updated_at = user_result.updated_at;
    assert_eq!(user_result, user);

    // Get the added user.
    let user_result = users::get(user.id, &tokens.access_token)
        .await
        .expect("User fetch error.");
    assert_eq!(user_result, user);

    // Update user.
    user.username = format!("test-{}", chrono::Utc::now().timestamp() as usize);
    let user_result = users::update(user.clone(), &tokens.access_token)
        .await
        .expect("User update error.");
    assert_ne!(user_result.updated_at, user.updated_at);
    user.updated_at = user_result.updated_at;
    assert_eq!(user_result, user);

    // Delete user.
    users::delete(user.id, &tokens.access_token)
        .await
        .expect("User delete error.");

    // Check the user.
    let result = users::get(user.id, &tokens.access_token).await;
    assert_api_error_status!(result, StatusCode::NOT_FOUND);

    // Drop test database.
    test_db.drop().await.unwrap();
}
