use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct StudentProfileData {
    pub display_id: String,
    pub display_name: String,
    pub classes: HashMap<String, String>,
    pub programs: HashMap<String, String>,
}
