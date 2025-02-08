use reqwest::StatusCode;
use serial_test::serial;

pub mod common;
use common::{
    constants::{API_PATH_HEALTH, API_V1},
    helpers,
    hyper_fetch::hyper_fetch,
    test_app,
};

#[tokio::test]
#[serial]
async fn health_test() {
    // Start API server.
    let test_db = test_app::run().await;

    let url = helpers::build_path(API_V1, API_PATH_HEALTH);

    // Fetch using `reqwest`.
    let response = reqwest::get(url.as_str()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.text().await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["status"], "healthy");

    // Fetch using `hyper`.
    let body = hyper_fetch(url.as_str()).await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["status"], "healthy");

    // Drop test database.
    test_db.drop().await.unwrap();
}
