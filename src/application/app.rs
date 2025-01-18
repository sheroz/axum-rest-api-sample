use crate::{
    api::router,
    application::{config, state::AppState},
    infrastructure::{postgres, redis},
};
use std::sync::Arc;
use tokio::{
    signal,
    sync::{oneshot, Mutex},
};
use tower_http::cors::{Any, CorsLayer};

pub async fn start_server(api_ready: oneshot::Sender<()>) {
    // Load configuration.
    config::load();
    let config = config::get();

    // Connect to Redis.
    let redis = redis::open(config).await;

    // Connect to PostgreSQL.
    let db_pool = postgres::pgpool(config).await;

    // Run migrations.
    sqlx::migrate!("src/infrastructure/postgres/migrations")
        .run(&db_pool)
        .await
        .unwrap();

    // Build a CORS layer.
    // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
    // for more details
    let cors_layer = CorsLayer::new().allow_origin(Any);
    // let cors_header_value = config.service_http_addr().parse::<HeaderValue>().unwrap();
    // let cors_layer = CorsLayer::new()
    //      .allow_origin(cors_header_value)
    //      .allow_methods([
    //          Method::HEAD,
    //          Method::GET,
    //          Method::POST,
    //          Method::PATCH,
    //          Method::DELETE,
    //      ])
    //      .allow_credentials(true)
    //      .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // Get the listening address.
    let addr = config.service_socket_addr();

    // Build the application state.
    let shared_state = Arc::new(AppState {
        db_pool,
        redis: Mutex::new(redis),
    });

    // Build the app.
    let app = router::routes(shared_state)
        .layer(cors_layer)
        .layer(axum::middleware::from_fn(router::logging_middleware));

    // Build the listener.
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on {}", addr);

    api_ready.send(()).expect("Couild not send a ready signal");

    // Start the API service.
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    tracing::info!("server shutdown successfully.");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("received termination signal, shutting down...");
}
