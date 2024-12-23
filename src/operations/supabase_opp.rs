use crate::models::supabase_models::*;
use anyhow::{anyhow, Result};
use dotenv::dotenv;
use postgrest::Postgrest;

// let table_headers = ["学籍番号", "表示名", "クラス", "プログラム", "書類完了"];

// TODO: add url and key to struct
pub struct SupabaseQuery {
    client: Postgrest,
}

impl SupabaseQuery {
    pub fn new() -> Self {
        dotenv().ok();
        let key = std::env::var("SUPABASE_KEY").expect("Problem loading database key");
        let mut url = std::env::var("SUPABASE_URL").expect("Problem loading database URL");
        if !url.contains("/rest/v1") {
            url = format!("{}/rest/v1", url);
        }

        let client = Postgrest::new(url).insert_header("apiKey", key);
        Self { client }
    }

    pub async fn all_students_info(&self) -> Result<Vec<StudentProfileData>> {
        let query = self
            .client
            .from("students")
            .select("display_id,display_name,classes(title),programs(name)")
            .order("id")
            .execute()
            .await?;
        let query_text = query
            .text()
            .await
            .map_err(|e| anyhow!("Failed to get text response from query: {}", e))?;
        let raw_students_data: Vec<StudentProfileData> = serde_json::from_str(&query_text)?;
        Ok(raw_students_data)
    }
}

// #[allow(dead_code)]
// async fn add_student(display_name: String, display_id: String) -> Result<()> {
//     dotenv().ok();
//     let url = std::env::var("SUPABASE_URL")?;
//     let key = std::env::var("SUPABASE_KEY")?;
//     let client = Postgrest::new(url).insert_header("apiKey", key.as_str());
//     let super_class = client
//         .from("classes")
//         .eq("title", "Super Class")
//         .select("id")
//         .order("id")
//         .execute()
//         .await?;
//     let super_class_json: serde_json::Value = serde_json::from_str(&super_class.text().await?)?;
//     let super_class_id = String::from(super_class_json[0]["id"].as_str().unwrap());

//     let new_student = Student {
//         id: Uuid::new_v4(),
//         display_id,
//         display_name,
//         class: String::from(super_class_id),
//         program: String::from("540ea150-3b34-445a-9851-d2e9000a6299"),
//         status: ActiveStatus::Active,
//         last_login_at: None,
//         login_count: 0,
//         created_at: chrono::Local::now(),
//         updated_at: None,
//     };
//     let json_string = serde_json::to_string(&new_student)?;
//     let raw_response = client
//         .from("students")
//         .insert(json_string)
//         .execute()
//         .await?;
//     let status = raw_response.status();
//     match status.is_success() {
//         true => {
//             // Did you know you can use serde_json::Value to check the response and see the types?
//             let success_response: Vec<Student> = serde_json::from_str(&raw_response.text().await?)?;
//             println!("{:#?}", success_response);
//             Ok(())
//         }
//         false => {
//             let error_response: InsertErrorResponse =
//                 serde_json::from_str(&raw_response.text().await?)?;
//             Err(anyhow!(
//                 "Error inserting student {}",
//                 error_response.message
//             ))
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_student_info() -> Result<()> {
        let supabase = SupabaseQuery::new();
        let _ = supabase.all_students_info().await?;
        Ok(())
    }
}
