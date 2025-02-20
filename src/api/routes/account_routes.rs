use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{
    api::handlers::account_handlers::{
        add_account_handler, delete_account_handler, get_account_handler, list_accounts_handler,
        update_account_handler,
    },
    application::state::SharedState,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(list_accounts_handler))
        .route("/", post(add_account_handler))
        .route("/{id}", get(get_account_handler))
        .route("/{id}", put(update_account_handler))
        .route("/{id}", delete(delete_account_handler))
}
