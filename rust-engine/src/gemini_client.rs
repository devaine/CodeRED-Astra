use anyhow::Result;
use serde::Deserialize;

// NOTE: This is a small stub to represent where you'd call the Gemini API.
// Replace with real API call and proper auth handling for production.

#[derive(Debug, Deserialize)]
pub struct GeminiTokenResponse {
    pub token: String,
}

pub async fn generate_token_for_file(_path: &str) -> Result<String> {
    Ok("gemini-token-placeholder".to_string())
}

/// Demo embedding generator - deterministic pseudo-embedding from filename/path
pub fn demo_embedding_from_path(path: &str) -> Vec<f32> {
    // Very simple: hash bytes into a small vector
    let mut v = vec![0f32; 64];
    for (i, b) in path.as_bytes().iter().enumerate() {
        let idx = i % v.len();
        v[idx] += (*b as f32) / 255.0;
    }
    v
}

pub const DEMO_EMBED_DIM: usize = 64;

/// Demo text embedding (replace with real Gemini text embedding API)
pub async fn demo_text_embedding(text: &str) -> Result<Vec<f32>> {
    let mut v = vec![0f32; DEMO_EMBED_DIM];
    for (i, b) in text.as_bytes().iter().enumerate() {
        let idx = i % v.len();
        v[idx] += (*b as f32) / 255.0;
    }
    Ok(v)
}
