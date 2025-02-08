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
async fn logout_test() {
    // Start API server.
    let test_db = test_app::run().await;

    let config = helpers::config();

    // Assert that revoked options are enabled.
    assert!(config.jwt_enable_revoked_tokens);

    // Try unauthorized access to the root handler.
    assert_eq!(
        root::fetch_root("").await.unwrap(),
        StatusCode::UNAUTHORIZED
    );

    // Login as an admin.
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    // Access to the root handler.
    assert_eq!(
        root::fetch_root(&tokens.access_token).await.unwrap(),
        StatusCode::OK
    );

    // Logout.
    assert_eq!(
        auth::logout(&tokens.refresh_token).await.unwrap(),
        StatusCode::OK
    );

    // Try access to the root handler after logout.
    assert_eq!(
        root::fetch_root(&tokens.access_token).await.unwrap(),
        StatusCode::UNAUTHORIZED
    );

    // Drop test database.
    test_db.drop().await.unwrap();
}
