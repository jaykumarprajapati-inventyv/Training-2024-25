// mod database;
// mod models;
// mod router;
// mod utils;

// use axum::{
//     routing::{get, post},
//     Router,
// };
// use std::net::SocketAddr;
// use tokio::net::TcpListener;

// use database::connection;
// use router::{get_func, login, product, registration};
// use sqlx::MySqlPool;
// use tokio::sync::OnceCell;
// static DB: OnceCell<MySqlPool> = OnceCell::const_new();
// use axum::middleware;
// mod middlewares;
// use crate::middlewares::auth::auth_middleware;

// #[tokio::main]
// async fn main() {
//     dotenvy::dotenv().ok();

//     // Connect to TiDB
//     let pool = connection::connect_to_db().await;
//     DB.set(pool.clone()).unwrap();

//     utils::env::init_secret_key(); // Ensure SECRET_KEY exists in .env

//     // Load the secret key
//     let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY not found in .env");
//     println!("Loaded Secret Key: {}", secret_key);

//     let app: Router = Router::new()
//         .route("/", get(get_func::get_method))
//         .route("/register", post(registration::register_to_user))
//         .route("/login", post(login::login_authentication))
//         .route("/addlaptop", post(product::product_assign)).layer(middleware::from_fn_with_state(pool.clone(), auth_middleware))
//         .with_state(pool);

//     let address = SocketAddr::from(([0, 0, 0, 0], 3001));

//     let listner = TcpListener::bind(address).await.unwrap();

//     axum::serve(listner, app).await.unwrap();
// }

// // pub async fn get_method() -> &'static str {
//   //  "Hello Axum and Rust."
// //}

mod database;
mod models;
mod router;
mod utils;

use axum::{
    extract::State,
    http::HeaderMap,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::{Mutex, OnceCell};

use axum::serve;
use rand::{rng, Rng};
use sqlx::MySqlPool;

use database::connection;
use router::{get_func, login, product, registration};

static DB: OnceCell<MySqlPool> = OnceCell::const_new();

#[derive(Clone)]
struct AppState {
    tokens: Arc<Mutex<HashMap<String, String>>>, // Store tokens dynamically
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Connect to TiDB
    let pool = connection::connect_to_db().await;
    DB.set(pool.clone()).unwrap();

    utils::env::init_secret_key(); // Ensure SECRET_KEY exists in .env

    // Load initial token from .env
    let initial_token = std::env::var("SECRET_KEY").expect("SECRET_KEY not found in .env");

    // Shared state to store tokens
    let state = Arc::new(AppState {
        tokens: Arc::new(Mutex::new(HashMap::from([(
            "session".to_string(),
            initial_token.clone(),
        )]))),
    });

    // Routes that require the database connection
    let database_routes = Router::new()
        .route("/register", post(registration::register_to_user))
        .route("/login", post(login::login_authentication))
        .route("/addlaptop", post(product::product_assign))
        .with_state(pool.clone()); // ✅ Uses MySQL pool

    // Routes that require token authentication
    let secure_routes = Router::new()
        .route("/secure", get(secure_handler))
        .with_state(state.clone()); // ✅ Uses token state

    let app = Router::new()
        .merge(database_routes)
        .merge(secure_routes)
        .route("/", get(get_func::get_method));

    // Start server
    let address = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("🚀 Server running at http://{}", address);

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}

// Secure route with token validation
// async fn secure_handler(
//     headers: HeaderMap,
//     State(state): State<Arc<AppState>>,
// ) -> Result<Json<HashMap<&'static str, String>>, StatusCode> {
//     let token = headers.get("Authorization").and_then(|h| h.to_str().ok());

//     let mut tokens = state.tokens.lock().await;

//     if let Some(token) = token {
//         if tokens.values().any(|v| v == token) {
//             // Generate new unique token
//             let new_token = generate_token();
//             tokens.insert("session".to_string(), new_token.clone());

//             let mut response = HashMap::new();
//             response.insert("message", "Request successful".to_string());
//             response.insert("new_token", new_token);

//             return Ok(Json(response));
//         }
//     }

//     Err(StatusCode::UNAUTHORIZED)
// }

async fn secure_handler(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> Result<Json<HashMap<&'static str, String>>, StatusCode> {
    let token = headers.get("Authorization").and_then(|h| h.to_str().ok());

    let mut tokens = state.tokens.lock().await;

    if let Some(token) = token {
        if let Some(_) = tokens.remove(&token.to_string()) { // Remove old token if exists
            let new_token = generate_token();
            tokens.insert(new_token.clone(), new_token.clone()); // Store new token

            let mut response = HashMap::new();
            response.insert("message", "Request successful".to_string());
            response.insert("new_token", new_token);

            return Ok(Json(response));
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}



fn generate_token() -> String {
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                    abcdefghijklmnopqrstuvwxyz\
                    0123456789";
    let mut rng = rand::thread_rng();
    
    (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}
