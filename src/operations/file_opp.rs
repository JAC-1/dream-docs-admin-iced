use crate::types::TaskType;
use anyhow::Result;
use chrono::{DateTime, Local};
use rfd::FileDialog;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

pub struct FileSaver {
    root: PathBuf,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FileToSave {
    content: Vec<u8>,
    file_name: String,
    display_name: String,
    created_at: String,
    task_type: TaskType,
}

impl FileToSave {
    pub fn new(
        content: Vec<u8>,
        file_name: String,
        display_name: String,
        crated_at: DateTime<Local>,
        task_type: TaskType,
    ) -> Self {
        FileToSave {
            content,
            file_name,
            display_name,
            created_at: crated_at
                .to_rfc3339()
                .split("T")
                .next()
                .unwrap()
                .to_string(),
            task_type,
        }
    }
}

impl FileSaver {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
    pub fn select_root() -> Result<PathBuf> {
        let path = Path::new("/");
        if let Some(root) = FileDialog::new().set_directory(path).pick_folder() {
            Ok(root)
        } else {
            anyhow::bail!("No directory selected")
        }
    }

    pub async fn save_individual(&self, file: FileToSave) -> Result<()> {
        let dir = self.root.join(&file.display_name);
        std::fs::create_dir_all(&dir)?;

        let raw_path = Path::new(&file.file_name);
        let extension = raw_path
            .extension()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .unwrap_or("");
        // Use bellow if actual file names are requested
        // let file_name = raw_path.file_stem().unwrap_or(OsStr::new("unreadable_file_name")).to_str().unwrap_or("unhandlable_file_name");
        // let clean_file_name = file_name.replace(".", "_");
        let document_type = &file.task_type.to_string();
        let first_name = &file.display_name.split(" ").next().unwrap();
        let last_name = &file.display_name.split(" ").last().unwrap();
        // let file_name = format!("{}({} {})", document_type, first_name, last_name);
        let file_name = if matches!(file.task_type, TaskType::FamilyImages) {
            let count = std::fs::read_dir(&dir)?
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    entry
                        .file_name()
                        .to_string_lossy()
                        .starts_with("Family Images")
                })
                .count();
            format!(
                "{}({} {})_{}",
                document_type,
                first_name,
                last_name,
                count + 1
            )
        } else {
            format!("{}({} {})", document_type, first_name, last_name)
        };
        // let final_path = dir.join(clean_file_name).with_extension(extension);
        let final_path = dir.join(file_name).with_extension(extension);
        std::fs::write(final_path, file.content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_root() {
        let root = FileSaver::select_root().unwrap();
        let file_saver = FileSaver::new(root);
        println!("{}", file_saver.root.display());
    }

    #[tokio::test]
    async fn test_save_individual() {
        let root = FileSaver::select_root().unwrap();
        let file_saver = FileSaver::new(root);
        let file = FileToSave::new(
            Vec::new(),
            String::from("test.txt"),
            String::from("john johnson"),
            Local::now(),
            TaskType::FamilyImages,
        );
        println!("{:?}", file_saver.save_individual(file).await.unwrap());
    }
}
