use axum::{extract::State, Json};
use sqlx::MySqlPool;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")] 
pub struct Laptop {
    pub company_name: String,
    pub ram_size: i32,
    pub model_name: String,
    pub price: i32,
}

pub async fn product_assign(
    State(pool): State<MySqlPool>,
    Json(payload): Json<Laptop>,
) -> Json<String> {
    let query =
        "INSERT INTO Laptops (CompanyName, RamSize, ModelName, Price)  VALUES (?, ?, ?, ?)";

    let result = sqlx::query(query)
        .bind(&payload.company_name)
        .bind(&payload.ram_size)
        .bind(&payload.model_name)
        .bind(&payload.price)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Json("Laptop details stored successfully!".to_string()),
        Err(err) => Json(format!("Failed: {:?}", err)),
    }
}
