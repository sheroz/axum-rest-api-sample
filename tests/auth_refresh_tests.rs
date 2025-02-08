use reqwest::StatusCode;
use serial_test::serial;

pub mod common;
use common::{
    auth,
    constants::{TEST_ADMIN_PASSWORD_HASH, TEST_ADMIN_USERNAME},
    helpers, root, test_app,
};

#[tokio::test]
#[serial]
async fn refresh_test() {
    // Start API server.
    let test_db = test_app::run().await;

    // Login as an admin.
    let (status, result) = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    let (access_token, refresh_token) = result.unwrap();

    // Refresh tokens.
    let (status, result) = auth::refresh(&refresh_token).await.unwrap();
    assert_eq!(status, StatusCode::OK);
    let (access_token_new, refresh_token_new) = result.unwrap();

    assert_ne!(access_token, access_token_new);
    assert_ne!(refresh_token, refresh_token_new);

    // Try access to the root handler with old token.
    assert_eq!(
        root::fetch_root(&access_token).await.unwrap(),
        StatusCode::UNAUTHORIZED
    );

    // Try access to the root handler with new token.
    assert_eq!(
        root::fetch_root(&access_token_new).await.unwrap(),
        StatusCode::OK
    );

    // Drop test database.
    test_db.drop().await.unwrap();
}

#[tokio::test]
#[serial]
async fn refresh_logout_test() {
    // Start API server.
    let test_db = test_app::run().await;

    let config = helpers::config();

    // Assert that revoked options are enabled.
    assert!(config.jwt_enable_revoked_tokens);

    // Login as an admin.
    let (status, result) = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    let (_, refresh_token) = result.unwrap();

    // Refresh tokens.
    let (status, result) = auth::refresh(&refresh_token).await.unwrap();
    assert_eq!(status, StatusCode::OK);
    let (_, refresh_token_new) = result.unwrap();

    // Try logout with old token.
    assert_eq!(
        auth::logout(&refresh_token).await.unwrap(),
        StatusCode::UNAUTHORIZED
    );

    // Logout with new token.
    assert_eq!(
        auth::logout(&refresh_token_new).await.unwrap(),
        StatusCode::OK
    );

    // Drop test database.
    test_db.drop().await.unwrap();
}
