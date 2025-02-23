use axum::{
    body::Body, extract::State, http::{Request, StatusCode}, middleware::Next, response::Response  
};

use crate::utils::jwt::validate_token;

use headers::{Authorization, HeaderMapExt};
use sqlx::MySqlPool;

pub async fn auth_middleware(
    State(pool): State<MySqlPool>,
    req: Request<Body>,
    next: Next, 
) -> Result<Response, StatusCode>{
    let headers = req.headers();

    if let Some(auth_header) = headers.typed_get::<Authorization<headers::authorization::Bearer>>() {
        let token = auth_header.token();
        if validate_token(token).is_ok() {
            return Ok(next.run(req).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}