use crate::storage;
use crate::vector_db::QdrantClient;
use anyhow::Result;
use bytes::Buf;
use futures_util::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, Row};
use tracing::{debug, info, warn};
use warp::{multipart::FormData, Filter, Rejection, Reply};

#[derive(Debug, Deserialize)]
struct DeleteQuery {
    id: String,
}

#[derive(Debug, Serialize)]
pub struct DemoImportSummary {
    pub imported: usize,
    pub skipped: usize,
    pub files_found: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_dir: Option<String>,
    pub attempted_paths: Vec<String>,
    pub force: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub async fn perform_demo_import(force: bool, pool: &MySqlPool) -> Result<DemoImportSummary> {
    use anyhow::Context;
    use std::fs;
    use std::path::PathBuf;

    let demo_dir_setting =
        std::env::var("DEMO_DATA_DIR").unwrap_or_else(|_| "demo-data".to_string());
    info!(force, requested_dir = %demo_dir_setting, "demo import requested");

    let base = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    // Build a list of plausible demo-data locations so local runs and containers both work.
    let mut candidates: Vec<PathBuf> = Vec::new();
    let configured = PathBuf::from(&demo_dir_setting);
    let mut push_candidate = |path: PathBuf| {
        if !candidates.iter().any(|existing| existing == &path) {
            candidates.push(path);
        }
    };

    push_candidate(base.join(&configured));
    push_candidate(PathBuf::from(&demo_dir_setting));
    push_candidate(base.join("rust-engine").join(&configured));
    push_candidate(base.join("rust-engine").join("demo-data"));
    push_candidate(base.join("demo-data"));
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            push_candidate(exe_dir.join(&configured));
            push_candidate(exe_dir.join("demo-data"));
            push_candidate(exe_dir.join("rust-engine").join(&configured));
        }
    }

    let mut attempted: Vec<PathBuf> = Vec::new();
    let mut resolved_dir: Option<PathBuf> = None;
    for candidate in candidates {
        debug!(candidate = %candidate.display(), "evaluating demo import path candidate");
        if candidate.exists() && candidate.is_dir() {
            resolved_dir = Some(candidate.clone());
            break;
        }
        attempted.push(candidate);
    }

    let attempted_paths: Vec<String> = attempted.iter().map(|p| p.display().to_string()).collect();
    let mut summary = DemoImportSummary {
        imported: 0,
        skipped: 0,
        files_found: 0,
        source_dir: resolved_dir.as_ref().map(|p| p.display().to_string()),
        attempted_paths,
        force,
        error: None,
    };

    let src_dir = match resolved_dir {
        Some(path) => path,
        None => {
            summary.error =
                Some("demo dir not found; set DEMO_DATA_DIR or bind mount demo PDFs".to_string());
            warn!(
                attempted_paths = ?summary.attempted_paths,
                "demo import skipped; source directory not found"
            );
            return Ok(summary);
        }
    };

    summary.source_dir = Some(src_dir.display().to_string());
    info!(
        source = %summary.source_dir.as_deref().unwrap_or_default(),
        attempted_paths = ?summary.attempted_paths,
        "demo import source resolved"
    );

    for entry in fs::read_dir(&src_dir).with_context(|| format!("reading {:?}", src_dir))? {
        let entry = entry.with_context(|| format!("reading entry in {:?}", src_dir))?;
        let path = entry.path();
        if path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("pdf"))
            .unwrap_or(false)
        {
            summary.files_found += 1;
            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown.pdf")
                .to_string();

            if !force {
                if let Some(_) = sqlx::query("SELECT id FROM files WHERE filename = ?")
                    .bind(&filename)
                    .fetch_optional(pool)
                    .await?
                {
                    summary.skipped += 1;
                    info!(%filename, "skipping demo import; already present");
                    continue;
                }
            }

            let data = fs::read(&path).with_context(|| format!("reading {:?}", path))?;
            let stored_path = storage::save_file(&filename, &data)?;
            info!(%filename, dest = %stored_path.to_string_lossy(), "demo file copied to storage");

            let id = uuid::Uuid::new_v4().to_string();

            if force {
                sqlx::query("DELETE FROM files WHERE filename = ?")
                    .bind(&filename)
                    .execute(pool)
                    .await?;
                info!(%filename, "existing file records removed due to force import");
            }

            sqlx::query("INSERT INTO files (id, filename, path, description, pending_analysis, analysis_status) VALUES (?, ?, ?, ?, ?, 'Queued')")
                .bind(&id)
                .bind(&filename)
                .bind(stored_path.to_string_lossy().to_string())
                .bind(Option::<String>::None)
                .bind(true)
                .execute(pool)
                .await?;
            info!(%filename, file_id = %id, "demo file inserted into database");

            sqlx::query("INSERT INTO file_jobs (file_id, status) VALUES (?, 'Queued')")
                .bind(&id)
                .execute(pool)
                .await?;
            info!(%filename, file_id = %id, "demo file queued for analysis");

            summary.imported += 1;
        } else {
            debug!(path = %path.display(), "skipping non-PDF entry during demo import");
        }
    }

    let source_label = summary.source_dir.as_deref().unwrap_or("unknown");

    if summary.files_found == 0 {
        warn!(source = %source_label, "demo import located zero PDFs");
    }

    info!(
        source = %source_label,
        files_found = summary.files_found,
        attempted_paths = ?summary.attempted_paths,
        imported = summary.imported,
        skipped = summary.skipped,
        force,
        "demo import completed"
    );

    Ok(summary)
}

pub fn routes(pool: MySqlPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let pool_filter = warp::any().map(move || pool.clone());

    // Import demo files from demo-data directory
    let import_demo = warp::path!("files" / "import-demo")
        .and(warp::post())
        .and(
            warp::query::<std::collections::HashMap<String, String>>()
                .or(warp::any().map(|| std::collections::HashMap::new()))
                .unify(),
        )
        .and(pool_filter.clone())
        .and_then(handle_import_demo);

    // Upload file
    let upload = warp::path("files")
        .and(warp::post())
        .and(warp::multipart::form().max_length(50_000_000)) // 50MB per part default; storage is filesystem-backed
        .and(pool_filter.clone())
        .and_then(handle_upload);

    // Delete file
    let delete = warp::path!("files" / "delete")
        .and(warp::get())
        .and(warp::query::<DeleteQuery>())
        .and(pool_filter.clone())
        .and_then(handle_delete);

    // List files
    let list = warp::path!("files" / "list")
        .and(warp::get())
        .and(pool_filter.clone())
        .and_then(handle_list);

    // Create query
    let create_q = warp::path!("query" / "create")
        .and(warp::post())
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(handle_create_query);

    // Query status
    let status = warp::path!("query" / "status")
        .and(warp::get())
        .and(warp::query::<DeleteQuery>())
        .and(pool_filter.clone())
        .and_then(handle_query_status);

    // Query result
    let result = warp::path!("query" / "result")
        .and(warp::get())
        .and(warp::query::<DeleteQuery>())
        .and(pool_filter.clone())
        .and_then(handle_query_result);

    // Cancel
    let cancel = warp::path!("query" / "cancel")
        .and(warp::get())
        .and(warp::query::<DeleteQuery>())
        .and(pool_filter.clone())
        .and_then(handle_cancel_query);

    let api = upload
        .or(import_demo)
        .or(delete)
        .or(list)
        .or(create_q)
        .or(status)
        .or(result)
        .or(cancel);
    warp::path("api").and(api)
}

async fn handle_upload(mut form: FormData, pool: MySqlPool) -> Result<impl Reply, Rejection> {
    let mut created_files = Vec::new();
    while let Some(field) = form.try_next().await.map_err(|_| warp::reject())? {
        let _name = field.name().to_string();
        let filename = field
            .filename()
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("upload-{}", uuid::Uuid::new_v4()));

        // Read stream of Buf into a Vec<u8>
        let data = field
            .stream()
            .map_ok(|mut buf| {
                let mut v = Vec::new();
                while buf.has_remaining() {
                    let chunk = buf.chunk();
                    v.extend_from_slice(chunk);
                    let n = chunk.len();
                    buf.advance(n);
                }
                v
            })
            .try_fold(Vec::new(), |mut acc, chunk_vec| async move {
                acc.extend_from_slice(&chunk_vec);
                Ok(acc)
            })
            .await
            .map_err(|_| warp::reject())?;

        // Save file
        let path = storage::save_file(&filename, &data).map_err(|_| warp::reject())?;

        // Insert file record with pending_analysis = true, description = NULL
        let id = uuid::Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO files (id, filename, path, description, pending_analysis, analysis_status) VALUES (?, ?, ?, ?, ?, 'Queued')")
            .bind(&id)
            .bind(&filename)
            .bind(path.to_str().unwrap())
            .bind(Option::<String>::None)
            .bind(true)
            .execute(&pool)
            .await
            .map_err(|e| {
                tracing::error!("DB insert error: {}", e);
                warp::reject()
            })?;
        created_files.push(serde_json::json!({
            "id": id,
            "filename": filename,
            "pending_analysis": true,
            "analysis_status": "Queued"
        }));
    }

    Ok(warp::reply::json(&serde_json::json!({
        "uploaded": created_files.len(),
        "files": created_files
    })))
}

async fn handle_import_demo(
    params: std::collections::HashMap<String, String>,
    pool: MySqlPool,
) -> Result<impl Reply, Rejection> {
    let force = params
        .get("force")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    match perform_demo_import(force, &pool).await {
        Ok(summary) => Ok(warp::reply::json(&summary)),
        Err(err) => {
            tracing::error!(error = %err, "demo import failed");
            let fallback = DemoImportSummary {
                imported: 0,
                skipped: 0,
                files_found: 0,
                source_dir: None,
                attempted_paths: Vec::new(),
                force,
                error: Some(err.to_string()),
            };
            Ok(warp::reply::json(&fallback))
        }
    }
}

async fn handle_delete(q: DeleteQuery, pool: MySqlPool) -> Result<impl Reply, Rejection> {
    if let Some(row) = sqlx::query("SELECT path FROM files WHERE id = ?")
        .bind(&q.id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| warp::reject())?
    {
        let path: String = row.get("path");
        let _ = storage::delete_file(std::path::Path::new(&path));
        // Remove from Qdrant
        let qdrant_url =
            std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://qdrant:6333".to_string());
        let qdrant = QdrantClient::new(&qdrant_url);
        let _ = qdrant.delete_point(&q.id).await;
        let _ = sqlx::query("DELETE FROM files WHERE id = ?")
            .bind(&q.id)
            .execute(&pool)
            .await;
        return Ok(warp::reply::json(&serde_json::json!({"deleted": true})));
    }
    Ok(warp::reply::json(&serde_json::json!({"deleted": false})))
}

async fn handle_list(pool: MySqlPool) -> Result<impl Reply, Rejection> {
    let rows = sqlx::query("SELECT id, filename, path, description, pending_analysis, analysis_status FROM files ORDER BY created_at DESC LIMIT 500")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!("DB list error: {}", e);
            warp::reject()
        })?;
    let files: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|r| {
            let id: String = r.get("id");
            let filename: String = r.get("filename");
            let path: String = r.get("path");
            let description: Option<String> = r.get("description");
            let pending: bool = r.get("pending_analysis");
            let status: Option<String> = r.try_get("analysis_status").ok();
            let storage_url = format!("/storage/{}", filename);
            serde_json::json!({
                "id": id,
                "filename": filename,
                "path": path,
                "storage_url": storage_url,
                "description": description,
                "pending_analysis": pending,
                "analysis_status": status
            })
        })
        .collect();

    Ok(warp::reply::json(&serde_json::json!({"files": files})))
}

async fn handle_create_query(
    body: serde_json::Value,
    pool: MySqlPool,
) -> Result<impl Reply, Rejection> {
    // Insert query as queued, worker will pick it up
    let id = uuid::Uuid::new_v4().to_string();
    let payload = body;
    sqlx::query("INSERT INTO queries (id, status, payload) VALUES (?, 'Queued', ?)")
        .bind(&id)
        .bind(payload)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("DB insert query error: {}", e);
            warp::reject()
        })?;

    Ok(warp::reply::json(&serde_json::json!({"id": id})))
}

async fn handle_query_status(q: DeleteQuery, pool: MySqlPool) -> Result<impl Reply, Rejection> {
    if let Some(row) = sqlx::query("SELECT status FROM queries WHERE id = ?")
        .bind(&q.id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| warp::reject())?
    {
        let status: String = row.get("status");
        return Ok(warp::reply::json(&serde_json::json!({"status": status})));
    }
    Ok(warp::reply::json(
        &serde_json::json!({"status": "not_found"}),
    ))
}

async fn handle_query_result(q: DeleteQuery, pool: MySqlPool) -> Result<impl Reply, Rejection> {
    if let Some(row) = sqlx::query("SELECT result FROM queries WHERE id = ?")
        .bind(&q.id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| warp::reject())?
    {
        let result: Option<serde_json::Value> = row.get("result");
        return Ok(warp::reply::json(&serde_json::json!({"result": result})));
    }
    Ok(warp::reply::json(&serde_json::json!({"result": null})))
}

async fn handle_cancel_query(q: DeleteQuery, pool: MySqlPool) -> Result<impl Reply, Rejection> {
    // Mark as cancelled; worker must check status before heavy steps
    sqlx::query("UPDATE queries SET status = 'Cancelled' WHERE id = ?")
        .bind(&q.id)
        .execute(&pool)
        .await
        .map_err(|_| warp::reject())?;
    Ok(warp::reply::json(&serde_json::json!({"cancelled": true})))
}
