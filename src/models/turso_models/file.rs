use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct EncryptedFile {
    pub id: i64,
    pub external_doc_id: String,
    pub processing_attempts: i64,
    pub file: String,
}
