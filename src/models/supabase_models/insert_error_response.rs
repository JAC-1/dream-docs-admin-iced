use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertErrorResponse {
    code: String,
    details: String,
    hint: Option<String>,
    pub(crate) message: String,
}
