use crate::types::{FileStatus, TaskType};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct File {
    id: String,
    document_id: String,
    file_path: String,
    file_name: String,
    file_size: i64,
    task_type: TaskType,
    mime_type: String,
    status: FileStatus,
    status_message: Option<String>,
    user_id: String,
    processing_attempts: i32,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}
