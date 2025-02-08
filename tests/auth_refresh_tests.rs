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
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    // Refresh tokens.
    let refreshed = auth::refresh(&tokens.refresh_token)
        .await
        .expect("Auth refresh error.");

    assert_ne!(tokens.access_token, refreshed.access_token);
    assert_ne!(tokens.refresh_token, refreshed.refresh_token);

    // Try access to the root handler with old token.
    assert_eq!(
        root::fetch_root(&tokens.access_token).await.unwrap(),
        StatusCode::UNAUTHORIZED
    );

    // Try access to the root handler with new token.
    assert_eq!(
        root::fetch_root(&refreshed.access_token).await.unwrap(),
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
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    // Refresh tokens.
    let refreshed = auth::refresh(&tokens.refresh_token)
        .await
        .expect("Auth refresh error.");

    // Try logout with old token.
    assert_eq!(
        auth::logout(&tokens.refresh_token).await.unwrap(),
        StatusCode::UNAUTHORIZED
    );

    // Logout with new token.
    assert_eq!(
        auth::logout(&refreshed.refresh_token).await.unwrap(),
        StatusCode::OK
    );

    // Drop test database.
    test_db.drop().await.unwrap();
}
