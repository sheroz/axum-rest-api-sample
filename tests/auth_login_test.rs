use reqwest::StatusCode;
use serial_test::serial;

pub mod common;
use common::{
    auth,
    constants::{TEST_ADMIN_PASSWORD_HASH, TEST_ADMIN_USERNAME},
    route, utils,
};

#[tokio::test]
#[serial]
async fn login_test() {
    // Load the test configuration and start the api server.
    utils::run_app().await;

    // Try unauthorized access to the root handler.
    assert_eq!(
        route::fetch_root("").await.unwrap(),
        StatusCode::UNAUTHORIZED
    );

    let username_wrong = format!("{}1", TEST_ADMIN_USERNAME);
    let (status, _) = auth::login(&username_wrong, TEST_ADMIN_PASSWORD_HASH)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let password_wrong = format!("{}1", TEST_ADMIN_PASSWORD_HASH);
    let (status, _) = auth::login(TEST_ADMIN_USERNAME, &password_wrong)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let (status, _) = auth::login(&username_wrong, &password_wrong).await.unwrap();
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // Login as an admin.
    let (status, result) = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    let (access_token, _) = result.unwrap();

    // Access to the root handler.
    assert_eq!(
        route::fetch_root(&access_token).await.unwrap(),
        StatusCode::OK
    );
}
