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
            TaskType::StudyAbroadAgreement => "Study_Abroad_Agreement",
            TaskType::PersonalInformationWaiver => "Personal_Information_Waiver",
            TaskType::WhyStudyInCanada => "Why_I_want_to_Study_in_Canada",
            TaskType::HomestayLetter => "Homestay_Letter",
            TaskType::Passport => "Passport",
            TaskType::Headshot => "Headshot",
            TaskType::StudyAbroadApplication => "Study_Abroad_Application",
            TaskType::ImmunizationRecord => "Immunization_Record",
            TaskType::FamilyImages => "Family_Images",
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
