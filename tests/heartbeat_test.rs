use serial_test::serial;
use uuid::Uuid;

use axum_web::application::constants::*;

pub mod common;
use common::{
    constants::{API_PATH_HEARTBEAT, API_V1},
    helpers,
    hyper_fetch::hyper_fetch,
    test_app,
};

#[tokio::test]
#[serial]
async fn heartbeat_test() {
    // Start the api server.
    test_app::run().await;

    let heartbeat_id = Uuid::new_v4().to_string();
    let url = helpers::build_url(API_V1, API_PATH_HEARTBEAT, &heartbeat_id);

    // Fetch using `reqwest`.
    let response = reqwest::get(url.as_str()).await.unwrap();
    let body = response.text().await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["service"], SERVICE_NAME);
    assert_eq!(json["version"], SERVICE_VERSION);
    assert_eq!(json["heartbeat-id"], heartbeat_id);

    // Fetch using `hyper`.
    let body = hyper_fetch(url.as_str()).await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["service"], SERVICE_NAME);
    assert_eq!(json["version"], SERVICE_VERSION);
    assert_eq!(json["heartbeat-id"], heartbeat_id);
}
