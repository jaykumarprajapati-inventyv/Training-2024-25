use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QueryResponses {
    id: i32,
    firstname: String,
    lastname: String,
    address:String
}

pub async fn query_parameters(Query(param): Query<QueryResponses>) -> Json<QueryResponses> {
    Json(param)
}
