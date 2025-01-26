use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use axum_web::application::app;

#[tokio::main]
async fn main() {
    // Tracing configuration.
    let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "axum_web=trace".into());
    let fmt_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_target(false)
        .with_file(true)
        .with_line_number(true);
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    tracing::info!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    app::run().await;
}
