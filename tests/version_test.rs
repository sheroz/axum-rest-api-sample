use reqwest::StatusCode;
use serial_test::serial;

pub mod common;
use common::{
    constants::{API_PATH_VERSION, API_V1},
    helpers, test_app,
};

#[tokio::test]
#[serial]
async fn version_test() {
    // Start API server.
    let test_db = test_app::run().await;

    let url = helpers::build_path(API_V1, API_PATH_VERSION);
    let response = reqwest::get(url.as_str()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.text().await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["name"], env!("CARGO_PKG_NAME"));
    assert_eq!(json["version"], env!("CARGO_PKG_VERSION"));

    // Drop test database.
    test_db.drop().await.unwrap();
}
