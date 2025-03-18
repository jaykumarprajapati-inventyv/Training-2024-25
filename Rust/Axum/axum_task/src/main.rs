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
    tokens: Arc<Mutex<HashMap<String, String>>>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

   
    let pool = connection::connect_to_db().await;
    DB.set(pool.clone()).unwrap();

    utils::env::init_secret_key();

   
    let initial_token = std::env::var("SECRET_KEY").expect("SECRET_KEY not found in .env");

   
    let state = Arc::new(AppState {
        tokens: Arc::new(Mutex::new(HashMap::from([(
            "session".to_string(),
            initial_token.clone(),
        )]))),
    });

   
    let database_routes = Router::new()
        .route("/register", post(registration::register_to_user))
        .route("/login", post(login::login_authentication))
        .route("/addlaptop", post(product::product_assign))
        .with_state(pool.clone());

   
    let secure_routes = Router::new()
        .route("/secure", get(secure_handler))
        .with_state(state.clone());

    let app = Router::new()
        .merge(database_routes)
        .merge(secure_routes)
        .route("/", get(get_func::get_method));

   
    let address = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("🚀 Server running at http://{}", address);

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}



async fn secure_handler(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> Result<Json<HashMap<&'static str, String>>, StatusCode> {
    let token = headers.get("Authorization").and_then(|h| h.to_str().ok());

    let mut tokens = state.tokens.lock().await;

    if let Some(token) = token {
        if let Some(_) = tokens.remove(&token.to_string()) { 
            let new_token = generate_token();
            tokens.insert(new_token.clone(), new_token.clone()); 

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
