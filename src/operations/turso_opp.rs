use crate::models::turso_models::*;
use libsql::{de, Builder, Connection, Database};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Clone)]
pub struct TursoQuery {
    // Use Arc<Mutex<>> to allow cloning and shared mutability
    db: Arc<Mutex<Option<Result<Database, String>>>>,
    env_hashmap: Option<HashMap<String, String>>,
}

impl TursoQuery {
    pub fn new_sync(env_hashmap: Option<HashMap<String, String>>) -> Self {
        TursoQuery {
            db: Arc::new(Mutex::new(None)),
            env_hashmap,
        }
    }

    pub async fn connect(&self) -> Result<(), String> {
        // Check if already connected
        {
            let db = self
                .db
                .lock()
                .map_err(|e| format!("Mutex lock error: {}", e))?;
            if db.is_some() {
                return Ok(());
            }
        }

        let env_map = self
            .env_hashmap
            .as_ref()
            .ok_or_else(|| "Environment hashmap required".to_string())?;

        let url = env_map
            .get("TURSO_DB_URL")
            .ok_or_else(|| "TURSO_DB_URL expected but none found.".to_string())?
            .clone();

        let token = env_map
            .get("TURSO_AUTH_TOKEN")
            .ok_or_else(|| "TURSO_AUTH_TOKEN expected but none found.".to_string())?
            .clone();

        let builder = Builder::new_remote(url, token)
            .build()
            .await
            .map_err(|e| format!("Failed to build database connection: {}", e))?;

        // Now update the mutex-protected database
        let mut db = self
            .db
            .lock()
            .map_err(|e| format!("Mutex lock error: {}", e))?;
        *db = Some(Ok(builder));

        Ok(())
    }

    async fn get_connection(&self) -> Result<Connection, String> {
        self.connect().await?;

        // Access the database through the mutex
        let db_guard = self
            .db
            .lock()
            .map_err(|e| format!("Mutex lock error: {}", e))?;

        // Safely access the database
        let db = match &*db_guard {
            Some(Ok(db)) => db,
            Some(Err(e)) => return Err(format!("Database error: {}", e)),
            None => return Err("Database not initialized".to_string()),
        };

        let conn = db
            .connect()
            .map_err(|e| format!("Failed to connect to turso database: {}", e))?;

        Ok(conn)
    }

    pub async fn get_file(&self, doc_id: String) -> Result<String, String> {
        let conn = self.get_connection().await?;
        let mut rows = conn
            .query(
                "SELECT * FROM files WHERE external_doc_id = ?1",
                libsql::params![doc_id.clone()],
            )
            .await
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        let row = rows
            .next()
            .await
            .map_err(|e| format!("Failed to get next row: {}", e))?
            .ok_or(format!("No file found for given document {}", doc_id))?;

        let file: EncryptedFile = de::from_row::<EncryptedFile>(&row)
            .map_err(|e| format!("Failed to deserialize row: {}", e))?;

        Ok(file.file)
    }

    // Keep your original async constructor for tests or other use cases
    pub async fn new(env_hashmap: Option<HashMap<String, String>>) -> Self {
        let query = Self::new_sync(env_hashmap);
        let _ = query.connect().await; // Ignore result for simplicity
        query
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn can_connect() -> Result<(), String> {
//         let turso = TursoQuery::new().await;
//         let _ = turso.db.unwrap();
//         Ok(())
//     }

//     #[tokio::test]
//     async fn can_get_file() -> Result<(), String> {
//         let turso = TursoQuery::new().await;
//         let doc_id = "efceaeb2-b698-11ef-8873-f0b61e3e615b".to_string();
//         let result = turso.get_file(doc_id).await.unwrap();
//         println!("{}", result);
//         Ok(())
//     }
// }
