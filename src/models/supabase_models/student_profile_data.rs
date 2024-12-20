use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentProfileData {
    pub display_id: String,
    pub display_name: String,
    pub class: String,
    pub program: String,
}
