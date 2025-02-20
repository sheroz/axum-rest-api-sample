use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    api::handlers::transaction_handlers::{get_transaction_handler, transfer_handler},
    application::state::SharedState,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/transfer", post(transfer_handler))
        .route("/{id}", get(get_transaction_handler))
}
