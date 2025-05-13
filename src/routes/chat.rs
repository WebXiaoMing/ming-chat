use crate::handlers::chat::*;
use crate::{AppState, handlers::chat::create_chat_handler};
use axum::{
    Router,
    routing::{delete, get, post},
};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/list", get(list_chat_handler))
        .route("/create", post(create_chat_handler))
        .route("/update", post(update_chat_handler))
        .route("/delete/:id", delete(delete_chat_handler))
        .with_state(state)
}
