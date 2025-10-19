use crate::gemini_client::{demo_text_embedding, DEMO_EMBED_DIM, generate_text};
use crate::models::{QueryRecord, QueryStatus};
use crate::vector_db::QdrantClient;
use anyhow::Result;
use sqlx::MySqlPool;
use std::time::Duration;
use tracing::{error, info};

pub struct Worker {
    pool: MySqlPool,
    qdrant: QdrantClient,
}

impl Worker {
    pub fn new(pool: MySqlPool) -> Self {
        let qdrant_url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://qdrant:6333".to_string());
        let qdrant = QdrantClient::new(&qdrant_url);
        Self { pool, qdrant }
    }

    pub async fn run(&self) {
        info!("Worker starting");

        // Ensure qdrant collection exists
        if let Err(e) = self.qdrant.ensure_files_collection(DEMO_EMBED_DIM).await {
            error!("Failed to ensure Qdrant collection: {}", e);
        }

        // Requeue stale InProgress jobs older than cutoff (e.g., 10 minutes)
        if let Err(e) = self.requeue_stale_inprogress(10 * 60).await {
            error!("Failed to requeue stale jobs: {}", e);
        }

        loop {
            // Claim next queued query
            match self.fetch_and_claim().await {
                Ok(Some(mut q)) => {
                    info!("Processing query {}", q.id);
                    if let Err(e) = self.process_query(&mut q).await {
                        error!("Error processing {}: {}", q.id, e);
                        let _ = self.mark_failed(&q.id, &format!("{}", e)).await;
                    }
                }
                Ok(None) => {
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
                Err(e) => {
                    error!("Worker fetch error: {}", e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    async fn fetch_and_claim(&self) -> Result<Option<QueryRecord>> {
        // Note: MySQL transactional SELECT FOR UPDATE handling is more complex; for this hackathon scaffold
        // we do a simple two-step: select one queued id, then update it to InProgress if it is still queued.
        if let Some(row) = sqlx::query("SELECT id, payload FROM queries WHERE status = 'Queued' ORDER BY created_at LIMIT 1")
            .fetch_optional(&self.pool)
            .await?
        {
            use sqlx::Row;
            let id: String = row.get("id");
            let payload: serde_json::Value = row.get("payload");

            let updated = sqlx::query("UPDATE queries SET status = 'InProgress' WHERE id = ? AND status = 'Queued'")
                .bind(&id)
                .execute(&self.pool)
                .await?;

            if updated.rows_affected() == 1 {
                let mut q = QueryRecord::new(payload);
                q.id = id;
                q.status = QueryStatus::InProgress;
                return Ok(Some(q));
            }
        }
        Ok(None)
    }

    async fn process_query(&self, q: &mut QueryRecord) -> Result<()> {
        // Stage 1: set InProgress (idempotent)
        self.update_status(&q.id, QueryStatus::InProgress).await?;

        // Stage 2: embed query text
        let text = q.payload.get("q").and_then(|v| v.as_str()).unwrap_or("");
    let emb = demo_text_embedding(text).await?;
    let top_k = q.payload.get("top_k").and_then(|v| v.as_u64()).unwrap_or(5) as usize;

        // Check cancellation
        if self.is_cancelled(&q.id).await? { return Ok(()); }

        // Stage 3: search top-K in Qdrant
    let hits = self.qdrant.search_top_k(emb, top_k).await.unwrap_or_default();
    let top_ids: Vec<String> = hits.iter().map(|(id, _)| id.clone()).collect();

        // Check cancellation
        if self.is_cancelled(&q.id).await? { return Ok(()); }

        // Stage 4: fetch file metadata for IDs
        let mut files_json = Vec::new();
        for (fid, score) in hits {
            if let Some(row) = sqlx::query("SELECT id, filename, path, description FROM files WHERE id = ? AND pending_analysis = FALSE")
                .bind(&fid)
                .fetch_optional(&self.pool)
                .await? {
                use sqlx::Row;
                let id: String = row.get("id");
                let filename: String = row.get("filename");
                let path: String = row.get("path");
                let description: Option<String> = row.get("description");
                files_json.push(serde_json::json!({
                    "id": id, "filename": filename, "path": path, "description": description, "score": score
                }));
            }
        }

        // Stage 5: call Gemini to analyze relationships and propose follow-up details strictly from provided files
        let relationships_prompt = build_relationships_prompt(text, &files_json);
        let relationships = generate_text(&relationships_prompt).await.unwrap_or_else(|e| format!("[demo] relationships error: {}", e));

        // Stage 6: final answer synthesis with strict constraints (no speculation; say unknown when insufficient)
        let final_prompt = build_final_answer_prompt(text, &files_json, &relationships);
        let final_answer = generate_text(&final_prompt).await.unwrap_or_else(|e| format!("[demo] final answer error: {}", e));

        // Stage 7: persist results
        let result = serde_json::json!({
            "summary": format!("Found {} related files", files_json.len()),
            "related_files": files_json,
            "relationships": relationships,
            "final_answer": final_answer,
        });
        sqlx::query("UPDATE queries SET status = 'Completed', result = ? WHERE id = ?")
            .bind(result)
            .bind(&q.id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn update_status(&self, id: &str, status: QueryStatus) -> Result<()> {
        let s = match status {
            QueryStatus::Queued => "Queued",
            QueryStatus::InProgress => "InProgress",
            QueryStatus::Completed => "Completed",
            QueryStatus::Cancelled => "Cancelled",
            QueryStatus::Failed => "Failed",
        };
        sqlx::query("UPDATE queries SET status = ? WHERE id = ?")
            .bind(s)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn mark_failed(&self, id: &str, message: &str) -> Result<()> {
        let result = serde_json::json!({"error": message});
        sqlx::query("UPDATE queries SET status = 'Failed', result = ? WHERE id = ?")
            .bind(result)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn requeue_stale_inprogress(&self, age_secs: i64) -> Result<()> {
        // MySQL: requeue items updated_at < now()-age and status = InProgress
        sqlx::query(
            "UPDATE queries SET status = 'Queued' WHERE status = 'InProgress' AND updated_at < (NOW() - INTERVAL ? SECOND)"
        )
        .bind(age_secs)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn is_cancelled(&self, id: &str) -> Result<bool> {
        if let Some(row) = sqlx::query("SELECT status FROM queries WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
        {
            use sqlx::Row;
            let s: String = row.get("status");
            return Ok(s == "Cancelled");
        }
        Ok(false)
    }
}

fn build_relationships_prompt(query: &str, files: &Vec<serde_json::Value>) -> String {
    let files_snippets: Vec<String> = files.iter().map(|f| format!(
        "- id: {id}, filename: {name}, path: {path}, desc: {desc}",
        id=f.get("id").and_then(|v| v.as_str()).unwrap_or(""),
        name=f.get("filename").and_then(|v| v.as_str()).unwrap_or(""),
        path=f.get("path").and_then(|v| v.as_str()).unwrap_or(""),
        desc=f.get("description").and_then(|v| v.as_str()).unwrap_or("")
    )).collect();
    format!(
        "You are an assistant analyzing relationships STRICTLY within the provided files.\n\
        Query: {query}\n\
        Files:\n{files}\n\
        Tasks:\n\
        1) Summarize key details from the files relevant to the query.\n\
        2) Describe relationships and linkages strictly supported by these files.\n\
        3) List important follow-up questions that could be answered only using the provided files.\n\
        Rules: Do NOT guess or invent. If information is insufficient in the files, explicitly state that.",
        query=query,
        files=files_snippets.join("\n")
    )
}

fn build_final_answer_prompt(query: &str, files: &Vec<serde_json::Value>, relationships: &str) -> String {
    let files_short: Vec<String> = files.iter().map(|f| format!(
        "- {name} ({id})",
        id=f.get("id").and_then(|v| v.as_str()).unwrap_or(""),
        name=f.get("filename").and_then(|v| v.as_str()).unwrap_or("")
    )).collect();
    format!(
        "You are to compose a final answer to the user query using only the information from the files.\n\
        Query: {query}\n\
        Files considered:\n{files}\n\
        Relationship analysis:\n{rels}\n\
        Requirements:\n\
        - Use only information present in the files and analysis above.\n\
        - If the answer is uncertain or cannot be determined from the files, clearly state that limitation.\n\
        - Avoid speculation or assumptions.\n\
        Provide a concise, structured answer.",
        query=query,
        files=files_short.join("\n"),
        rels=relationships
    )
}
