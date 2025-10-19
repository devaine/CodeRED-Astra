use anyhow::Result;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn storage_dir() -> PathBuf {
    std::env::var("ASTRA_STORAGE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/app/storage"))
}

pub fn ensure_storage_dir() -> Result<()> {
    let dir = storage_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(())
}

pub fn save_file(filename: &str, contents: &[u8]) -> Result<PathBuf> {
    ensure_storage_dir()?;
    let mut path = storage_dir();
    path.push(filename);
    let mut f = fs::File::create(&path)?;
    f.write_all(contents)?;
    Ok(path)
}

pub fn delete_file(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}
