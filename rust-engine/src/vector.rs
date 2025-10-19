use anyhow::Result;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref VECTOR_STORE: Mutex<HashMap<String, Vec<f32>>> = Mutex::new(HashMap::new());
}

pub fn store_embedding(id: &str, emb: Vec<f32>) -> Result<()> {
    let mut s = VECTOR_STORE.lock().unwrap();
    s.insert(id.to_string(), emb);
    Ok(())
}

pub fn query_top_k(_query_emb: &[f32], k: usize) -> Result<Vec<String>> {
    // Very naive: return up to k ids from the store.
    let s = VECTOR_STORE.lock().unwrap();
    let mut out = Vec::new();
    for key in s.keys().take(k) {
        out.push(key.clone());
    }
    Ok(out)
}
