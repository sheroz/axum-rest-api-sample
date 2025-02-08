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
async fn access_token_expire_test() {
    // Start API server.
    let test_db = test_app::run().await;

    let config = helpers::config();

    // Assert that revoked options are enabled.
    assert!(config.jwt_enable_revoked_tokens);

    // Login as an admin.
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    // Wait to expire access token.
    tokio::time::sleep(tokio::time::Duration::from_secs(
        (config.jwt_expire_access_token_seconds + config.jwt_validation_leeway_seconds + 1) as u64,
    ))
    .await;

    // Check the access to the root handler with expired token.
    assert_eq!(
        root::fetch_root(&tokens.access_token).await.unwrap(),
        StatusCode::UNAUTHORIZED
    );

    // Refresh tokens.
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    // Try access to the root handler with new token.
    assert_eq!(
        root::fetch_root(&tokens.access_token).await.unwrap(),
        StatusCode::OK
    );

    // Drop test database.
    test_db.drop().await.unwrap();
}

#[tokio::test]
#[serial]
async fn refresh_token_expire_test() {
    // Start API server.
    let test_db = test_app::run().await;

    let config = helpers::config();

    // Assert that revoked options are enabled.
    assert!(config.jwt_enable_revoked_tokens);

    // Login as an admin.
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    // Wait to expire refresh token.
    tokio::time::sleep(tokio::time::Duration::from_secs(
        (config.jwt_expire_refresh_token_seconds + config.jwt_validation_leeway_seconds + 1) as u64,
    ))
    .await;

    // Try to refresh with expired token
    let result = auth::refresh(&tokens.refresh_token).await;
    assert_api_error_status!(result, StatusCode::UNAUTHORIZED);

    // Drop test database.
    test_db.drop().await.unwrap();
}
