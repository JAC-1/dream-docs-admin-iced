use serde::Deserializer;

#[derive(Debug, Clone)]
pub enum TaskType {
    StudyAbroadAgreement,
    PersonalInformationWaiver,
    WhyStudyInCanada,
    HomestayLetter,
    Passport,
    Headshot,
    StudyAbroadApplication,
    ImmunizationRecord,
    FamilyImages,
}

impl TaskType {
    fn from_str(s: &str) -> Option<TaskType> {
        match s {
            "Study_Abroad_Agreement" => Some(TaskType::StudyAbroadAgreement),
            "Personal_Information_Waiver" => Some(TaskType::PersonalInformationWaiver),
            "Why_Study_In_Canada" => Some(TaskType::WhyStudyInCanada),
            "Homestay_Letter" => Some(TaskType::HomestayLetter),
            "Passport" => Some(TaskType::Passport),
            "Headshot" => Some(TaskType::Headshot),
            "Study_Abroad_Application" => Some(TaskType::StudyAbroadApplication),
            "Immunization_Record" => Some(TaskType::ImmunizationRecord),
            "Family_Images" => Some(TaskType::FamilyImages),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            TaskType::StudyAbroadAgreement => "Study Abroad Agreement",
            TaskType::PersonalInformationWaiver => "Personal Information Waiver",
            TaskType::WhyStudyInCanada => "Why I want to Study in Canada",
            TaskType::HomestayLetter => "Homestay Letter",
            TaskType::Passport => "Passport",
            TaskType::Headshot => "Headshot",
            TaskType::StudyAbroadApplication => "Study Abroad Application",
            TaskType::ImmunizationRecord => "Immunization Record",
            TaskType::FamilyImages => "Family Images",
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            TaskType::StudyAbroadAgreement => "Study Abroad Agreement".to_string(),
            TaskType::PersonalInformationWaiver => "Personal Information Waiver".to_string(),
            TaskType::WhyStudyInCanada => "Why I want to Study in Canada".to_string(),
            TaskType::HomestayLetter => "Homestay_Letter".to_string(),
            TaskType::Passport => "Passport".to_string(),
            TaskType::Headshot => "Headshot".to_string(),
            TaskType::StudyAbroadApplication => "Study Abroad Application".to_string(),
            TaskType::ImmunizationRecord => "Immunization Record".to_string(),
            TaskType::FamilyImages => "Family Images".to_string(),
        }
    }
}

impl serde::Serialize for TaskType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_str())
    }
}

impl<'de> serde::Deserialize<'de> for TaskType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        TaskType::from_str(&s)
            .ok_or_else(|| serde::de::Error::custom(format!("Error converting TaskType to String. Got {}, expected one of: Study_Abroad_Agreement, Personal_Information_Waiver, Why_Study_In_Canada, Homestay_Letter, Passport, Headshot, Study_Abroad_Application, Immunization_Record, Family_Images . ", s)))
    }
}
