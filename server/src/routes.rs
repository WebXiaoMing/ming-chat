use crate::{AppState, handlers::*};
use axum::{
    Router,
    routing::{delete, get, post},
};

pub(crate) fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler))
}

pub(crate) fn chat_router() -> Router<AppState> {
    Router::new()
        .route("/chat/list", get(list_chat_handler))
        .route("/chat/create", post(create_chat_handler))
        .route("/chat/update", post(update_chat_handler))
        .route("/chat/{id}", delete(delete_chat_handler))
}

pub(crate) fn messages_router() -> Router<AppState> {
    Router::new().route(
        "/messages/{chat_id}",
        get(list_message_handler).post(send_message_handler),
    )
}

pub fn app_router(state: AppState) -> Router {
    Router::new()
        .merge(auth_router())
        .merge(chat_router())
        .merge(messages_router())
        .with_state(state)
}
