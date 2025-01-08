use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileKey {
    pub id: String,
    pub document_id: String,
    pub encrypted_key: String,
    pub status: String,
    pub last_accessed_at: Option<DateTime<Local>>,
    pub created_at: DateTime<Local>,
    pub rotated_at: Option<DateTime<Local>>,
    pub expires_at: Option<DateTime<Local>>,
    pub revoked_at: Option<DateTime<Local>>,
}
