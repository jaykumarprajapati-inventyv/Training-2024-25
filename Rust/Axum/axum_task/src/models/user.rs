use serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(Debug, Deserialize, Serialize, FromRow)]

pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]

pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
