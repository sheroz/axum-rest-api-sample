use reqwest::StatusCode;
use serial_test::serial;

use axum_web::application::config;

pub mod common;
use common::{auth, route, utils, *};

#[tokio::test]
#[serial]
async fn refresh_test() {
    // Load the test configuration and start the api server.
    utils::start_api().await;

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
        route::fetch_root(&access_token).await.unwrap(),
        StatusCode::UNAUTHORIZED
    );

    // Try access to the root handler with new token.
    assert_eq!(
        route::fetch_root(&access_token_new).await.unwrap(),
        StatusCode::OK
    );
}

#[tokio::test]
#[serial]
async fn refresh_logout_test() {
    // Load the test configuration and start the api server.
    utils::start_api().await;
    let config = config::get();

    // Assert that revoked options are enabled.
    assert!(config.jwt_enable_revoked_tokens);

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
}
