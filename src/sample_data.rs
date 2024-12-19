use crate::models::supabase_models::Student;
use crate::types::ActiveStatus;
use chrono::Local;
use std::collections::HashMap;
use uuid::Uuid;

#[allow(dead_code)]
pub fn get_sample_data() -> HashMap<&'static str, Student> {
    let mut rows = HashMap::new();
    rows.insert(
        "1",
        Student {
            id: Uuid::new_v4(),
            class: "Amazing Class".to_string(),
            program: "Amazing Program".to_string(),
            display_id: "山田太郎".to_string(),
            display_name: "Taro Yamada".to_string(),
            status: ActiveStatus::Active,
            login_count: 0,
            last_login_at: None,
            updated_at: None,
            created_at: Local::now(),
        },
    );
    rows
}
