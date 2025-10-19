mod api;
mod db;
mod file_worker;
mod gemini_client;
mod models;
mod storage;
mod vector;
mod vector_db;
mod worker;

use std::env;
use std::error::Error;
use tracing::{error, info, warn};
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
    let pool = db::init_db(&database_url)
        .await
        .map_err(|e| -> Box<dyn Error> { Box::new(e) })?;

    let auto_import_setting = env::var("AUTO_IMPORT_DEMO").unwrap_or_else(|_| "true".to_string());
    let auto_import = !matches!(
        auto_import_setting.trim().to_ascii_lowercase().as_str(),
        "0" | "false" | "off" | "no"
    );
    if auto_import {
        match api::perform_demo_import(false, &pool).await {
            Ok(summary) => {
                if let Some(err_msg) = summary.error.as_ref() {
                    warn!(error = %err_msg, "startup demo import completed with warnings");
                }
                info!(
                    imported = summary.imported,
                    skipped = summary.skipped,
                    files_found = summary.files_found,
                    source = summary.source_dir.as_deref().unwrap_or("unknown"),
                    "startup demo import completed"
                );
            }
            Err(err) => {
                error!(error = %err, "startup demo import failed");
            }
        }
    } else {
        info!("AUTO_IMPORT_DEMO disabled; skipping startup demo import");
    }

    // Spawn query worker
    let worker = worker::Worker::new(pool.clone());
    tokio::spawn(async move { worker.run().await });

    // Spawn file analysis worker
    let file_worker = file_worker::FileWorker::new(pool.clone());
    tokio::spawn(async move { file_worker.run().await });

    // API routes
    let api_routes = api::routes(pool.clone())
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_headers(vec!["content-type", "authorization"])
                .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]),
        )
        .with(warp::log("rust_engine"));

    info!("Rust Engine started on http://0.0.0.0:8000");

    warp::serve(api_routes).run(([0, 0, 0, 0], 8000)).await;

    Ok(())
}
