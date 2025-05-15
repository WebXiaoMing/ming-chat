use axum::{extract::Path, response::IntoResponse};

pub async fn list_chat_handler() -> impl IntoResponse {
    "list_chat_handler"
}

pub async fn create_chat_handler() -> impl IntoResponse {
    "create_chat_handler"
}

pub async fn update_chat_handler() -> impl IntoResponse {
    "update_chat_handler"
}

pub async fn delete_chat_handler(Path(id): Path<u64>) -> impl IntoResponse {
    println!("delete chat id: {}", id);
    "delete_chat_handler"
}
