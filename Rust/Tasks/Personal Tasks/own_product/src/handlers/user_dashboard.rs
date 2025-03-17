use axum::{extract::Request, Json};

pub async fn user_dashboard_handler(req: Request) -> Result<Json<String>, String> {
    println!("Protected rquest:-{:#?}",req);
    // Extract the user ID added by the middleware
    let user_id = req.extensions().get::<i32>().ok_or("Unauthorized")?; // Handle missing ID
    Ok(Json(format!("Access granted for user ID: {}", user_id)))
}
