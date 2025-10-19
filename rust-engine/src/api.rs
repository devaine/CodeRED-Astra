use crate::gemini_client;
use crate::vector_db::QdrantClient;
use crate::storage;
use anyhow::Result;
use bytes::Buf;
use futures_util::{StreamExt, TryStreamExt};
use serde::Deserialize;
use sqlx::{MySqlPool, Row};
use warp::{multipart::FormData, Filter, Rejection, Reply};

#[derive(Debug, Deserialize)]
struct DeleteQuery {
    id: String,
}

pub fn routes(pool: MySqlPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let pool_filter = warp::any().map(move || pool.clone());

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

    upload.or(delete).or(list).or(create_q).or(status).or(result).or(cancel)
}

async fn handle_upload(mut form: FormData, pool: MySqlPool) -> Result<impl Reply, Rejection> {
    // qdrant client
    let qdrant_url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://qdrant:6333".to_string());
    let qdrant = QdrantClient::new(&qdrant_url);

    while let Some(field) = form.try_next().await.map_err(|_| warp::reject())? {
        let name = field.name().to_string();
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

        // Generate gemini token/description (stub)
        let token = gemini_client::generate_token_for_file(path.to_str().unwrap()).await.map_err(|_| warp::reject())?;

        // Insert file record
        let id = uuid::Uuid::new_v4().to_string();
        let desc = Some(format!("token:{}", token));
        sqlx::query("INSERT INTO files (id, filename, path, description) VALUES (?, ?, ?, ?)")
            .bind(&id)
            .bind(&filename)
            .bind(path.to_str().unwrap())
            .bind(desc)
            .execute(&pool)
            .await
            .map_err(|e| {
                tracing::error!("DB insert error: {}", e);
                warp::reject()
            })?;

        // generate demo embedding and upsert to Qdrant (async best-effort)
        let emb = crate::gemini_client::demo_embedding_from_path(path.to_str().unwrap());
        let qdrant_clone = qdrant.clone();
        let id_clone = id.clone();
        tokio::spawn(async move {
            if let Err(e) = qdrant_clone.upsert_point(&id_clone, emb).await {
                tracing::error!("qdrant upsert failed: {}", e);
            }
        });
    }

    Ok(warp::reply::json(&serde_json::json!({"success": true})))
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
    let _ = sqlx::query("DELETE FROM files WHERE id = ?").bind(&q.id).execute(&pool).await;
        return Ok(warp::reply::json(&serde_json::json!({"deleted": true})));
    }
    Ok(warp::reply::json(&serde_json::json!({"deleted": false})))
}

async fn handle_list(pool: MySqlPool) -> Result<impl Reply, Rejection> {
    let rows = sqlx::query("SELECT id, filename, path, description FROM files ORDER BY created_at DESC LIMIT 500")
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
            serde_json::json!({"id": id, "filename": filename, "path": path, "description": description})
        })
        .collect();

    Ok(warp::reply::json(&serde_json::json!({"files": files})))
}

async fn handle_create_query(body: serde_json::Value, pool: MySqlPool) -> Result<impl Reply, Rejection> {
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
    Ok(warp::reply::json(&serde_json::json!({"status": "not_found"})))
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
