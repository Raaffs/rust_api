mod routes;
mod handlers;
mod models;
mod repository;

use std::sync::Arc;
use repository::mock_repository::MockContactRepository;
use repository::ContactRepository;
use tokio::net::TcpListener;
use axum::serve;

#[tokio::main]
async fn main() {
    let repo = Arc::new(MockContactRepository::new()) as Arc<dyn ContactRepository>;
    let app = routes::create_routes(repo);

    let addr = "127.0.0.1:3000";
    println!("Server running on {}", addr);

    // New style: use TcpListener + axum::serve
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}
