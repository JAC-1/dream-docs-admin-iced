use anyhow::Result;
use chrono::{DateTime, Local};
use rfd::FileDialog;
use std::path::Path;
use std::path::PathBuf;

pub struct FileSaver {
    root: PathBuf,
}

#[derive(Debug, Clone)]
pub struct FileToSave {
    content: Vec<u8>,
    file_name: String,
    display_name: String,
    created_at: String,
}

impl FileToSave {
    pub fn new(
        content: Vec<u8>,
        file_name: String,
        display_name: String,
        crated_at: DateTime<Local>,
    ) -> Self {
        Self {
            content,
            file_name,
            display_name,
            created_at: crated_at
                .to_rfc3339()
                .split("T")
                .next()
                .unwrap()
                .to_string(),
        }
    }
}

impl FileSaver {
    pub fn set_root() -> Result<Self> {
        let path = Path::new("/");
        if let Some(root) = FileDialog::new().set_directory(path).pick_folder() {
            Ok(Self { root })
        } else {
            anyhow::bail!("No directory selected")
        }
    }

    pub async fn save_individual(&self, file: FileToSave) -> Result<()> {
        let file_name_segment = file.file_name.split(".").collect::<Vec<&str>>();
        let file_name = file_name_segment.get(0).unwrap_or(&"").to_string();
        let file_extension = file_name_segment.get(1).unwrap_or(&"").to_string();
        let display_dir = self.root.join(&file.display_name);
        std::fs::create_dir_all(&display_dir)?;
        let file_path = display_dir
            .join(format!(
                "{}_{}_{}",
                file.display_name, file_name, file.created_at
            ))
            .with_extension(file_extension);
        std::fs::write(file_path, file.content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_root() {
        let file_saver = FileSaver::set_root().unwrap();
        println!("{}", file_saver.root.display());
    }

    #[tokio::test]
    async fn test_save_individual() {
        let file_saver = FileSaver::set_root().unwrap();
        let file = FileToSave::new(
            Vec::new(),
            String::from("test.txt"),
            String::from("john johnson"),
            Local::now(),
        );
        println!("{:?}", file_saver.save_individual(file).await.unwrap());
    }
}
