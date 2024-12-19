use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum FileStatus {
    New,
    Pending,
    Approved,
    Declined,
}

impl Serialize for FileStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_str())
    }
}

impl<'de> Deserialize<'de> for FileStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FileStatus::from_str(&s)
            .ok_or_else(|| serde::de::Error::custom(format!("invalid FileStatus: {}", s)))
    }
}

impl FileStatus {
    pub fn from_str(s: &str) -> Option<FileStatus> {
        match s {
            "new" => Some(FileStatus::New),
            "pending" => Some(FileStatus::Pending),
            "approved" => Some(FileStatus::Approved),
            "declined" => Some(FileStatus::Declined),
            _ => None,
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            FileStatus::New => "new",
            FileStatus::Pending => "pending",
            FileStatus::Approved => "approved",
            FileStatus::Declined => "declined",
        }
    }
}
