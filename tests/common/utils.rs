use std::{sync::OnceLock, time::Duration};

use reqwest::StatusCode;
use tokio::time::Instant;

use axum_web::application::{
    app,
    config::{self, Config},
};

use crate::common::constants::{API_PATH_HEARTBEAT, API_V1};

pub static CONFIG: OnceLock<Config> = OnceLock::new();

// TODO: use isolated databases and remove all `serial` dependencies.

pub async fn run_app() {
    std::env::set_var("ENV_TEST", "1");

    let config = config::load();
    CONFIG.get_or_init(|| config);

    // Run the api server.
    tokio::spawn(async move {
        app::run().await;
    });

    wait_for_service(Duration::from_secs(5)).await;
}

async fn wait_for_service(duration: Duration) {
    let timeout = Instant::now() + duration;
    loop {
        let url = build_url(API_V1, API_PATH_HEARTBEAT, "1");
        let response = reqwest::get(url.as_str()).await.unwrap();
        if response.status() == StatusCode::OK {
            break;
        }
        if Instant::now() > timeout {
            panic!("Could not start API Service in 5 seconds");
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

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
