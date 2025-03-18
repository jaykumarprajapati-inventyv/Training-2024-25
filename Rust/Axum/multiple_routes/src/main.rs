use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
mod routing_methods;

use routing_methods::{
    get_method::method_of_get, path_variables::get_id, post_method::method_of_post,query_parameters::query_parameters
};

#[tokio::main]
async fn main() {
    
    let app: Router = Router::new()
        .route("/", get(method_of_get()))
        .route("/post-method", post(method_of_post))
        .route("/get-id/{id}", get(get_id)).route("/query-params", get(query_parameters));

    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);

    
    let listener = TcpListener::bind(addr).await.unwrap();

    
    axum::serve(listener, app).await.unwrap();
}
