use serial_test::serial;
use uuid::Uuid;

use axum_web::{
    application::security::roles::UserRole,
    domain::models::{account::Account, transaction::TransactionResult, user::User},
};
use reqwest::StatusCode;

pub mod common;
use common::{
    accounts, auth,
    constants::{TEST_ADMIN_PASSWORD_HASH, TEST_ADMIN_USERNAME},
    transactions, users, utils,
};

#[tokio::test]
#[serial]
async fn account_transaction_test() {
    // Load the test configuration and start the api server.
    utils::start_api().await;

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

    // Try unauthorized access to transaction handlers.
    let some_id = Uuid::new_v4();
    let (status, _) = transactions::get(some_id, &access_token).await.unwrap();
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let (status, _) = transactions::transfer(some_id, some_id, 0, &access_token)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // Login as an admin.
    let (status, result) = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    let (access_token, _) = result.unwrap();

    // Add user for Alice.
    let username = "alice";
    let user_alice = User {
        id: Uuid::new_v4(),
        username: format!(
            "test-{}-{}",
            username,
            chrono::Utc::now().timestamp() as usize
        ),
        email: format!("{}@email.com", username),
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

    // Get the added account.
    let (status, result) = accounts::get(account_alice.id, &access_token)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    assert_eq!(result.unwrap(), account_alice);

    // Add user for Bob.
    let username = "bob";
    let user_bob = User {
        id: Uuid::new_v4(),
        username: format!(
            "test-{}-{}",
            username,
            chrono::Utc::now().timestamp() as usize
        ),
        email: format!("{}@email.com", username),
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

    // Get the added account.
    let (status, result) = accounts::get(account_bob.id, &access_token).await.unwrap();
    assert_eq!(status, StatusCode::OK);
    assert_eq!(result.unwrap(), account_bob);

    // list the existing accounts.
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
    let amount = 25;
    let (status, result) =
        transactions::transfer(account_alice.id, account_bob.id, amount, &access_token)
            .await
            .unwrap();
    assert_eq!(status, reqwest::StatusCode::OK);
    let transaction_result = result.unwrap();
    assert!(matches!(transaction_result, TransactionResult::Success(_)));

    // Check for transaction details.
    if let TransactionResult::Success(transaction) = transaction_result {
        assert_eq!(transaction.from_account_id, account_alice.id);
        assert_eq!(transaction.to_account_id, account_bob.id);
        assert_eq!(transaction.amount_cents, amount);
        let (status, result) = transactions::get(transaction.id, &access_token)
            .await
            .unwrap();
        assert_eq!(status, StatusCode::OK);
        assert_eq!(result.unwrap(), transaction);
    } else {
        panic!("Invalid transaction result.")
    }

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
    let amount = 30;
    let (status, result) =
        transactions::transfer(account_bob.id, account_alice.id, amount, &access_token)
            .await
            .unwrap();
    assert_eq!(status, reqwest::StatusCode::OK);
    let transaction_result = result.unwrap();
    assert!(matches!(transaction_result, TransactionResult::Success(_)));

    // Check for transaction details.
    if let TransactionResult::Success(transaction) = transaction_result {
        assert_eq!(transaction.from_account_id, account_bob.id);
        assert_eq!(transaction.to_account_id, account_alice.id);
        assert_eq!(transaction.amount_cents, amount);
        let (status, result) = transactions::get(transaction.id, &access_token)
            .await
            .unwrap();
        assert_eq!(status, StatusCode::OK);
        assert_eq!(result.unwrap(), transaction);
    } else {
        panic!("Invalid transaction result.")
    }

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
    let amount = 200;
    let (status, result) =
        transactions::transfer(account_bob.id, account_alice.id, amount, &access_token)
            .await
            .unwrap();
    assert_eq!(status, reqwest::StatusCode::UNPROCESSABLE_ENTITY);
    assert!(matches!(
        result.unwrap(),
        TransactionResult::InsufficientFunds
    ));

    // Check for invalid source account.
    let amount = 200;
    let account_id = Uuid::new_v4();
    let (status, result) =
        transactions::transfer(account_id, account_bob.id, amount, &access_token)
            .await
            .unwrap();
    assert_eq!(status, reqwest::StatusCode::UNPROCESSABLE_ENTITY);
    let transaction_result = result.unwrap();
    assert!(matches!(
        transaction_result,
        TransactionResult::SourceAccountNotFound(_)
    ));
    if let TransactionResult::SourceAccountNotFound(id) = transaction_result {
        assert_eq!(id, account_id);
    } else {
        panic!("Invalid transaction result.")
    }

    // Check for invalid destination account.
    let (status, result) =
        transactions::transfer(account_alice.id, account_id, amount, &access_token)
            .await
            .unwrap();
    assert_eq!(status, reqwest::StatusCode::UNPROCESSABLE_ENTITY);
    let transaction_result = result.unwrap();
    assert!(matches!(
        transaction_result,
        TransactionResult::DestinationAccountNotFound(_)
    ));
    if let TransactionResult::DestinationAccountNotFound(id) = transaction_result {
        assert_eq!(id, account_id);
    } else {
        panic!("Invalid transaction result.")
    }
}
