use crate::models::model::User;
use axum::{extract::State, Json};
use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn register_user(
    State(db_pool): State<Arc<Mutex<MySqlPool>>>,
    body: User,
) -> Json<String> {
    let pool = db_pool.lock().await;

    let result = sqlx::query!(
        "INSERT INTO User (username, email, password) VALUES (?, ?, ?)",
        body.username,
        body.email,
        body.password
    )
    .execute(&*pool)
    .await;

    match result {
        Ok(_) => Json("User registered successfully".to_string()),
        Err(_) => Json("Registration failed".to_string()),
    }
}
