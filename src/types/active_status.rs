use serde::Deserializer;

#[derive(Debug)]
pub enum ActiveStatus {
    Active,
    Inactive,
}

impl ActiveStatus {
    fn to_str(&self) -> &str {
        match self {
            ActiveStatus::Active => "active",
            ActiveStatus::Inactive => "inactive",
        }
    }

    fn from_str(s: &str) -> Option<ActiveStatus> {
        match s {
            "active" => Some(ActiveStatus::Active),
            "inactive" => Some(ActiveStatus::Inactive),
            _ => None,
        }
    }
}
impl serde::Serialize for ActiveStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_str())
    }
}

impl<'de> serde::Deserialize<'de> for ActiveStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ActiveStatus::from_str(&s).ok_or_else(|| {
            serde::de::Error::custom(format!(
                "invalid ActiveStatus: {}. Must be 'active' or 'inactive'",
                s
            ))
        })
    }
}
