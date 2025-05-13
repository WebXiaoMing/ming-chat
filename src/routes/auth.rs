use crate::{AppState, handlers::auth::*};
use axum::{Router, routing::post};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler))
        .with_state(state)
}
