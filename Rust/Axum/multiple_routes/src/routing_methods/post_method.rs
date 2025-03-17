use axum::extract::Json;
use serde_json::Value;

pub async fn method_of_post(Json(body): Json<Value>) -> Json<Value> {
    Json(body) // Returns the received JSON
}
