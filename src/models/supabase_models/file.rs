use crate::types::{FileStatus, TaskType};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct File {
    pub document_id: String,
    pub file_name: String,
    pub file_size: i64,
    pub task_type: TaskType,
    pub mime_type: String,
    pub status: FileStatus,
    pub status_message: Option<String>,
    pub user_id: String,
    pub processing_attempts: i32,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Default for File {
    fn default() -> Self {
        Self {
            document_id: "".to_string(),
            file_name: "".to_string(),
            file_size: 0,
            task_type: TaskType::StudyAbroadAgreement,
            mime_type: "".to_string(),
            status: FileStatus::Declined,
            status_message: None,
            user_id: "".to_owned(),
            processing_attempts: 0,
            created_at: Local::now(),
            updated_at: Local::now(),
        }
    }
}
