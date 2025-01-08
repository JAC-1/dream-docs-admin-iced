use crate::models::turso_models::*;
use dotenv::dotenv;
use libsql::{de, Builder, Database};

pub struct TursoQuery {
    db: Result<Database, String>,
}

impl TursoQuery {
    async fn new() -> Self {
        dotenv().ok();
        let url = std::env::var("TURSO_DB_URL").expect("TURSO_DB_URL expected but none found.");
        let token =
            std::env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN expected but none found.");
        let builder = Builder::new_remote(url, token).build().await;
        match builder {
            Ok(db) => Self { db: Ok(db) },
            Err(e) => Self {
                db: Err(format!("Error connecting to remote turso database: {}", e)),
            },
        }
    }
    async fn get_file(self, doc_id: String) -> Result<String, String> {
        match self.db {
            Ok(db) => {
                let conn = db
                    .connect()
                    .expect("Failed to connect to to the turso database in get_file");
                let mut rows = conn
                    .query(
                        "SELECT * FROM files WHERE external_doc_id = ?1",
                        libsql::params![doc_id],
                    )
                    .await
                    .unwrap();
                let row = rows.next().await.unwrap().unwrap(); // Get's first row only
                let file: EncryptedFile = de::from_row::<EncryptedFile>(&row).unwrap();
                Ok(file.file)
            }

            Err(e) => Err(format!(
                "An error creating the conector in the get_file function occured: {}",
                e
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn can_connect() -> Result<(), String> {
        let turso = TursoQuery::new().await;
        let _ = turso.db.unwrap();
        Ok(())
    }

    #[tokio::test]
    async fn can_get_file() -> Result<(), String> {
        let turso = TursoQuery::new().await;
        let doc_id = "efceaeb2-b698-11ef-8873-f0b61e3e615b".to_string();
        let result = turso.get_file(doc_id).await.unwrap();
        println!("{}", result);
        Ok(())
    }
}
