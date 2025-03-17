use axum::middleware;
use axum::routing::get;
use axum::{routing::post, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
mod middlewares;
mod models;
mod router;
use handlers::{registeration::register_user, user_login::login};
mod handlers;
mod jwt_utils;

/* For database */
use crate::handlers::protected_handler::protected_handler;
use crate::middlewares::auth_middleware::auth_middleware;
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Connect to TiDB
    let pool = MySqlPool::connect(&std::env::var("DATABASE_URL").expect("Database URL is wrong!!"))
        .await
        .expect("Failed to connect database");

    let shared_pool = Arc::new(Mutex::new(pool));

    let app: Router<_> = Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login))
        .route("/dashboard", get(user_dashboard_handler))
        .route_layer(middleware::from_fn(auth_middleware))
        .with_state(shared_pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
