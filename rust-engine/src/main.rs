use std::env;
use warp::Filter;
use sqlx::mysql::MySqlPool;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

#[derive(Debug, Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://astraadmin:password@mysql:3306/astra".to_string());

    info!("Starting Rust Engine...");
    info!("Connecting to database: {}", database_url);

    // Connect to database
    let pool = match MySqlPool::connect(&database_url).await {
        Ok(pool) => {
            info!("Successfully connected to database");
            pool
        }
        Err(e) => {
            warn!("Failed to connect to database: {}. Starting without DB connection.", e);
            // In a hackathon setting, we might want to continue without DB for initial testing
            return start_server_without_db().await;
        }
    };

    // CORS configuration
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type", "authorization"])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]);

    // Health check endpoint
    let health = warp::path("health")
        .and(warp::get())
        .map(|| {
            let response = HealthResponse {
                status: "healthy".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            };
            warp::reply::json(&ApiResponse {
                success: true,
                data: Some(response),
                message: None,
            })
        });

    // API routes - you'll expand these for your hackathon needs
    let api = warp::path("api")
        .and(
            health.or(
                // Add more routes here as needed
                warp::path("version")
                    .and(warp::get())
                    .map(|| {
                        warp::reply::json(&ApiResponse {
                            success: true,
                            data: Some("1.0.0"),
                            message: Some("Rust Engine API".to_string()),
                        })
                    })
            )
        );

    let routes = api
        .with(cors)
        .with(warp::log("rust_engine"));

    info!("Rust Engine started on http://0.0.0.0:8000");

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8000))
        .await;

    Ok(())
}

async fn start_server_without_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting server in DB-less mode for development");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type", "authorization"])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]);

    let health = warp::path("health")
        .and(warp::get())
        .map(|| {
            let response = HealthResponse {
                status: "healthy (no db)".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            };
            warp::reply::json(&ApiResponse {
                success: true,
                data: Some(response),
                message: Some("Running without database connection".to_string()),
            })
        });

    let routes = warp::path("api")
        .and(health)
        .with(cors)
        .with(warp::log("rust_engine"));

    info!("Rust Engine started on http://0.0.0.0:8000 (DB-less mode)");

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8000))
        .await;

    Ok(())
}