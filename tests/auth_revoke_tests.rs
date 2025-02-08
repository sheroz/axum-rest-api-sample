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
    // Start API server.
    let test_db = test_app::run().await;

    let config = helpers::config();

    // Assert that revoked options are enabled.
    assert!(config.jwt_enable_revoked_tokens);

    // Login as an admin.
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    let access_claims: AccessClaims = jwt::decode_token(&tokens.access_token, config).unwrap();
    let user_id = access_claims.sub;

    assert_eq!(
        auth::revoke_user(&tokens.access_token, &user_id)
            .await
            .unwrap(),
        StatusCode::OK
    );

    // Try access to the root handler with the same token again.
    assert_eq!(
        root::fetch_root(&tokens.access_token).await.unwrap(),
        StatusCode::UNAUTHORIZED
    );

    // Currently, timestamps in claims are defined as the number of seconds since Epoch (RFC 7519).
    // We need to pause for one second so as not to interfere with the authentication of the next logins.
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Drop test database.
    test_db.drop().await.unwrap();
}

#[tokio::test]
#[serial]
async fn revoke_all_test() {
    // Start API server.
    let test_db = test_app::run().await;

    let config = helpers::config();

    // Assert that revoked options are enabled.
    assert!(config.jwt_enable_revoked_tokens);

    // Login as an admin.
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    auth::revoke_all(&tokens.access_token).await.unwrap();

    // Try access to the root handler with the same token again.
    assert_eq!(
        root::fetch_root(&tokens.access_token).await.unwrap(),
        StatusCode::UNAUTHORIZED
    );

    // Currently, timestamps in claims are defined as the number of seconds since Epoch (RFC 7519).
    // We need to pause for one second so as not to interfere with the authentication of the next logins.
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Drop test database.
    test_db.drop().await.unwrap();
}

#[tokio::test]
#[serial]
async fn cleanup_test() {
    // Start API server.
    let test_db = test_app::run().await;

    let config = helpers::config();

    // Assert that revoked options are enabled.
    assert!(config.jwt_enable_revoked_tokens);

    // Login as an admin.
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    let _initial_cleanup = auth::cleanup(&tokens.access_token)
        .await
        .expect("Auth cleanup error.");

    // Expected 2 tokens to expire after resfresh.
    let refreshed = auth::refresh(&tokens.refresh_token)
        .await
        .expect("Auth refresh error.");

    // Expected 2 tokens to expire after logout.
    assert_eq!(
        auth::logout(&refreshed.refresh_token).await.unwrap(),
        StatusCode::OK
    );

    // Wait to make sure that tokens expire.
    tokio::time::sleep(tokio::time::Duration::from_secs(
        (config.jwt_expire_access_token_seconds + config.jwt_validation_leeway_seconds) as u64,
    ))
    .await;
    tokio::time::sleep(tokio::time::Duration::from_secs(
        (config.jwt_expire_refresh_token_seconds + config.jwt_validation_leeway_seconds) as u64,
    ))
    .await;

    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    let deleted_tokens = auth::cleanup(&tokens.access_token).await.unwrap();
    assert!(deleted_tokens >= 4);

    // Drop test database.
    test_db.drop().await.unwrap();
}
