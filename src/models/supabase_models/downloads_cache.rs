use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadCacheEntry {
    id: String,
    document_id: String,
    downloaded_at: DateTime<Local>,
}
