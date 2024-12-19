use crate::types::ActiveStatus;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    id: String,
    name: String,
    description: Option<String>,
    location: String,
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate,
    duration: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    status: ActiveStatus,
}
