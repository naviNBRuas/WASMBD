use axum::{routing::get, Router};
use std::net::SocketAddr;
use axum::serve;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(handler));

    // run it with hyper on `localhost:3000`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn handler() -> String {
    "Hello, Controller!".to_string()
}