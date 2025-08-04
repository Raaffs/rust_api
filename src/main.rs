mod routes;
mod handlers;
mod models;
mod repository;

use std::sync::Arc;
use repository::pg_repository::PostgresContactRepository;
use repository::ContactRepository;
use tokio::net::TcpListener;
use axum::serve;
use sqlx::PgPool;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Could not connect to Postgres");

    let repo = Arc::new(PostgresContactRepository::new(pool)) as Arc<dyn ContactRepository>;

    let app = routes::create_routes(repo);

    let addr = "127.0.0.1:3000";
    println!("Server running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app.into_make_service()).await.unwrap();

    Ok(())
}
