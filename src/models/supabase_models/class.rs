use crate::types::ActiveStatus;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    id: String,
    year: i32,
    title: String,
    description: Option<String>,
    created_at: DateTime<Local>,
    status: ActiveStatus,
}
