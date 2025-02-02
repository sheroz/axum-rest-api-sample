use std::sync::OnceLock;

use reqwest::{Response, StatusCode};
use serde::Deserialize;

use axum_web::{api::APIError, application::config::Config};

use crate::common::{TestError, TestResult};

pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn config() -> &'static Config {
    CONFIG.get().unwrap()
}

pub fn build_url(version: &str, path: &str, url: &str) -> reqwest::Url {
    let url = format!(
        "{}/{}/{}/{}",
        config().service_http_addr(),
        version,
        path,
        url
    );
    reqwest::Url::parse(&url).unwrap()
}

pub fn build_path(version: &str, path: &str) -> reqwest::Url {
    let url = format!("{}/{}/{}", config().service_http_addr(), version, path);
    reqwest::Url::parse(&url).unwrap()
}

pub async fn dispatch_reqwest_response<T>(
    response: Response,
    expected_status: StatusCode,
) -> TestResult<Option<T>>
where
    T: for<'a> Deserialize<'a>,
{
    let status = response.status();
    if status == expected_status {
        let body = response.text().await.unwrap();
        if body.is_empty() {
            return Ok(None);
        } else {
            let result: T = serde_json::from_str(&body).unwrap();
            return Ok(Some(result));
        }
    }

    if status.is_client_error() || status.is_server_error() {
        let body = response.text().await.unwrap();
        let api_error = serde_json::from_str::<APIError>(&body).unwrap();
        Err(api_error)?
    } else {
        Err(TestError::UnexpectedResponse { response })?
    }
}

#[macro_export]
macro_rules! assert_api_error_status {
    ($result:expr, $expected:expr) => {
        assert!($result.is_err());
        let error = $result.err().unwrap();
        match error {
            $crate::common::TestError::APIError(api_error) => {
                assert_eq!(api_error.status, $expected);
            }
            $crate::common::TestError::NetworkError(error) => {
                panic!("Unexpected network error: {}", error)
            }
            $crate::common::TestError::UnexpectedResponse { response } => panic!(
                "Unexpected response status. Expected: {}, Found: {}",
                $expected,
                response.status()
            ),
        }
    };
}
