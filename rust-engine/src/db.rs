use sqlx::{MySql, MySqlPool};
use tracing::info;

pub async fn init_db(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    let pool = MySqlPool::connect(database_url).await?;

    // Create tables if they don't exist. Simple schema for demo/hackathon use.
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS files (
            id VARCHAR(36) PRIMARY KEY,
            filename TEXT NOT NULL,
            path TEXT NOT NULL,
            description TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS queries (
            id VARCHAR(36) PRIMARY KEY,
            status VARCHAR(32) NOT NULL,
            payload JSON,
            result JSON,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
        );
        "#,
    )
    .execute(&pool)
    .await?;

    info!("Database initialized");
    Ok(pool)
}
