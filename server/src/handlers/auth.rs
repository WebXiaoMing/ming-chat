use axum::{extract::State, response::IntoResponse};

use crate::AppState;

pub async fn signin_handler(state: State<AppState>) -> impl IntoResponse {
    println!("app state: {:?}", state.config);
    "sigin"
}

pub async fn signup_handler() -> impl IntoResponse {
    "signup"
}
