use axum::{extract::Json, http::Method};
use axum::{middleware, Extension};
use axum::{
    routing::{get, post},
    Router,
};
use http::HeaderMap;
use read_custom_header_for_middleware::read_custom_header;
use serde_json::Value;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod read_custom_header_for_middleware;
mod set_custom_middleware_header;
#[derive(Clone)]
pub struct ShareData {
    message: String,
}
#[tokio::main]

async fn main() {
    //Instance of Struct
    let shared_data = ShareData {
        message: "Helo from Shared Middleware Data.".to_owned(),
    };

    //Declared CorsLayer
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app: Router<_> = Router::new()
        .route(
            "/read-custom-header-for-middleware",
            get(read_custom_header),
        )
        .route_layer(middleware::from_fn(
            set_custom_middleware_header::my_custom_middleware,
        ))
        .route("/get", get(get_data()))
        .route("/post", post(post_to_data))
        .route("/headers", get(custom_headers))
        .route("/middleware-message", get(middleware_method))
        .layer(Extension(shared_data))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn get_data() -> String {
    "I'm from get method".to_string()
}

async fn post_to_data(Json(body): Json<Value>) -> Json<Value> {
    Json(body)
}

async fn custom_headers(header: HeaderMap) -> String {
    let msg_val = header.get("x-message").unwrap();
    let msg = msg_val.to_str().unwrap().to_owned();
    msg
}

//This method is global so every handlers can access,it is commonly which shared state globally.
async fn middleware_method(Extension(shared_data): Extension<ShareData>) -> String {
    shared_data.message
}
