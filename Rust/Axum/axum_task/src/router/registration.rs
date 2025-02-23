use crate::models::user::User;
use axum::{extract::State, Json};
use sqlx::MySqlPool;

// /* To show User credentials as a response */
// // pub async fn post_method(Json(payload):Json<Value>)->Json<Value>{
// //     println!("My json data is:{:?}",payload);
// //     Json(payload)
// // }

// /* To show succcess message as a response */
pub async fn register_to_user(
    State(pool): State<MySqlPool>,
    Json(payload): Json<User>,
) -> Json<String> {
    let query = "INSERT INTO User (username, email, password) VALUES (?, ?, ?)";

    let result = sqlx::query(query)
        .bind(&payload.username)
        .bind(&payload.email)
        .bind(&payload.password)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Json("Registration Successful!".to_string()),
        Err(err) => Json(format!("Failed: {:?}", err)),
    }
}
