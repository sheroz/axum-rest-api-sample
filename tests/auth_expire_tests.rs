use reqwest::StatusCode;
use serial_test::serial;

use axum_web::application::config;

pub mod common;
use common::{
    auth,
    constants::{TEST_ADMIN_PASSWORD_HASH, TEST_ADMIN_USERNAME},
    route, utils,
};

#[tokio::test]
#[serial]
async fn access_token_expire_test() {
    // Load the test configuration and start the api server.
    utils::run_app().await;
    let config = utils::config();

    // Assert that revoked options are enabled.
    assert!(config.jwt_enable_revoked_tokens);

    // Login as an admin.
    let (status, result) = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    let (access_token, refresh_token) = result.unwrap();

    // Wait to expire access token.
    tokio::time::sleep(tokio::time::Duration::from_secs(
        (config.jwt_expire_access_token_seconds + config.jwt_validation_leeway_seconds + 1) as u64,
    ))
    .await;

    // Check the access to the root handler with expired token.
    assert_eq!(
        route::fetch_root(&access_token).await.unwrap(),
        StatusCode::UNAUTHORIZED
    );

    // Refresh tokens.
    let (status, result) = auth::refresh(&refresh_token).await.unwrap();
    assert_eq!(status, StatusCode::OK);
    let (access_token_new, _) = result.unwrap();

    // Try access to the root handler with new token.
    assert_eq!(
        route::fetch_root(&access_token_new).await.unwrap(),
        StatusCode::OK
    );
}

#[tokio::test]
#[serial]
async fn refresh_token_expire_test() {
    // Load the test configuration and start the api server.
    utils::run_app().await;
    let config = config::load();

    // Assert that revoked options are enabled.
    assert!(config.jwt_enable_revoked_tokens);

    // Login as an admin.
    let (status, result) = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    let (_, refresh_token) = result.unwrap();

    // Wait to expire refresh token.
    tokio::time::sleep(tokio::time::Duration::from_secs(
        (config.jwt_expire_refresh_token_seconds + config.jwt_validation_leeway_seconds + 1) as u64,
    ))
    .await;

    // Try to refresh with expired token
    let (status, _) = auth::refresh(&refresh_token).await.unwrap();
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}
