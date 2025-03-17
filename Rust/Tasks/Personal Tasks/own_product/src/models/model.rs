use axum::{
    body::Body,
    extract::{FromRequest, Request},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]

pub struct User {
    pub username: String,
    #[validate(email(message = "Email should be valid format!!"))]
    pub email: String,
    #[validate(length(min = 8, message = "Atleast 8 character should be.!!"))]
    pub password: String,
}

impl<S> FromRequest<S, Body> for User
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request<Body>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<User>::from_request(req, _state)
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid JSON format".to_string()))?;

        if let Err(errors) = data.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
        }
        Ok(data)
    }
}
