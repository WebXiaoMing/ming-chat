use axum::{extract::Path, response::IntoResponse};

pub async fn list_message_handler(Path(chat_id): Path<u64>) -> impl IntoResponse {
    println!("get messages from chat: {}", chat_id);
    "list_message_handler"
}

pub async fn send_message_handler(Path(chat_id): Path<u64>) -> impl IntoResponse {
    println!("send messages from chat: {}", chat_id);
    "send_message_handler"
}
