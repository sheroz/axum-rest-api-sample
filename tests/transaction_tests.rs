use reqwest::StatusCode;
use serial_test::serial;
use uuid::Uuid;

use axum_web::{
    api::{handlers::transaction_handlers::TransactionError, APIErrorCode, APIErrorKind},
    application::{
        security::roles::UserRole, service::transaction_service::TransferValidationError,
    },
    domain::models::{account::Account, user::User},
};

pub mod common;
use common::{
    accounts,
    auth::{self, AuthTokens},
    constants::{TEST_ADMIN_PASSWORD_HASH, TEST_ADMIN_USERNAME},
    test_app, transactions, users, TestError,
};

// TODO: run transaction tests in parallel and remove `serial` dependencies.

// Prepare accounts for Alice and Bob.
async fn prepare_accounts(tokens: &AuthTokens) -> (Account, Account) {
    // Add user for Alice.
    let id = Uuid::new_v4();
    let username = "alice";
    let user_alice = User {
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

    let _ = users::add(user_alice.clone(), &tokens.access_token)
        .await
        .expect("User creation error.");

    // Add account for Alice.
    let account_alice = Account {
        id: Uuid::new_v4(),
        user_id: user_alice.id,
        balance_cents: 100,
        created_at: None,
        updated_at: None,
    };

    let account_alice = accounts::add(account_alice.clone(), &tokens.access_token)
        .await
        .expect("Account creation error.");

    // Add user for Bob.
    let id = Uuid::new_v4();
    let username = "bob";
    let user_bob = User {
        id: Uuid::new_v4(),
        username: format!("test-{}-{}", username, id),
        email: format!("{}-{}@email.com", username, id),
        password_hash: "xyz123".to_string(),
        password_salt: "xyz123".to_string(),
        active: true,
        roles: UserRole::Customer.to_string(),
        created_at: None,
        updated_at: None,
    };

    let _ = users::add(user_bob.clone(), &tokens.access_token)
        .await
        .expect("User creation error.");

    // Add account for Bob.
    let account_bob = Account {
        id: Uuid::new_v4(),
        user_id: user_bob.id,
        balance_cents: 100,
        created_at: None,
        updated_at: None,
    };

    let account_bob = accounts::add(account_bob.clone(), &tokens.access_token)
        .await
        .expect("Account creation error.");

    (account_alice, account_bob)
}

#[serial]
#[tokio::test]
async fn transaction_unauthorized_test() {
    // Start api server.
    let test_db = test_app::run().await;

    // Try unauthorized access to transaction handlers.
    let wrong_access_token = "xyz";
    let some_id = Uuid::new_v4();
    let result = transactions::get(some_id, wrong_access_token).await;
    assert_api_error_status!(result, StatusCode::UNAUTHORIZED);

    let result = transactions::transfer(some_id, some_id, 0, wrong_access_token).await;
    assert_api_error_status!(result, StatusCode::UNAUTHORIZED);

    // Drop test database.
    test_db.drop().await.unwrap();
}

#[serial]
#[tokio::test]
async fn transaction_non_existing_test() {
    // Start api server.
    let test_db = test_app::run().await;

    // Login as an admin.
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    // Check for non existing transaction.
    let transaction_id = Uuid::new_v4();
    let result = transactions::get(transaction_id, &tokens.access_token).await;

    assert!(result.is_err());
    match result.err().unwrap() {
        TestError::APIError(api_error) => {
            assert_eq!(api_error.status, StatusCode::NOT_FOUND);
            assert_eq!(api_error.errors.len(), 1);

            let error_entry = api_error.errors[0].clone();
            assert_eq!(
                error_entry.code,
                Some(APIErrorCode::TransactionNotFound.to_string())
            );
            assert_eq!(
                error_entry.kind,
                Some(APIErrorKind::ResourceNotFound.to_string())
            );
            assert_eq!(
                error_entry.message,
                TransactionError::TransactionNotFound(transaction_id).to_string()
            );
            let json = error_entry.detail.unwrap();
            assert_eq!(json["transaction_id"], transaction_id.to_string());
        }
        _ => panic!("invalid transaction result"),
    }

    // Drop test database.
    test_db.drop().await.unwrap();
}

#[serial]
#[tokio::test]
async fn transaction_transfer_test() {
    // Start api server.
    let test_db = test_app::run().await;

    // Login as an admin.
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    let (account_alice, account_bob) = prepare_accounts(&tokens).await;

    // Transfer money from Alice to Bob.
    let amount_cents = 25;
    let transaction = transactions::transfer(
        account_alice.id,
        account_bob.id,
        amount_cents,
        &tokens.access_token,
    )
    .await
    .unwrap();

    // Check for transaction details.
    assert_eq!(transaction.source_account_id, account_alice.id);
    assert_eq!(transaction.destination_account_id, account_bob.id);
    assert_eq!(transaction.amount_cents, amount_cents);
    let transaction_persisted = transactions::get(transaction.id, &tokens.access_token)
        .await
        .expect("Transaction fetch error.");
    assert_eq!(transaction_persisted, transaction);

    // Check for transfer results.
    let account_alice = accounts::get(account_alice.id, &tokens.access_token)
        .await
        .expect("Account fetch error.");
    assert_eq!(account_alice.balance_cents, 75);

    let account_bob = accounts::get(account_bob.id, &tokens.access_token)
        .await
        .expect("Account fetch error.");
    assert_eq!(account_bob.balance_cents, 125);

    // Transfer money from Bob to Alice.
    let amount_cents = 30;
    let transaction = transactions::transfer(
        account_bob.id,
        account_alice.id,
        amount_cents,
        &tokens.access_token,
    )
    .await
    .expect("Transaction error.");

    // Check for transaction details.
    assert_eq!(transaction.source_account_id, account_bob.id);
    assert_eq!(transaction.destination_account_id, account_alice.id);
    assert_eq!(transaction.amount_cents, amount_cents);

    // Check for persisted transaction.
    let transaction_persisted = transactions::get(transaction.id, &tokens.access_token)
        .await
        .expect("Transaction fetch error.");
    assert_eq!(transaction_persisted, transaction);

    // Check for transfer results.
    let account_alice = accounts::get(account_alice.id, &tokens.access_token)
        .await
        .expect("Account fetch error.");
    assert_eq!(account_alice.balance_cents, 105);

    let account_bob = accounts::get(account_bob.id, &tokens.access_token)
        .await
        .expect("Account fetch error.");
    assert_eq!(account_bob.balance_cents, 95);

    // Drop test database.
    test_db.drop().await.unwrap();
}

#[serial]
#[tokio::test]
async fn transfer_validate_invalid_accounts_test() {
    // Start api server.
    let test_db = test_app::run().await;

    // Login as an admin.
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    let source_account_id = Uuid::new_v4();
    let destination_account_id = Uuid::new_v4();
    let amount_cents = 100;

    let result = transactions::transfer(
        source_account_id,
        destination_account_id,
        amount_cents,
        &tokens.access_token,
    )
    .await;

    assert!(result.is_err());
    match result.err().unwrap() {
        TestError::APIError(api_error) => {
            assert_eq!(api_error.status, StatusCode::UNPROCESSABLE_ENTITY);
            assert_eq!(api_error.errors.len(), 2);

            let error_entry = api_error.errors[0].clone();
            assert_eq!(
                error_entry.code,
                Some(APIErrorCode::TransferSourceAccountNotFound.to_string())
            );
            assert_eq!(
                error_entry.kind,
                Some(APIErrorKind::ValidationError.to_string())
            );
            assert_eq!(
                error_entry.message,
                TransferValidationError::SourceAccountNotFound(source_account_id).to_string()
            );
            let json = error_entry.detail.unwrap();
            assert_eq!(json["source_account_id"], source_account_id.to_string());

            let error = api_error.errors[1].clone();
            assert_eq!(
                error.code,
                Some(APIErrorCode::TransferDestinationAccountNotFound.to_string())
            );
            assert_eq!(error.kind, Some(APIErrorKind::ValidationError.to_string()));
            assert_eq!(
                error.message,
                TransferValidationError::DestinationAccountNotFound(destination_account_id)
                    .to_string()
            );
            let json = error.detail.unwrap();
            assert_eq!(
                json["destination_account_id"],
                destination_account_id.to_string()
            );
        }
        _ => panic!("invalid transfer result"),
    }

    // Drop test database.
    test_db.drop().await.unwrap();
}

#[serial]
#[tokio::test]
async fn transfer_validate_distinct_accounts_test() {
    // Start api server.
    let test_db = test_app::run().await;

    // Login as an admin.
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    let source_account_id = Uuid::new_v4();
    let destination_account_id = source_account_id;
    let amount_cents = 100;

    let result = transactions::transfer(
        source_account_id,
        destination_account_id,
        amount_cents,
        &tokens.access_token,
    )
    .await;

    assert!(result.is_err());
    match result.err().unwrap() {
        TestError::APIError(api_error) => {
            assert_eq!(api_error.status, StatusCode::UNPROCESSABLE_ENTITY);
            assert_eq!(api_error.errors.len(), 2);

            let error_entry = api_error.errors[0].clone();
            assert_eq!(
                error_entry.code,
                Some(APIErrorCode::TransferSourceAccountNotFound.to_string())
            );
            assert_eq!(
                error_entry.kind,
                Some(APIErrorKind::ValidationError.to_string())
            );
            assert_eq!(
                error_entry.message,
                TransferValidationError::SourceAccountNotFound(source_account_id).to_string()
            );
            let json = error_entry.detail.unwrap();
            assert_eq!(json["source_account_id"], source_account_id.to_string());

            let error = api_error.errors[1].clone();
            assert_eq!(
                error.code,
                Some(APIErrorCode::TransferAccountsAreSame.to_string())
            );
            assert_eq!(error.kind, Some(APIErrorKind::ValidationError.to_string()));
            assert_eq!(
                error.message,
                TransferValidationError::AccountsAreSame.to_string()
            );
            assert_eq!(error.detail, None);
        }
        _ => panic!("invalid transfer result"),
    }

    // Drop test database.
    test_db.drop().await.unwrap();
}

#[serial]
#[tokio::test]
async fn transfer_validate_unsufficient_funds_test() {
    // Start api server.
    let test_db = test_app::run().await;

    // Login as an admin.
    let tokens = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .expect("Login error.");

    let (account_alice, account_bob) = prepare_accounts(&tokens).await;
    let amount_cents = 200;
    let result = transactions::transfer(
        account_bob.id,
        account_alice.id,
        amount_cents,
        &tokens.access_token,
    )
    .await;
    assert!(result.is_err());
    match result.err().unwrap() {
        TestError::APIError(api_error) => {
            assert_eq!(api_error.status, StatusCode::UNPROCESSABLE_ENTITY);
            assert_eq!(api_error.errors.len(), 1);

            let error_entry = api_error.errors[0].clone();
            assert_eq!(
                error_entry.code,
                Some(APIErrorCode::TransferInsufficientFunds.to_string())
            );
            assert_eq!(
                error_entry.kind,
                Some(APIErrorKind::ValidationError.to_string())
            );
            assert_eq!(
                error_entry.message,
                TransferValidationError::InsufficientFunds.to_string()
            );
            assert_eq!(error_entry.detail, None);
        }
        _ => panic!("invalid transfer result"),
    }

    // Drop test database.
    test_db.drop().await.unwrap();
}
