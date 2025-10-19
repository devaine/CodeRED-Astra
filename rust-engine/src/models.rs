use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileRecord {
    pub id: String,
    pub filename: String,
    pub path: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub pending_analysis: bool, // true if file is not yet ready for search
    pub analysis_status: String, // 'Queued', 'InProgress', 'Completed', 'Failed'
}

impl FileRecord {
    pub fn new(filename: impl Into<String>, path: impl Into<String>, description: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            filename: filename.into(),
            path: path.into(),
            description,
            created_at: None,
            pending_analysis: true,
            analysis_status: "Queued".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum QueryStatus {
    Queued,
    InProgress,
    Completed,
    Cancelled,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryRecord {
    pub id: String,
    pub status: QueryStatus,
    pub payload: serde_json::Value,
    pub result: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl QueryRecord {
    pub fn new(payload: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            status: QueryStatus::Queued,
            payload,
            result: None,
            created_at: None,
            updated_at: None,
        }
    }
}
