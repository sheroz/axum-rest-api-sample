use reqwest::StatusCode;
use serial_test::serial;
use uuid::Uuid;

use axum_web::{
    api::{APIErrorCode, APIErrorKind},
    application::security::roles::UserRole,
    domain::models::{account::Account, user::User},
};

pub mod common;
use common::{
    accounts,
    auth::{self, AuthTokens},
    constants::{TEST_ADMIN_PASSWORD_HASH, TEST_ADMIN_USERNAME},
    test_app, users, TestError,
};

// TODO: run account tests in parallel and remove `serial` dependencies.

#[serial]
#[tokio::test]
async fn account_unauthorized_test() {
    // Start api server.
    let test_db = test_app::run().await;

    let account = Account {
        id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        balance_cents: 0,
        created_at: None,
        updated_at: None,
    };

    // Try unauthorized access to account handlers.
    let wrong_access_token = "xyz";
    let result = accounts::get(account.id, wrong_access_token).await;
    assert_api_error_status!(result, StatusCode::UNAUTHORIZED);

    let result = accounts::add(account.clone(), wrong_access_token).await;
    assert_api_error_status!(result, StatusCode::UNAUTHORIZED);

    let result = accounts::update(account.clone(), wrong_access_token).await;
    assert_api_error_status!(result, StatusCode::UNAUTHORIZED);

    // Drop test database.
    test_db.drop().await.unwrap();
}

#[serial]
#[tokio::test]
async fn account_api_error_test() {
    // Start api server.
    let test_db = test_app::run().await;

    // Login as an admin.
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    let AuthTokens {
        access_token,
        refresh_token: _,
    } = tokens;

    // Check for non existing account.
    let account_id = Uuid::new_v4();
    let result = accounts::get(account_id, &access_token).await;

    assert!(result.is_err());

    // We do not have error mapping in accounts::get handler,
    // let's test the default behavior for database-originated errors.
    match result.err().unwrap() {
        TestError::APIError(api_error) => {
            assert_eq!(api_error.status, StatusCode::NOT_FOUND);
            assert_eq!(api_error.errors.len(), 1);

            let error_entry = api_error.errors[0].clone();

            assert_eq!(
                error_entry.code,
                Some(APIErrorCode::ResourceNotFound.to_string())
            );

            assert_eq!(
                error_entry.kind,
                Some(APIErrorKind::ResourceNotFound.to_string())
            );

            // We expect to see raw database error message for non production builds.
            // The message is valid for PostgreSQL database.
            assert!(error_entry.message.contains("no rows returned"));

            // We do not expect the error details exist.
            assert_eq!(error_entry.detail, None);
        }
        _ => panic!("invalid account result"),
    }

    // Drop test database.
    test_db.drop().await.unwrap();
}

#[serial]
#[tokio::test]
async fn account_test() {
    // Start api server.
    let test_db = test_app::run().await;

    // Login as an admin.
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    let AuthTokens {
        access_token,
        refresh_token: _,
    } = tokens;

    // Add a test user.
    let id = Uuid::new_v4();
    let username = "test";
    let user = User {
        id,
        username: format!("test-{}-{}", username, id),
        email: format!("{}-{}@email.com", username, id),
        password_hash: "xyz123".to_string(),
        password_salt: "xyz123".to_string(),
        active: true,
        roles: UserRole::Customer.to_string(),
        created_at: None,
        updated_at: None,
    };

    let _ = users::add(user.clone(), &access_token)
        .await
        .expect("User creation error.");

    // Add account for the user.
    let mut account = Account {
        id: Uuid::new_v4(),
        user_id: user.id,
        balance_cents: 0,
        created_at: None,
        updated_at: None,
    };

    // Test for non existence of account.
    let result = accounts::get(account.id, &access_token).await;
    assert_api_error_status!(result, StatusCode::NOT_FOUND);

    // Add a new account.
    let account_added = accounts::add(account.clone(), &access_token)
        .await
        .expect("Account creation error.");
    assert!(account_added.updated_at.is_some());
    assert!(account_added.created_at.is_some());
    account.created_at = account_added.created_at;
    account.updated_at = account_added.updated_at;
    assert_eq!(account_added, account);

    // Fetch the added account.
    let account_fetched = accounts::get(account.id, &access_token)
        .await
        .expect("Account fetch error.");
    assert_eq!(account_fetched, account_added);

    // list existing accounts.
    let accounts = accounts::list(&access_token)
        .await
        .expect("Fetching the account list error.");
    assert!(accounts.contains(&account));

    // Update account.
    account.balance_cents = 100;
    let account_updated = accounts::update(account.clone(), &access_token)
        .await
        .expect("Account update error.");
    assert_ne!(account_updated.updated_at, account.updated_at);
    account.updated_at = account_updated.updated_at;
    assert_eq!(account_updated, account);

    // Drop test database.
    test_db.drop().await.unwrap();
}
