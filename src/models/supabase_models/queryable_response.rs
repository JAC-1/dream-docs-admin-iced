use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryableResponse {
    pub(crate) classes: Vec<Class>,
    pub(crate) file_cache: Vec<File>,
    pub(crate) file_keys: Vec<FileKey>,
    pub(crate) programs: Vec<Program>,
    pub(crate) downloads_cache: Vec<DownloadCacheEntry>,
    pub(crate) students: Vec<Student>,
}
