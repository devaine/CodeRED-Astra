use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use reqwest::Client;

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

/// Generate text with Gemini (Generative Language API). Falls back to a demo string if GEMINI_API_KEY is not set.
pub async fn generate_text(prompt: &str) -> Result<String> {
    let api_key = match std::env::var("GEMINI_API_KEY") {
        Ok(k) if !k.is_empty() => k,
        _ => {
            return Ok(format!("[demo] Gemini not configured. Prompt preview: {}", truncate(prompt, 240)));
        }
    };

    let model = std::env::var("GEMINI_MODEL").unwrap_or_else(|_| "gemini-1.5-pro".to_string());
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );

    let body = json!({
        "contents": [ { "parts": [ { "text": prompt } ] } ]
    });

    let client = Client::new();
    let resp = client.post(&url).json(&body).send().await?;
    let status = resp.status();
    let txt = resp.text().await?;
    if !status.is_success() {
        return Ok(format!("[demo] Gemini error {}: {}", status, truncate(&txt, 240)));
    }

    #[derive(Deserialize)]
    struct Part { text: Option<String> }
    #[derive(Deserialize)]
    struct Content { parts: Vec<Part> }
    #[derive(Deserialize)]
    struct Candidate { content: Content }
    #[derive(Deserialize)]
    struct Response { candidates: Option<Vec<Candidate>> }

    let data: Response = serde_json::from_str(&txt).unwrap_or(Response { candidates: None });
    let out = data
        .candidates
        .and_then(|mut v| v.pop())
        .and_then(|c| c.content.parts.into_iter().find_map(|p| p.text))
        .unwrap_or_else(|| "[demo] Gemini returned empty response".to_string());
    Ok(out)
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max { s.to_string() } else { format!("{}…", &s[..max]) }
}
