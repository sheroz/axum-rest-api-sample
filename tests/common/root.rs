use reqwest::StatusCode;

use crate::common::{TestResult, helpers};

// Fetch the root using `reqwest`.
pub async fn fetch_root(access_token: &str) -> TestResult<StatusCode> {
    let url = helpers::config().service_http_addr();

    let authorization = format!("Bearer {}", access_token);
    let response = reqwest::Client::new()
        .get(&url)
        .header("Authorization", authorization)
        .send()
        .await?;

    let response_status = response.status();
    if response_status == StatusCode::OK {
        let found = response.text().await.unwrap();
        let expected = r#"{"message":"Hello from Axum-Web!"}"#;
        assert_eq!(found, expected);
    }
    Ok(response_status)
}
