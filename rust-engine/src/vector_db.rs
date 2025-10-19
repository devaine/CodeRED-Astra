use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

#[derive(Clone)]
pub struct QdrantClient {
    base: String,
    client: Client,
}

impl QdrantClient {
    /// Delete a point from collection 'files' by id
    pub async fn delete_point(&self, id: &str) -> Result<()> {
        let url = format!("{}/collections/files/points/delete", self.base);
        let body = json!({
            "points": [id]
        });
        let resp = self.client.post(&url).json(&body).send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let t = resp.text().await.unwrap_or_default();
            Err(anyhow::anyhow!("qdrant delete failed: {} - {}", status, t))
        }
    }
    pub fn new(base: &str) -> Self {
        Self {
            base: base.trim_end_matches('/').to_string(),
            client: Client::new(),
        }
    }

    /// Upsert a point into collection `files` with id and vector
    pub async fn upsert_point(&self, id: &str, vector: Vec<f32>) -> Result<()> {
        let url = format!("{}/collections/files/points", self.base);
        let body = json!({
            "points": [{
                "id": id,
                "vector": vector,
                "payload": {"type": "file"}
            }]
        });

        let resp = self.client.post(&url).json(&body).send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let t = resp.text().await.unwrap_or_default();
            Err(anyhow::anyhow!("qdrant upsert failed: {} - {}", status, t))
        }
    }

    /// Ensure the 'files' collection exists with the given dimension and distance metric
    pub async fn ensure_files_collection(&self, dim: usize) -> Result<()> {
        let url = format!("{}/collections/files", self.base);
        let body = json!({
            "vectors": {"size": dim, "distance": "Cosine"}
        });
        let resp = self.client.put(&url).json(&body).send().await?;
        // 200 OK or 201 Created means ready; 409 Conflict means already exists
        if resp.status().is_success() || resp.status().as_u16() == 409 {
            Ok(())
        } else {
            let status = resp.status();
            let t = resp.text().await.unwrap_or_default();
            Err(anyhow::anyhow!(
                "qdrant ensure collection failed: {} - {}",
                status,
                t
            ))
        }
    }

    /// Search top-k nearest points from 'files', return (id, score)
    pub async fn search_top_k(&self, vector: Vec<f32>, k: usize) -> Result<Vec<(String, f32)>> {
        let url = format!("{}/collections/files/points/search", self.base);
        let body = json!({
            "vector": vector,
            "limit": k
        });
        let resp = self.client.post(&url).json(&body).send().await?;
        let status = resp.status();
        if !status.is_success() {
            let t = resp.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("qdrant search failed: {} - {}", status, t));
        }
        #[derive(Deserialize)]
        struct Hit {
            id: serde_json::Value,
            score: f32,
        }
        #[derive(Deserialize)]
        struct Data {
            result: Vec<Hit>,
        }
        let data: Data = resp.json().await?;
        let mut out = Vec::new();
        for h in data.result {
            // id can be string or number; handle string
            if let Some(s) = h.id.as_str() {
                out.push((s.to_string(), h.score));
            } else {
                out.push((h.id.to_string(), h.score));
            }
        }
        Ok(out)
    }
}
