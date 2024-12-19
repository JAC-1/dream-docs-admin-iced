use crate::types::ActiveStatus;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    pub(crate) id: Uuid,
    pub(crate) display_id: String,
    pub(crate) display_name: String,
    pub(crate) class: String,
    pub(crate) program: String,
    pub(crate) status: ActiveStatus,
    pub(crate) last_login_at: Option<DateTime<Local>>,
    pub(crate) login_count: i32,
    pub(crate) created_at: DateTime<Local>,
    pub(crate) updated_at: Option<DateTime<Local>>,
}

impl Default for Student {
    fn default() -> Student {
        let fake_id = Uuid::new_v4();
        Student {
            id: fake_id,
            display_id: "STUDENT123".to_string(),
            display_name: "Test Student".to_string(),
            class: "Test Class".to_string(),
            program: "Test Program".to_string(),
            status: ActiveStatus::Active,
            last_login_at: None,
            login_count: 0,
            created_at: Local::now(),
            updated_at: None,
        }
    }
}
