use crate::gemini_client::{generate_text, demo_text_embedding, DEMO_EMBED_DIM};
use crate::vector_db::QdrantClient;
use sqlx::MySqlPool;
use anyhow::Result;
use tracing::{info, error};

pub struct FileWorker {
    pool: MySqlPool,
    qdrant: QdrantClient,
}

impl FileWorker {
    pub fn new(pool: MySqlPool) -> Self {
        let qdrant_url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://qdrant:6333".to_string());
        let qdrant = QdrantClient::new(&qdrant_url);
        Self { pool, qdrant }
    }

    pub async fn run(&self) {
        info!("FileWorker starting");
        if let Err(e) = self.qdrant.ensure_files_collection(DEMO_EMBED_DIM).await {
            error!("Failed to ensure Qdrant collection: {}", e);
        }
        loop {
            match self.fetch_and_claim().await {
                Ok(Some(fid)) => {
                    info!("Processing file {}", fid);
                    if let Err(e) = self.process_file(&fid).await {
                        error!("Error processing file {}: {}", fid, e);
                    }
                }
                Ok(None) => {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
                Err(e) => {
                    error!("FileWorker fetch error: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }
        }
    }

    async fn fetch_and_claim(&self) -> Result<Option<String>> {
        // Claim files that are queued or stuck in progress for >10min
        if let Some(row) = sqlx::query(
            "SELECT id FROM files WHERE (analysis_status = 'Queued' OR (analysis_status = 'InProgress' AND created_at < (NOW() - INTERVAL 10 MINUTE))) AND pending_analysis = TRUE LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await? {
            use sqlx::Row;
            let id: String = row.get("id");
            // Mark as in-progress
            let _ = sqlx::query("UPDATE files SET analysis_status = 'InProgress' WHERE id = ?")
                .bind(&id)
                .execute(&self.pool)
                .await?;
            Ok(Some(id))
        } else {
            Ok(None)
        }
    }

    async fn process_file(&self, file_id: &str) -> Result<()> {
        use sqlx::Row;
        let row = sqlx::query("SELECT filename, path FROM files WHERE id = ?")
            .bind(file_id)
            .fetch_one(&self.pool)
            .await?;
        let filename: String = row.get("filename");
        let path: String = row.get("path");

        // Stage 1: Gemini 2.5 Flash for description
        std::env::set_var("GEMINI_MODEL", "gemini-1.5-flash");
        let desc = generate_text(&format!("Describe the file '{filename}' and extract all key components, keywords, and details for later vectorization. Be comprehensive and factual.")).await.unwrap_or_else(|e| format!("[desc error: {}]", e));
        sqlx::query("UPDATE files SET description = ?, analysis_status = 'InProgress' WHERE id = ?")
            .bind(&desc)
            .bind(file_id)
            .execute(&self.pool)
            .await?;

        // Stage 2: Gemini 2.5 Pro for deep vector graph data
        std::env::set_var("GEMINI_MODEL", "gemini-1.5-pro");
        let vector_graph = generate_text(&format!("Given the file '{filename}' and its description: {desc}\nGenerate a set of vector graph data (keywords, use cases, relationships) that can be used for broad and precise search. Only include what is directly supported by the file.")).await.unwrap_or_else(|e| format!("[vector error: {}]", e));

        // Stage 3: Embed and upsert to Qdrant
        let emb = demo_text_embedding(&vector_graph).await?;
        self.qdrant.upsert_point(file_id, emb).await?;

        // Mark file as ready
        sqlx::query("UPDATE files SET pending_analysis = FALSE, analysis_status = 'Completed' WHERE id = ?")
            .bind(file_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
