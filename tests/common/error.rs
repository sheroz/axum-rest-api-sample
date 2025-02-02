use axum_web::api::APIError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TestError {
    #[error("API error: {0}")]
    APIError(APIError),
    #[error("Network error: {0}")]
    NetworkError(reqwest::Error),
    #[error("Unexpected response status: {}", response.status())]
    UnexpectedResponse { response: reqwest::Response },
}

impl From<APIError> for TestError {
    fn from(error: APIError) -> Self {
        Self::APIError(error)
    }
}

impl From<reqwest::Error> for TestError {
    fn from(error: reqwest::Error) -> Self {
        Self::NetworkError(error)
    }
}

impl From<reqwest::Response> for TestError {
    fn from(response: reqwest::Response) -> Self {
        Self::UnexpectedResponse { response }
    }
}
