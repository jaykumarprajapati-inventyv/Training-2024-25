use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod custom_headers;

#[tokio::main]
async fn main() {
    let app: Router = Router::new().route("/standard-headers", get(custom_headers::custom_header));

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener: TcpListener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
