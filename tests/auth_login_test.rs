use reqwest::StatusCode;
use serial_test::serial;

pub mod common;
use common::{
    auth,
    constants::{TEST_ADMIN_PASSWORD_HASH, TEST_ADMIN_USERNAME},
    root, test_app,
};

#[tokio::test]
#[serial]
async fn login_test() {
    // Start API server.
    let test_db = test_app::run().await;

    // Try unauthorized access to the root handler.
    assert_eq!(
        root::fetch_root("").await.unwrap(),
        StatusCode::UNAUTHORIZED
    );

    let username_wrong = format!("{}1", TEST_ADMIN_USERNAME);
    let result = auth::login(&username_wrong, TEST_ADMIN_PASSWORD_HASH).await;
    assert_api_error_status!(result, StatusCode::UNAUTHORIZED);

    let password_wrong = format!("{}1", TEST_ADMIN_PASSWORD_HASH);
    let result = auth::login(TEST_ADMIN_USERNAME, &password_wrong).await;
    assert_api_error_status!(result, StatusCode::UNAUTHORIZED);

    let result = auth::login(&username_wrong, &password_wrong).await;
    assert_api_error_status!(result, StatusCode::UNAUTHORIZED);

    // Login as an admin.
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    // Access to the root handler.
    assert_eq!(
        root::fetch_root(&tokens.access_token).await.unwrap(),
        StatusCode::OK
    );

    // Drop test database.
    test_db.drop().await.unwrap();
}
