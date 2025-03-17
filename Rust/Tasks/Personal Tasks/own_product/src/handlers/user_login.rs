use crate::jwt_utils::create_jwt;
use crate::models::model::User;
use axum::{extract::State, http::StatusCode, Json};
use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn login(
    State(db_pool): State<Arc<Mutex<MySqlPool>>>,
    body: User,
) -> Result<Json<String>, StatusCode> {
    // Lock the Mutex to get access to the pool
    let pool = db_pool.lock().await;

    let user = sqlx::query!(
        "SELECT id, username, password FROM User WHERE email=?",
        body.email
    )
    .fetch_optional(&*pool) // Dereference the pool to get &MySqlPool
    .await;

    match user {
        Ok(Some(record))
            if (record.password == body.password && record.username == body.username) =>
        {
            // Create a JWT for the user
            let token = create_jwt(record.id); // Pass the user ID to create a unique token
            Ok(Json(format!("Login successfully!! Token:-{}", token)))
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
