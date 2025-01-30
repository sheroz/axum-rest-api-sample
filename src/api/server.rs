use std::{collections::HashMap, sync::Arc};

use axum::{
    body::Body,
    extract::{Path, Query, Request},
    http::{HeaderMap, Method, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{any, get},
    Json, Router,
};
use serde_json::json;
use tokio::{
    net::TcpListener,
    signal::{
        self,
        unix::{self, SignalKind},
    },
};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    api::{
        error::APIError,
        routes::{account_routes, auth_routes, transaction_routes, user_routes},
        version::{self, APIVersion},
    },
    application::{constants::*, security::jwt::AccessClaims, state::SharedState},
};

pub async fn start(state: SharedState) {
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

    // Build the router.
    let router = Router::new()
        .route("/", get(root_handler))
        .route("/head", get(head_request_handler))
        .route("/any", any(any_request_handler))
        .route("/{version}/heartbeat/{id}", get(heartbeat_handler))
        // Nesting authentication routes.
        .nest("/{version}/auth", auth_routes::routes())
        // Nesting user routes.
        .nest("/{version}/users", user_routes::routes())
        // Nesting account routes.
        .nest("/{version}/accounts", account_routes::routes())
        // Nesting transaction routes.
        .nest("/{version}/transactions", transaction_routes::routes())
        // Add a fallback service for handling routes to unknown paths.
        .fallback(error_404_handler)
        .with_state(Arc::clone(&state))
        .layer(cors_layer)
        .layer(middleware::from_fn(logging_middleware));

    // Build the listener.
    let addr = state.config.service_socket_addr();
    let listener = TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on {}", addr);

    // Start the API service.
    axum::serve(listener, router)
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
        unix::signal(SignalKind::terminate())
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

#[tracing::instrument(level = tracing::Level::TRACE, name = "axum", skip_all, fields(method=request.method().to_string(), uri=request.uri().to_string()))]
pub async fn logging_middleware(request: Request<Body>, next: Next) -> Response {
    tracing::trace!(
        "received a {} request to {}",
        request.method(),
        request.uri()
    );
    next.run(request).await
}

pub async fn root_handler(access_claims: AccessClaims) -> Result<impl IntoResponse, APIError> {
    if tracing::enabled!(tracing::Level::TRACE) {
        tracing::trace!(
            "current timestamp, chrono::Utc {}",
            chrono::Utc::now().timestamp() as usize
        );
        let start = std::time::SystemTime::now();
        let validation_timestamp = start
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        tracing::trace!("current timestamp, std::time {}", validation_timestamp);
        tracing::trace!("authentication details: {:#?}", access_claims);
    }
    Ok(Json(json!({"message": "Hello from Axum-Web!"})))
}

pub async fn heartbeat_handler(
    Path((version, id)): Path<(String, String)>,
) -> Result<impl IntoResponse, APIError> {
    let api_version: APIVersion = version::parse_version(&version)?;
    tracing::trace!("heartbeat: api version: {}", api_version);
    tracing::trace!("heartbeat: received id: {}", id);
    let map = HashMap::from([
        ("service".to_string(), SERVICE_NAME.to_string()),
        ("version".to_string(), SERVICE_VERSION.to_string()),
        ("heartbeat-id".to_string(), id),
    ]);
    Ok(Json(map))
}

pub async fn head_request_handler(method: Method) -> Response {
    // Using HEAD requests makes sense if processing (computing) the response body is costly.
    if method == Method::HEAD {
        tracing::debug!("HEAD method found");
        return [("x-some-header", "header from HEAD")].into_response();
    }
    ([("x-some-header", "header from GET")], "body from GET").into_response()
}

pub async fn any_request_handler(
    method: Method,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    request: Request,
) -> impl IntoResponse {
    if tracing::enabled!(tracing::Level::DEBUG) {
        tracing::debug!("method: {:?}", method);
        tracing::debug!("headers: {:?}", headers);
        tracing::debug!("params: {:?}", params);
        tracing::debug!("request: {:?}", request);
    }
    StatusCode::OK
}

pub async fn error_404_handler(request: Request) -> impl IntoResponse {
    tracing::error!("route not found: {:?}", request);
    StatusCode::NOT_FOUND
}
