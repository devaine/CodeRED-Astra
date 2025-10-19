use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

// NOTE: This file provides lightweight helpers around the Gemini API. For the
// hackathon demo we fall back to deterministic strings when the API key is not
// configured so the flows still work end-to-end.

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

/// Generate text using the default model (GEMINI_MODEL or gemini-2.5-pro).
#[allow(dead_code)]
pub async fn generate_text(prompt: &str) -> Result<String> {
    let model = std::env::var("GEMINI_MODEL").unwrap_or_else(|_| "gemini-2.5-pro".to_string());
    generate_text_with_model(&model, prompt).await
}

/// Generate text with an explicit Gemini model. Falls back to a deterministic
/// response when the API key is not set so the demo still runs.
pub async fn generate_text_with_model(model: &str, prompt: &str) -> Result<String> {
    let api_key = match std::env::var("GEMINI_API_KEY") {
        Ok(k) if !k.is_empty() => k,
        _ => {
            return Ok(format!(
                "[demo] Gemini ({}) not configured. Prompt preview: {}",
                model,
                truncate(prompt, 240)
            ));
        }
    };

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
        return Ok(format!(
            "[demo] Gemini ({}) error {}: {}",
            model,
            status,
            truncate(&txt, 240)
        ));
    }

    #[derive(Deserialize)]
    struct Part {
        text: Option<String>,
    }
    #[derive(Deserialize)]
    struct Content {
        parts: Vec<Part>,
    }
    #[derive(Deserialize)]
    struct Candidate {
        content: Content,
    }
    #[derive(Deserialize)]
    struct Response {
        candidates: Option<Vec<Candidate>>,
    }

    let data: Response = serde_json::from_str(&txt).unwrap_or(Response { candidates: None });
    let out = data
        .candidates
        .and_then(|mut v| v.pop())
        .and_then(|c| c.content.parts.into_iter().find_map(|p| p.text))
        .unwrap_or_else(|| "[demo] Gemini returned empty response".to_string());
    Ok(out)
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max])
    }
}
