use reqwest::StatusCode;
use serial_test::serial;

use axum_web::application::security::jwt::{self, AccessClaims};

pub mod common;
use common::{
    auth,
    constants::{TEST_ADMIN_PASSWORD_HASH, TEST_ADMIN_USERNAME},
    helpers, root, test_app,
};

#[tokio::test]
#[serial]
async fn revoke_user_test() {
    // Start the api server.
    test_app::run().await;

    let config = helpers::config();

    // Assert that revoked options are enabled.
    assert!(config.jwt_enable_revoked_tokens);

    // Login as an admin.
    let (status, result) = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    let (access_token, _) = result.unwrap();

    let access_claims: AccessClaims = jwt::decode_token(&access_token, config).unwrap();
    let user_id = access_claims.sub;

    assert_eq!(
        auth::revoke_user(&access_token, &user_id).await.unwrap(),
        StatusCode::OK
    );

    // Try access to the root handler with the same token again.
    assert_eq!(
        root::fetch_root(&access_token).await.unwrap(),
        StatusCode::UNAUTHORIZED
    );

    // Needs pause to pass authentication of next logins.
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}

#[tokio::test]
#[serial]
async fn revoke_all_test() {
    // Start the api server.
    test_app::run().await;

    let config = helpers::config();

    // Assert that revoked options are enabled.
    assert!(config.jwt_enable_revoked_tokens);

    // Login as an admin.
    let (status, result) = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    let (access_token, _) = result.unwrap();

    auth::revoke_all(&access_token).await.unwrap();

    // Try access to the root handler with the same token again.
    assert_eq!(
        root::fetch_root(&access_token).await.unwrap(),
        StatusCode::UNAUTHORIZED
    );

    // Needs pause to pass authentication of next logins.
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}

#[tokio::test]
#[serial]
async fn cleanup_test() {
    // Start the api server.
    test_app::run().await;

    let config = helpers::config();

    // Assert that revoked options are enabled.
    assert!(config.jwt_enable_revoked_tokens);

    // Login as an admin.
    let (status, result) = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    let (access_token, refresh_token) = result.unwrap();

    let _initial_cleanup = auth::cleanup(&access_token).await.unwrap();

    // Expected 2 tokens to expire after resfresh.
    let (status, result) = auth::refresh(&refresh_token).await.unwrap();
    assert_eq!(status, StatusCode::OK);
    let (_, refresh_token) = result.unwrap();

    // Expected 2 tokens to expire after logout.
    assert_eq!(auth::logout(&refresh_token).await.unwrap(), StatusCode::OK);

    // Wait to make sure that tokens expire.
    tokio::time::sleep(tokio::time::Duration::from_secs(
        (config.jwt_expire_access_token_seconds + config.jwt_validation_leeway_seconds) as u64,
    ))
    .await;
    tokio::time::sleep(tokio::time::Duration::from_secs(
        (config.jwt_expire_refresh_token_seconds + config.jwt_validation_leeway_seconds) as u64,
    ))
    .await;

    let (status, result) = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    let (access_token, _) = result.unwrap();

    let deleted_tokens = auth::cleanup(&access_token).await.unwrap();
    assert!(deleted_tokens >= 4);
}
