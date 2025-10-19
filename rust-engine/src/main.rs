mod api;
mod db;
mod gemini_client;
mod models;
mod storage;
mod vector;
mod worker;
mod vector_db;

use std::env;
use std::error::Error;
use tracing::info;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://astraadmin:password@mysql:3306/astra".to_string());

    info!("Starting Rust Engine...");

    // Ensure storage dir
    storage::ensure_storage_dir().expect("storage dir");

    // Initialize DB
    let pool = db::init_db(&database_url).await.map_err(|e| -> Box<dyn Error> { Box::new(e) })?;

    // Spawn worker
    let worker = worker::Worker::new(pool.clone());
    tokio::spawn(async move { worker.run().await });

    // API routes
    let api_routes = api::routes(pool.clone())
        .with(warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type", "authorization"])
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]))
        .with(warp::log("rust_engine"));

    info!("Rust Engine started on http://0.0.0.0:8000");

    warp::serve(api_routes)
        .run(([0, 0, 0, 0], 8000))
        .await;

    Ok(())
}