#![windows_subsystem = "windows"]
use std::{collections::HashMap, path::PathBuf, env};

use chrono::{DateTime, Local};
use iced::{widget, Center, Element, Fill, Font, Task};
use iced::widget::Container;

mod components;
mod custom_settings;
mod models;
mod operations;
mod sample_data;
mod styles;
mod types;
use components::{navbar, views};
use custom_settings::window_settings;
use models::supabase_models::*;
use once_cell::sync::Lazy;
use operations::{Decrypter, FileSaver, FileToSave, SupabaseQuery, TursoQuery};
use types::FileStatus;

pub static NOTO_SANS_JP: Font = Font::with_name("Noto Sans JP");
static SUPABASE_CLIENT: Lazy<SupabaseQuery> = Lazy::new(|| SupabaseQuery::new());
const ENV_FILE: &str = include_str!("../.env");
fn parse_env(env_str: &str) -> HashMap<String, String> {
    env_str
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#')) // Ignore comments and empty lines
        .filter_map(|line| {
            let mut parts = line.splitn(2, '=');
            Some((parts.next()?.trim().to_string(), parts.next()?.trim().to_string()))
        })
        .collect()
}


fn main() -> iced::Result {
    let env_vars = parse_env(ENV_FILE);
    for (key, value) in env_vars {
        env::set_var(key, value);
    }
    let font_bytes_regular = include_bytes!("fonts/NotoSansJP-Regular.ttf").as_slice();
    let font_bytes_bold = include_bytes!("fonts/NotoSansJP-Bold.ttf").as_slice();
    iced::application("Dashboard", Dashboard::update, Dashboard::view)
        .window(window_settings::settings())
        .font(font_bytes_regular)
        .font(font_bytes_bold)
        .run_with(Dashboard::new)
}

#[derive(Debug, Clone, Default)]
struct DashboardState {
    loading_message: String,
    students: Vec<StudentProfileData>,
    selected_student: Option<StudentProfileData>,
    selected_student_docs: Vec<File>,
    doc_to_save: Option<FileToSave>,
    docs_to_save: Option<Vec<FileToSave>>,
    is_loading: bool,
    error: Option<String>,
    current_view: View,
    save_root: Option<PathBuf>,
}

#[derive(Debug, Clone, Default)]
pub enum View {
    #[default]
    Students,
    Home,
    StudentProfile,
}

#[derive(Debug)]
struct Dashboard {
    state: DashboardState,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetView(View),
    SetLoading(bool),
    SetError(Option<String>),
    ClearError,
    SetStudents(Vec<StudentProfileData>),
    SetSelectedStudent(StudentProfileData),
    SelectAndViewStudent(StudentProfileData),
    SetDocuments(Vec<File>),
    DownloadAllDocs,
    FetchStudentDoc(File),
    DocumentStatusSelected(FileStatus, String),
    UpdatedDocStatus(String),
    DocumentSave(FileToSave),
    DocumentsSave(Vec<FileToSave>),
    SetSaveRoot(PathBuf),
    SetSaveRootAndDownloadAll(PathBuf),
    SelectSaveRoot,
    Close,
}

impl Dashboard {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                state: DashboardState::default(),
            },
            Self::load_students(),
        )
    }

    fn load_students() -> Task<Message> {
        Task::perform(
            async {
                SUPABASE_CLIENT
                    .all_students_info()
                    .await
                    .map_err(|e| e.to_string())
            },
            move |result| match result {
                Ok(students) => Message::SetStudents(students),
                Err(error) => Message::SetError(Some(error)),
            },
        )
    }

    fn get_student_docs(student: StudentProfileData) -> Task<Message> {
        Task::perform(
            async move {
                SUPABASE_CLIENT
                    .get_student_document_info(student.display_id.clone())
                    .await
                    .map_err(|e| e.to_string())
            },
            move |result| match result {
                Ok(docs) => Message::SetDocuments(docs),
                Err(error) => Message::SetError(Some(error)),
            },
        )
    }

    fn get_student_doc(
        doc_id: String,
        full_file_name: String,
        created_at: DateTime<Local>,
        student_name: String,
    ) -> Task<Message> {
        Task::perform(
            async move {
                let enc_key = SUPABASE_CLIENT
                    .fetch_key(doc_id.clone())
                    .await
                    .map_err(|e| e.to_string())?;
                let turso = TursoQuery::new().await;
                let enc_file = turso.get_file(doc_id).await.map_err(|e| e.to_string())?;
                let decrypter = Decrypter::new(&enc_key, Some(&enc_file), &full_file_name)
                    .map_err(|e| e.to_string())?;
                let decrypted = decrypter
                    .decrypt_symetric_file()
                    .map_err(|e| e.to_string())?
                    .decrypted_data;
                Ok(FileToSave::new(
                    decrypted,
                    full_file_name,
                    student_name,
                    created_at,
                ))
            },
            |result: Result<FileToSave, String>| match result {
                Ok(file) => Message::DocumentSave(file),
                Err(error) => Message::SetError(Some(error)),
            },
        )
    }

    fn download_all_docs(docs: Vec<File>, student_name: String) -> Task<Message> {
        Task::perform(
            async move {
                let mut files_to_save: Vec<FileToSave> = vec![];
                for doc in docs {
                    let enc_key = SUPABASE_CLIENT
                        .fetch_key(doc.document_id.clone())
                        .await
                        .map_err(|e| e.to_string())?;
                    let turso = TursoQuery::new().await;
                    let enc_file = turso
                        .get_file(doc.document_id.clone())
                        .await
                        .map_err(|e| e.to_string())?;
                    let decrypter = Decrypter::new(&enc_key, Some(&enc_file), &doc.file_name)
                        .map_err(|e| e.to_string())?;
                    let decrypted = decrypter
                        .decrypt_symetric_file()
                        .map_err(|e| e.to_string())?
                        .decrypted_data;
                    let file_to_save: FileToSave = FileToSave::new(
                        decrypted,
                        doc.file_name,
                        student_name.clone(),
                        doc.created_at,
                    );
                    files_to_save.push(file_to_save);
                }
                Ok(files_to_save)
            },
            |result: Result<Vec<FileToSave>, String>| match result {
                Ok(files) => Message::DocumentsSave(files),
                Err(error) => Message::SetError(Some(error)),
            },
        )
    }

    fn save_file(file: FileToSave, root: PathBuf) -> Task<Message> {
        Task::perform(
            async move {
                let file_saver = FileSaver::new(root);
                file_saver.save_individual(file).await.unwrap()
            },
            |_| Message::SetLoading(false),
        )
    }

    fn save_all_files(files: Vec<FileToSave>, root: PathBuf) -> Task<Message> {
        Task::perform(
            async move {
                for file in files {
                    let file_saver = FileSaver::new(root.clone());
                    file_saver.save_individual(file).await.unwrap();
                }
            },
            |_| Message::SetLoading(false),
        )
    }

    fn update_file_status(status_string: String, doc_id: String) -> Task<Message> {
        Task::perform(
            async {
                SUPABASE_CLIENT
                    .update_doc_status(status_string, doc_id)
                    .await
                    .map_err(|e| e.to_string())
            },
            move |result| match result {
                Ok(s) => Message::UpdatedDocStatus(s),
                Err(error) => Message::SetError(Some(error)),
            },
        )
    }

    // // TODO: Implement this into the views
    // #[allow(dead_code)]
    // fn title(&self) -> String {
    //     match self {
    //         Dashboard::StudentDocsLoading => String::new(),
    //         Dashboard::StudentProfileDataLoading => String::new(),
    //         Dashboard::HomeView => String::from("Home - Dashboard"),
    //         Dashboard::StudentsView { .. } => String::from("Students - Dashboard"),
    //         Dashboard::StudentProfileview { .. } => String::from("Student Profile - Dashboard"),
    //         Dashboard::Errored(_) => String::from("Error - Dashboard"),
    //     }
    // }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetView(view) => {
                self.state.current_view = view;
                Task::none()
            }
            Message::SetLoading(is_loading) => {
                self.state.is_loading = is_loading;
                Task::none()
            }
            Message::SetStudents(students) => {
                self.state.students = students;
                self.state.loading_message = "Loading students".to_string();
                self.state.is_loading = false;
                Task::none()
            }
            Message::SetError(error) => {
                self.state.error = error;
                self.state.is_loading = false;
                Task::none()
            }
            Message::ClearError => {
                self.state.error = None;
                self.state.is_loading = false;
                Task::none()
            }
            Message::SetSelectedStudent(student_profile_data) => {
                self.state.selected_student = Some(student_profile_data);
                self.state.is_loading = false;
                Task::none()
            }
            Message::SelectAndViewStudent(student) => {
                self.state.selected_student = Some(student.clone());
                self.state.current_view = View::StudentProfile;
                self.state.loading_message = "Loading student profiles...".to_string();
                self.state.is_loading = true;
                self.state.loading_message = "Loading Student Profile...".to_string();
                Self::get_student_docs(student)
            }
            Message::SetDocuments(student_docs) => {
                self.state.selected_student_docs = student_docs;
                self.state.is_loading = false;
                Task::none()
            }
            Message::FetchStudentDoc(file) => {
                if let Some(selected_student) = &self.state.selected_student {
                    self.state.loading_message =
                        "Fetching and preparing selected document...".to_string();
                    self.state.is_loading = true;
                    Self::get_student_doc(
                        file.document_id,
                        file.file_name,
                        file.created_at,
                        selected_student.display_name.clone(),
                    )
                } else {
                    self.state.error =
                        Some("Error decrypting and preparing file for download.".to_string());
                    Task::none()
                }
            }
            Message::DocumentSave(file_to_download) => {
                self.state.loading_message = "".to_string();
                self.state.is_loading = true;
                if let Some(root) = &self.state.save_root {
                    self.state.loading_message = "Saving file ...".to_string();
                    Self::save_file(file_to_download, root.clone())
                } else {
                    self.state.doc_to_save = Some(file_to_download);
                    self.state.loading_message = "Setting root directory...".to_string();
                    Task::perform(
                        async { FileSaver::select_root().map_err(|e| e.to_string()) },
                        |result| match result {
                            Ok(root) => Message::SetSaveRoot(root),
                            Err(error) => Message::SetError(Some(error)),
                        },
                    )
                }
            }
            Message::DocumentsSave(files_to_download) => {
                self.state.is_loading = true;
                if let Some(root) = &self.state.save_root {
                    Self::save_all_files(files_to_download, root.clone())
                } else {
                    self.state.docs_to_save = Some(files_to_download);
                    Task::perform(
                        async { FileSaver::select_root().map_err(|e| e.to_string()) },
                        |result| match result {
                            Ok(root) => Message::SetSaveRootAndDownloadAll(root),
                            Err(error) => Message::SetError(Some(error)),
                        },
                    )
                }
            }
            Message::DownloadAllDocs => match &self.state.selected_student {
                Some(student) => {
                    self.state.loading_message =
                        "Downloading and processing all docs ...".to_string();
                    self.state.is_loading = true;
                    Self::download_all_docs(
                        self.state.selected_student_docs.clone(),
                        student.display_name.clone(),
                    )
                }
                None => {
                    self.state.is_loading = false;
                    self.state.error =
                        Some("Error: No student found to download all docs.".to_string());
                    Task::none()
                }
            },
            Message::SetSaveRootAndDownloadAll(root) => {
                self.state.save_root = Some(root.clone());
                if let Some(docs_to_save) = self.state.docs_to_save.take() {
                    Self::save_all_files(docs_to_save, root)
                } else {
                    self.state.error = Some("An Erorr saving all docs has occured".to_string());
                    Task::none()
                }
            }
            Message::SetSaveRoot(root) => {
                self.state.save_root = Some(root.clone());
                if let Some(file) = self.state.doc_to_save.take() {
                    self.state.loading_message = "Saving file...".to_string();
                    Self::save_file(file, root)
                } else {
                    self.state.error = Some("An error has occured saving the file.".to_string());
                    Task::none()
                }
            }
            Message::SelectSaveRoot => Task::perform(
                async { FileSaver::select_root().map_err(|e| e.to_string()) },
                |result| match result {
                    Ok(root) => Message::SetSaveRoot(root),
                    Err(error) => Message::SetError(Some(error)),
                },
            ),
            Message::DocumentStatusSelected(file_status, doc_id) => {
                // self.state.is_loading = true;
                let status_string = file_status.clone().to_str().to_string();
                Self::update_file_status(status_string, doc_id)
            }
            Message::UpdatedDocStatus(message) => {
                println!("{}", message);
                self.state.current_view = View::StudentProfile;
                match self.state.selected_student.clone() {
                    Some(student) => Self::get_student_docs(student),
                    None => Task::none(),
                }
            }
            Message::Close => std::process::exit(0),
        }
    }

    fn view(&self) -> Element<Message> {
        let nav_bar = navbar::nav_bar();
        let content = match &self.state.current_view {
            View::Home => views::home_view(),
            View::Students => {
                if self.state.is_loading {
                    widget::text("Students Loading...").size(50).center().into()
                } else if let Some(err) = &self.state.error {
                    let error_column = widget::column![
                        widget::text("Something went wrong...").size(40),
                        widget::text(err),
                        widget::button("Back")
                            .on_press(Message::SetView(View::Students))
                            .on_press(Message::ClearError)
                    ]
                    .spacing(20).width(Fill).height(Fill).align_x(Center);
                    Container::new(error_column).width(Fill).height(Fill).center(Fill).padding(78).into()
                } else {
                    views::students_view(&self.state.students)
                }
            }
            View::StudentProfile => {
                if self.state.is_loading {
                    let loading_text = self.state.loading_message.clone();
                    widget::text(loading_text)
                        .size(50)
                        .center()
                        .into()
                } else if let Some(err) = &self.state.error {
                    widget::column![
                        widget::text("Something went wrong..").size(40),
                        widget::text(err),
                        widget::button("Back")
                            .on_press(Message::SetView(View::Students))
                            .on_press(Message::ClearError)
                    ]
                    .spacing(20)
                    .into()
                } else if let Some(student) = &self.state.selected_student {
                    views::student_profile(student, &self.state.selected_student_docs)
                } else {
                    widget::text("No student selected, or no such student found")
                        .size(50)
                        .center()
                        .into()
                }
            }
        };
        widget::column![nav_bar, content].into()
    }
}
