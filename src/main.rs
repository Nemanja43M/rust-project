use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use sqlx::PgPool;
use std::net::SocketAddr;
use dotenvy::dotenv;
use std::env;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenv().ok();

    // Connect to the database
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Unable to connect to the database");

    // Configure Axum router
    let app = Router::new()
        .route("/", get(root))
        .layer(Extension(pool));

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Starting server at address {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "API is running!"
}
