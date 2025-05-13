use axum::Router;

use crate::AppState;

mod auth;
mod chat;
mod messages;

pub fn api(state: AppState) -> Router {
    Router::new()
        .nest("/auth", auth::routes(state.clone()))
        .nest("/chat", chat::routes(state.clone()))
        .nest("/messages", messages::routes(state.clone()))
}
