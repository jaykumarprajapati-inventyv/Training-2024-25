use std::net::SocketAddr;

use axum::{routing::post, Json, Router};
use serde::Deserialize;
use tokio::net::TcpListener;
#[derive(Deserialize, Debug)]

pub struct LoginCredentials {
    username: String,
    password: String,
    email:Option<String> //If we're not passing "email" though it'll sucsess because its option
}
#[tokio::main]
async fn main() {
    // println!("Hello, world!");
    let app: Router<_> = Router::new().route("/validate_to_serde", post(validation));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

//Deserialize to upcoming Serialize data. Json -> Rust Struct.

async fn validation(Json(user): Json<LoginCredentials>) {
    dbg!(user);
}
