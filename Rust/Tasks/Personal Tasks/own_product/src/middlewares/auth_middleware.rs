use crate::jwt_utils::validate_jwt;
use axum::{http::Request, middleware::Next, response::Response};

pub async fn auth_middleware(
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, String> {
    if req.uri().path() == "/register" || req.uri().path() == "/login" {
        return Ok(next.run(req).await);
    }

    // Extract the token from the Authorization header
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok()) //Convert Val. into String 
        .ok_or("Missing token")?;

    //Validate to token
    let user_id = validate_jwt(token).map_err(|_| "Invalid token")?;

    // Attach the user ID to the request for use in handlers
    req.extensions_mut().insert(user_id);

    // Continue to the next middleware/handler
    Ok(next.run(req).await)
}
