use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use tokio::net::TcpListener;

#[tokio::main]

async fn main() {
    let app: Router<_> = Router::new().route("/custom-statuscode", post(custom_statuscode));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let receiver = TcpListener::bind(addr).await.unwrap();

    axum::serve(receiver, app).await.unwrap();
}

async fn custom_statuscode() -> Response {
    (StatusCode::CREATED, "This is a 201 status code".to_owned()).into_response()
}
