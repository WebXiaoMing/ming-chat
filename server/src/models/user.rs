use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub fullname: String,
    pub email: String,
    pub avatar: String,
    pub hashed_password: String,
    pub create_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct CreateUser {
    pub fullname: String,
    pub email: String,
    pub avatar: Option<String>,
    pub hashed_password: String,
}
