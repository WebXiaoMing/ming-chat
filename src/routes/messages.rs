use crate::{AppState, handlers::messages::*};
use axum::{Router, routing::get};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route(
            "/:chat_id",
            get(list_message_handler).post(send_message_handler),
        )
        .with_state(state)
}
