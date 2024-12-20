use crate::models::supabase_models::*;
use crate::types::ActiveStatus;
use anyhow::{anyhow, Result};
use dotenv::dotenv;
use postgrest::Postgrest;
use uuid::Uuid;

// let table_headers = ["学籍番号", "表示名", "クラス", "プログラム", "書類完了"];

pub struct SupabaseQuery;

impl SupabaseQuery {
    fn check_for_valid_url(url: String) -> String {
        if !url.contains("/rest/v1") {
            format!("{}/rest/v1", url)
        } else {
            url
        }
    }

    pub async fn all_students_info() -> Result<Vec<StudentProfileData>> {
        dotenv().ok();
        let url = std::env::var("SUPABASE_URL")?;
        let url = SupabaseQuery::check_for_valid_url(url);
        let key = std::env::var("SUPABASE_KEY")?;
        let client = Postgrest::new(url).insert_header("apiKey", key.as_str());
        let query = client
            .from("students")
            .select("display_id,display_name,class,program")
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

// async fn all_student_data() -> Result<StudentData> {}

#[allow(dead_code)]
async fn all_table_data() -> Result<QueryableResponse> {
    dotenv().ok();
    let url = std::env::var("SUPABASE_URL")?;
    let key = std::env::var("SUPABASE_KEY")?;

    let client = Postgrest::new(url).insert_header("apiKey", key.as_str());
    let classes = client
        .from("classes")
        .select("*")
        .order("id")
        .execute()
        .await?;
    let file_cache = client
        .from("file_cache")
        .select("*")
        .order("id")
        .execute()
        .await?;
    let file_keys = client
        .from("file_keys")
        .select("*")
        .order("id")
        .execute()
        .await?;
    let programs = client
        .from("programs")
        .select("*")
        .order("id")
        .execute()
        .await?;
    let downloads_cache = client
        .from("downloads_cache")
        .select("*")
        .order("id")
        .execute()
        .await?;
    let students = client
        .from("students")
        .select("*")
        .order("id")
        .execute()
        .await?;

    let response_data: QueryableResponse = QueryableResponse {
        classes: serde_json::from_str(&classes.text().await?)
            .map_err(|e| anyhow!("Error parsing classes: {:?}", e))?,
        file_cache: serde_json::from_str(&file_cache.text().await?)
            .map_err(|e| anyhow!("Error parsing file_cache: {:?}", e))?,
        file_keys: serde_json::from_str(&file_keys.text().await?)
            .map_err(|e| anyhow!("Error parsing file_keys: {:?}", e))?,
        programs: serde_json::from_str(&programs.text().await?)
            .map_err(|e| anyhow!("Error parsing programs: {:?}", e))?,
        downloads_cache: serde_json::from_str(&downloads_cache.text().await?)
            .map_err(|e| anyhow!("Error parsing downloads_cache: {:?}", e))?,
        students: serde_json::from_str(&students.text().await?)
            .map_err(|e| anyhow!("Error parsing students: {:?}", e))?,
    };

    Ok(response_data)
}

#[allow(dead_code)]
async fn add_student(display_name: String, display_id: String) -> Result<()> {
    dotenv().ok();
    let url = std::env::var("SUPABASE_URL")?;
    let key = std::env::var("SUPABASE_KEY")?;
    let client = Postgrest::new(url).insert_header("apiKey", key.as_str());
    let super_class = client
        .from("classes")
        .eq("title", "Super Class")
        .select("id")
        .order("id")
        .execute()
        .await?;
    let super_class_json: serde_json::Value = serde_json::from_str(&super_class.text().await?)?;
    let super_class_id = String::from(super_class_json[0]["id"].as_str().unwrap());

    let new_student = Student {
        id: Uuid::new_v4(),
        display_id,
        display_name,
        class: String::from(super_class_id),
        program: String::from("540ea150-3b34-445a-9851-d2e9000a6299"),
        status: ActiveStatus::Active,
        last_login_at: None,
        login_count: 0,
        created_at: chrono::Local::now(),
        updated_at: None,
    };
    let json_string = serde_json::to_string(&new_student)?;
    let raw_response = client
        .from("students")
        .insert(json_string)
        .execute()
        .await?;
    let status = raw_response.status();
    match status.is_success() {
        true => {
            // Did you know you can use serde_json::Value to check the response and see the types?
            let success_response: Vec<Student> = serde_json::from_str(&raw_response.text().await?)?;
            println!("{:#?}", success_response);
            Ok(())
        }
        false => {
            let error_response: InsertErrorResponse =
                serde_json::from_str(&raw_response.text().await?)?;
            Err(anyhow!(
                "Error inserting student {}",
                error_response.message
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_all_table_data() -> Result<()> {
        let result = all_table_data().await?;
        Ok(assert!(result.classes.len() > 0))
    }

    #[tokio::test]
    async fn test_student_info() -> Result<()> {
        let _ = SupabaseQuery::all_students_info();
        Ok(())
    }
    // #[tokio::test]
    // async fn test_add_random_student() -> Result<()> {
    //     let name = "Test Student".to_string();
    //     let display_id = format!("{}-{}", "test", uuid::Uuid::new_v4().to_string());
    //     add_student(name, display_id).await?;
    //     Ok(())
    // }

    // #[tokio::test]
    // async fn test_add_student_returns_error() -> Result<()> {
    //     let name = "Test Student".to_string();
    //     let display_id = "maybe".to_string();
    //     assert!(add_student(name, display_id).await.is_err());
    //     Ok(())
    // }
}
