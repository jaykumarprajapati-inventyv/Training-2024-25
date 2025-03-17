use axum::{
    body::Body,
    extract::{FromRequest, Request},
    http::StatusCode,
    routing::post,
    Json, Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RequestData {
    #[validate(email(message = "Email should be valid!!"))]
    email: String,
    #[validate(length(min = 8, message = "Atleast 8 character should be.!!"))]
    password: String,
}

impl<S> FromRequest<S, Body> for RequestData
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request<Body>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<RequestData>::from_request(req, _state)
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid JSON format".to_string()))?;

        if let Err(errors) = data.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
        }
        Ok(data)
    }
}

#[tokio::main]
async fn main() {
    let app: Router<_> = Router::new().route("/custom-extract", post(my_custom_extractor));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn my_custom_extractor(userdata: RequestData) {
    dbg!(userdata);
}
