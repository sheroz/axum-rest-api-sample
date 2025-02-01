use std::sync::OnceLock;

use axum_web::application::config::Config;

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
