use crate::gemini_client::{demo_text_embedding, generate_text_with_model, DEMO_EMBED_DIM};
use crate::vector;
use crate::vector_db::QdrantClient;
use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use pdf_extract::extract_text;
use sqlx::MySqlPool;
use std::path::PathBuf;
use tracing::{error, info, warn};

pub struct FileWorker {
    pool: MySqlPool,
    qdrant: QdrantClient,
}

impl FileWorker {
    pub fn new(pool: MySqlPool) -> Self {
        let qdrant_url =
            std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://qdrant:6333".to_string());
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
                        if let Err(mark_err) = self.mark_failed(&fid, &format!("{}", e)).await {
                            error!("Failed to mark file {} as failed: {}", fid, mark_err);
                        }
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

        let (file_excerpt, truncated) = match extract_file_excerpt(&path).await {
            Ok(res) => res,
            Err(err) => {
                error!(file_id, %filename, %path, error = ?err, "failed to extract text from file; continuing with filename only");
                (String::new(), false)
            }
        };
        if file_excerpt.is_empty() {
            warn!(file_id, %filename, %path, "extracted excerpt is empty; prompts may lack context");
        }

        let (raw_base64, raw_truncated) = match read_file_base64(&path).await {
            Ok(tuple) => tuple,
            Err(err) => {
                warn!(file_id, %filename, %path, error = ?err, "failed to read raw file bytes for prompt");
                (String::new(), false)
            }
        };

        let excerpt_note = if truncated {
            "(excerpt truncated for prompt size)"
        } else {
            ""
        };

        let raw_note = if raw_truncated {
            "(base64 truncated to first 512KB)"
        } else {
            "(base64)"
        };

        // Stage 1: Gemini 2.5 Flash for description
        let mut desc_prompt = format!(
            "You are reviewing the PDF file '{filename}'. Use the following extracted text {excerpt_note} to produce a concise, factual description and key highlights that will help downstream search and reasoning.\n\n--- BEGIN EXCERPT ---\n{}\n--- END EXCERPT ---",
            file_excerpt
        );
        if !raw_base64.is_empty() {
            desc_prompt.push_str(&format!(
                "\n\n--- BEGIN RAW FILE {raw_note} ---\n{}\n--- END RAW FILE ---",
                raw_base64
            ));
        }
        let desc = generate_text_with_model("gemini-2.5-flash", &desc_prompt)
            .await
            .unwrap_or_else(|e| format!("[desc error: {}]", e));
        sqlx::query(
            "UPDATE files SET description = ?, analysis_status = 'InProgress' WHERE id = ?",
        )
        .bind(&desc)
        .bind(file_id)
        .execute(&self.pool)
        .await?;

        // Stage 2: Gemini 2.5 Pro for deep vector graph data
        let mut vector_prompt = format!(
            "You are constructing vector search metadata for the PDF file '{filename}'.\nCurrent description: {desc}\nUse the extracted text {excerpt_note} below to derive precise keywords, thematic clusters, and relationships that are explicitly supported by the content. Provide richly structured bullet points grouped by themes.\n\n--- BEGIN EXCERPT ---\n{}\n--- END EXCERPT ---",
            file_excerpt
        );
        if !raw_base64.is_empty() {
            vector_prompt.push_str(&format!(
                "\n\n--- BEGIN RAW FILE {raw_note} ---\n{}\n--- END RAW FILE ---",
                raw_base64
            ));
        }
        let vector_graph = generate_text_with_model("gemini-2.5-pro", &vector_prompt)
            .await
            .unwrap_or_else(|e| format!("[vector error: {}]", e));

        // Stage 3: Embed and upsert to Qdrant
        let emb = demo_text_embedding(&vector_graph).await?;
        match self.qdrant.upsert_point(file_id, emb.clone()).await {
            Ok(_) => {
                let _ = vector::store_embedding(file_id, emb.clone());
            }
            Err(err) => {
                error!("Qdrant upsert failed for {}: {}", file_id, err);
                let _ = vector::store_embedding(file_id, emb);
            }
        }

        // Mark file as ready
        sqlx::query(
            "UPDATE files SET pending_analysis = FALSE, analysis_status = 'Completed' WHERE id = ?",
        )
        .bind(file_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn mark_failed(&self, file_id: &str, reason: &str) -> Result<()> {
        sqlx::query(
            "UPDATE files SET analysis_status = 'Failed', pending_analysis = TRUE WHERE id = ?",
        )
        .bind(file_id)
        .execute(&self.pool)
        .await?;
        sqlx::query("UPDATE files SET description = COALESCE(description, ?) WHERE id = ?")
            .bind(format!("[analysis failed: {}]", reason))
            .bind(file_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

// Maximum number of characters from the extracted text to include in prompts.
const MAX_EXCERPT_CHARS: usize = 4000;
const MAX_RAW_BYTES: usize = 512 * 1024; // limit base64 payload fed into prompts

async fn extract_file_excerpt(path: &str) -> Result<(String, bool)> {
    let path_buf = PathBuf::from(path);
    let extension = path_buf
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_default();

    let raw_text = if extension == "pdf" {
        let pdf_path = path_buf.clone();
        tokio::task::spawn_blocking(move || extract_text(&pdf_path))
            .await
            .map_err(|e| anyhow!("pdf text extraction task panicked: {e}"))??
    } else {
        let bytes = tokio::fs::read(&path_buf)
            .await
            .with_context(|| format!("reading file bytes from {path}"))?;
        String::from_utf8_lossy(&bytes).into_owned()
    };

    let cleaned = raw_text.replace('\r', "");
    let condensed = collapse_whitespace(&cleaned);
    let (excerpt, truncated) = truncate_to_chars(&condensed, MAX_EXCERPT_CHARS);

    Ok((excerpt, truncated))
}

fn truncate_to_chars(text: &str, max_chars: usize) -> (String, bool) {
    if max_chars == 0 {
        return (String::new(), !text.is_empty());
    }

    let mut result = String::new();
    let mut chars = text.chars();
    for _ in 0..max_chars {
        match chars.next() {
            Some(ch) => result.push(ch),
            None => return (result, false),
        }
    }

    if chars.next().is_some() {
        result.push('…');
        (result, true)
    } else {
        (result, false)
    }
}

fn collapse_whitespace(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut prev_was_ws = false;
    for ch in input.chars() {
        if ch.is_whitespace() {
            if !prev_was_ws {
                output.push(' ');
            }
            prev_was_ws = true;
        } else {
            prev_was_ws = false;
            output.push(ch);
        }
    }
    output.trim().to_string()
}

async fn read_file_base64(path: &str) -> Result<(String, bool)> {
    let bytes = tokio::fs::read(path).await?;
    if bytes.is_empty() {
        return Ok((String::new(), false));
    }
    let truncated = bytes.len() > MAX_RAW_BYTES;
    let slice = if truncated {
        &bytes[..MAX_RAW_BYTES]
    } else {
        &bytes[..]
    };
    let encoded = BASE64_STANDARD.encode(slice);
    Ok((encoded, truncated))
}
