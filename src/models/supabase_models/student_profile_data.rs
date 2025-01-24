use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct StudentProfileData {
    pub display_id: String,
    pub display_name: String,
    pub classes: HashMap<String, String>,
    pub programs: HashMap<String, String>,
}

impl Default for StudentProfileData {
    fn default() -> Self {
        StudentProfileData {
            display_id: String::new(),
            display_name: String::new(),
            classes: HashMap::new(),
            programs: HashMap::new(),
        }
    }
}
