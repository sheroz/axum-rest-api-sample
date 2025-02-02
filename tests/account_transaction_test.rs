use reqwest::StatusCode;
use serial_test::serial;
use uuid::Uuid;

use axum_web::{
    api::{handlers::transaction_handlers::TransactionError, APIError, APIErrorCode, APIErrorKind},
    application::{
        security::roles::UserRole, service::transaction_service::TransferValidationError,
    },
    domain::models::{account::Account, user::User},
};

pub mod common;
use common::{
    accounts, auth,
    constants::{TEST_ADMIN_PASSWORD_HASH, TEST_ADMIN_USERNAME},
    test_app, transactions, users,
};

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
    let access_token = "xyz".to_string();
    let (status, _) = accounts::get(account.id, &access_token).await.unwrap();
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let (status, _) = accounts::add(account.clone(), &access_token).await.unwrap();
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let (status, _) = accounts::update(account.clone(), &access_token)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // Drop test database.
    test_db.drop().await.unwrap();
}

#[serial]
#[tokio::test]
async fn transaction_unauthorized_test() {
    // Start api server.
    let test_db = test_app::run().await;

    // Try unauthorized access to transaction handlers.
    let access_token = "xyz".to_string();
    let some_id = Uuid::new_v4();
    let result = transactions::get(some_id, &access_token).await;
    assert!(result.is_err());
    match result.err().unwrap() {
        transactions::TransactionResponseError::UnexpectedResponse(response) => {
            assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);
        }
        _ => panic!("invalid access result"),
    }

    let result = transactions::transfer(some_id, some_id, 0, &access_token).await;
    assert!(result.is_err());
    match result.err().unwrap() {
        transactions::TransactionResponseError::UnexpectedResponse(response) => {
            assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED)
        }
        _ => panic!("invalid access result"),
    }

    // Drop test database.
    test_db.drop().await.unwrap();
}

#[serial]
#[tokio::test]
async fn account_transaction_test() {
    // Start api server.
    let test_db = test_app::run().await;

    // Login as an admin.
    let (status, result) = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    let (access_token, _) = result.unwrap();

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

    let (status, _) = users::add(user_alice.clone(), &access_token).await.unwrap();
    assert_eq!(status, StatusCode::CREATED);

    // Add account for Alice.
    let mut account_alice = Account {
        id: Uuid::new_v4(),
        user_id: user_alice.id,
        balance_cents: 0,
        created_at: None,
        updated_at: None,
    };

    let (status, _) = accounts::get(account_alice.id, &access_token)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::NOT_FOUND);

    let (status, result) = accounts::add(account_alice.clone(), &access_token)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::CREATED);
    let account_result = result.unwrap();
    assert!(account_result.updated_at.is_some());
    assert!(account_result.created_at.is_some());
    account_alice.created_at = account_result.created_at;
    account_alice.updated_at = account_result.updated_at;
    assert_eq!(account_result, account_alice);

    // Get added account.
    let (status, result) = accounts::get(account_alice.id, &access_token)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    assert_eq!(result.unwrap(), account_alice);

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

    let (status, _) = users::add(user_bob.clone(), &access_token).await.unwrap();
    assert_eq!(status, StatusCode::CREATED);

    // Add account for Bob.
    let mut account_bob = Account {
        id: Uuid::new_v4(),
        user_id: user_bob.id,
        balance_cents: 0,
        created_at: None,
        updated_at: None,
    };

    let (status, _) = accounts::get(account_bob.id, &access_token).await.unwrap();
    assert_eq!(status, StatusCode::NOT_FOUND);

    let (status, result) = accounts::add(account_bob.clone(), &access_token)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::CREATED);
    let account_result = result.unwrap();
    assert!(account_result.updated_at.is_some());
    assert!(account_result.created_at.is_some());
    account_bob.created_at = account_result.created_at;
    account_bob.updated_at = account_result.updated_at;
    assert_eq!(account_result, account_bob);

    // Get added account.
    let (status, result) = accounts::get(account_bob.id, &access_token).await.unwrap();
    assert_eq!(status, StatusCode::OK);
    assert_eq!(result.unwrap(), account_bob);

    // list existing accounts.
    let (status, result) = accounts::list(&access_token).await.unwrap();
    assert_eq!(status, reqwest::StatusCode::OK);
    let accounts = result.unwrap();
    assert!(accounts.contains(&account_alice));
    assert!(accounts.contains(&account_bob));

    // Update accounts.
    account_alice.balance_cents = 100;
    let (status, result) = accounts::update(account_alice.clone(), &access_token)
        .await
        .unwrap();
    assert_eq!(status, reqwest::StatusCode::OK);
    assert!(result.is_some());
    let account_result = result.unwrap();
    assert_ne!(account_result.updated_at, account_alice.updated_at);
    account_alice.updated_at = account_result.updated_at;
    assert_eq!(account_result, account_alice);

    account_bob.balance_cents = 100;
    let (status, result) = accounts::update(account_bob.clone(), &access_token)
        .await
        .unwrap();
    assert_eq!(status, reqwest::StatusCode::OK);
    assert!(result.is_some());
    let account_result = result.unwrap();
    assert_ne!(account_result.updated_at, account_bob.updated_at);
    account_bob.updated_at = account_result.updated_at;
    assert_eq!(account_result, account_bob);

    // Transfer money from Alice to Bob.
    let amount_cents = 25;
    let transaction = transactions::transfer(
        account_alice.id,
        account_bob.id,
        amount_cents,
        &access_token,
    )
    .await
    .unwrap();

    // Check for transaction details.
    assert_eq!(transaction.source_account_id, account_alice.id);
    assert_eq!(transaction.destination_account_id, account_bob.id);
    assert_eq!(transaction.amount_cents, amount_cents);
    let transaction_persisted = transactions::get(transaction.id, &access_token)
        .await
        .unwrap();
    assert_eq!(transaction_persisted, transaction);

    // Check for transfer results.
    let (status, result) = accounts::get(account_alice.id, &access_token)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    assert_eq!(result.unwrap().balance_cents, 75);

    let (status, result) = accounts::get(account_bob.id, &access_token).await.unwrap();
    assert_eq!(status, StatusCode::OK);
    assert_eq!(result.unwrap().balance_cents, 125);

    // Transfer money from Bob to Alice.
    let amount_cents = 30;
    let transaction = transactions::transfer(
        account_bob.id,
        account_alice.id,
        amount_cents,
        &access_token,
    )
    .await
    .unwrap();
    assert_eq!(status, reqwest::StatusCode::OK);

    // Check for transaction details.
    assert_eq!(transaction.source_account_id, account_bob.id);
    assert_eq!(transaction.destination_account_id, account_alice.id);
    assert_eq!(transaction.amount_cents, amount_cents);

    // Check for persisted transaction.
    let transaction_persisted = transactions::get(transaction.id, &access_token)
        .await
        .unwrap();
    assert_eq!(transaction_persisted, transaction);

    // Check for transfer results.
    let (status, result) = accounts::get(account_alice.id, &access_token)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    assert_eq!(result.unwrap().balance_cents, 105);

    let (status, result) = accounts::get(account_bob.id, &access_token).await.unwrap();
    assert_eq!(status, StatusCode::OK);
    assert_eq!(result.unwrap().balance_cents, 95);

    // Check for unsufficient funds.
    let amount_cents = 200;
    let result = transactions::transfer(
        account_bob.id,
        account_alice.id,
        amount_cents,
        &access_token,
    )
    .await;
    assert!(result.is_err());
    match result.err().unwrap() {
        transactions::TransactionResponseError::UnexpectedResponse(response) => {
            assert_eq!(response.status(), reqwest::StatusCode::UNPROCESSABLE_ENTITY);
            let body = response.text().await.unwrap();
            let error_response = serde_json::from_str::<APIError>(&body).unwrap();
            assert_eq!(error_response.status, StatusCode::UNPROCESSABLE_ENTITY);
            assert_eq!(error_response.errors.len(), 1);

            let error = error_response.errors[0].clone();
            assert_eq!(
                error.code,
                serde_json::to_string(&APIErrorCode::TransactionInsufficientFunds).ok()
            );
            assert_eq!(
                error.kind,
                serde_json::to_string(&APIErrorKind::ValidationError).ok()
            );
            assert_eq!(
                error.message,
                TransferValidationError::InsufficientFunds.to_string()
            );
            assert_eq!(error.detail, None);
        }
        _ => panic!("invalid transaction transfer result"),
    }

    // Drop test database.
    test_db.drop().await.unwrap();
}

#[serial]
#[tokio::test]
async fn transaction_account_validation_test() {
    // Start api server.
    let test_db = test_app::run().await;

    // Login as an admin.
    let (status, result) = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    let (access_token, _) = result.unwrap();

    // Check for invalid source account.
    let source_account_id = Uuid::new_v4();
    let destination_account_id = Uuid::new_v4();
    let amount_cents = 100;

    let result = transactions::transfer(
        source_account_id,
        destination_account_id,
        amount_cents,
        &access_token,
    )
    .await;

    assert!(result.is_err());
    match result.err().unwrap() {
        transactions::TransactionResponseError::UnexpectedResponse(response) => {
            assert_eq!(response.status(), reqwest::StatusCode::UNPROCESSABLE_ENTITY);
            let body = response.text().await.unwrap();
            let error_response = serde_json::from_str::<APIError>(&body).unwrap();
            assert_eq!(error_response.status, StatusCode::UNPROCESSABLE_ENTITY);
            assert_eq!(error_response.errors.len(), 2);

            let error = error_response.errors[0].clone();
            assert_eq!(
                error.code,
                serde_json::to_string(&APIErrorCode::TransactionSourceAccountNotFound).ok()
            );
            assert_eq!(
                error.kind,
                serde_json::to_string(&APIErrorKind::ValidationError).ok()
            );
            assert_eq!(
                error.message,
                TransferValidationError::SourceAccountNotFound(source_account_id).to_string()
            );
            let json = error.detail.unwrap();
            assert_eq!(json["source_account_id"], source_account_id.to_string());

            let error = error_response.errors[1].clone();
            assert_eq!(
                error.code,
                serde_json::to_string(&APIErrorCode::TransactionDestinationAccountNotFound).ok()
            );
            assert_eq!(
                error.kind,
                serde_json::to_string(&APIErrorKind::ValidationError).ok()
            );
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
        _ => panic!("invalid transaction transfer result"),
    }

    // Drop test database.
    test_db.drop().await.unwrap();
}

#[serial]
#[tokio::test]
async fn transaction_non_existing_test() {
    // Start api server.
    let test_db = test_app::run().await;

    // Login as an admin.
    let (status, result) = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    let (access_token, _) = result.unwrap();

    // Check for non existing transaction.
    let transaction_id = Uuid::new_v4();
    let result = transactions::get(transaction_id, &access_token).await;

    assert!(result.is_err());
    match result.err().unwrap() {
        transactions::TransactionResponseError::UnexpectedResponse(response) => {
            assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);
            let body = response.text().await.unwrap();
            let error_response = serde_json::from_str::<APIError>(&body).unwrap();
            assert_eq!(error_response.status, StatusCode::NOT_FOUND);
            assert_eq!(error_response.errors.len(), 1);

            let error = error_response.errors[0].clone();
            assert_eq!(
                error.code,
                serde_json::to_string(&APIErrorCode::TransactionNotFound).ok()
            );
            assert_eq!(
                error.kind,
                serde_json::to_string(&APIErrorKind::ResourceNotFound).ok()
            );
            assert_eq!(
                error.message,
                TransactionError::TransactionNotFound(transaction_id).to_string()
            );
            let json = error.detail.unwrap();
            assert_eq!(json["transaction_id"], transaction_id.to_string());
        }
        _ => panic!("invalid transaction result"),
    }

    // Drop test database.
    test_db.drop().await.unwrap();
}
