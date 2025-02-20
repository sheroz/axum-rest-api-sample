use axum::{Router, routing::post};

use crate::{
    api::handlers::auth_handlers::{
        cleanup_handler, login_handler, logout_handler, refresh_handler, revoke_all_handler,
        revoke_user_handler,
    },
    application::state::SharedState,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler))
        .route("/refresh", post(refresh_handler))
        .route("/revoke-all", post(revoke_all_handler))
        .route("/revoke-user", post(revoke_user_handler))
        .route("/cleanup", post(cleanup_handler))
}
