use crate::models::user::{LoginRequest, User};
use crate::utils::jwt::generate_token;
use axum::{extract::State, Json};
use sqlx::MySqlPool; 


pub async fn login_authentication(
    State(pool): State<MySqlPool>,
    Json(payload): Json<LoginRequest>,
) -> Json<serde_json::Value> {
    let query = "SELECT username, email, password FROM User WHERE email = ?";

    let user = sqlx::query_as::<_, User>(query)
        .bind(&payload.email)
        .fetch_optional(&pool)
        .await;

    match user {
        Ok(Some(user)) if user.password == payload.password => {
            let token = generate_token(&user.email);
            Json(serde_json::json!({ "message": "Login Successful!", "token": token }))
        }
        _ => Json(serde_json::json!({ "message": "Invalid email or password" })),
    }
}
