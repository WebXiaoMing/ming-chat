use axum::response::IntoResponse;

pub async fn list_message_handler() -> impl IntoResponse {
    "list_message_handler"
}

pub async fn send_message_handler() -> impl IntoResponse {
    "send_message_handler"
}
