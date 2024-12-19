use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileKey {
    id: String,
    document_id: String,
    encrypted_key: String,
    status: String,
    last_accessed_at: Option<DateTime<Local>>,
    created_at: DateTime<Local>,
    rotated_at: Option<DateTime<Local>>,
    expires_at: Option<DateTime<Local>>,
    revoked_at: Option<DateTime<Local>>,
}
