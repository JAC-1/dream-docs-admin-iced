use anyhow::Result;
use chrono::{DateTime, Local};
use iced::{widget, Element, Font, Task};

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

pub static NOTO_SANS_JP: Font = Font::with_name("Noto Sans JP");
static SUPABASE_CLIENT: Lazy<SupabaseQuery> = Lazy::new(|| SupabaseQuery::new());

fn main() -> iced::Result {
    let font_bytes_regular = include_bytes!("fonts/NotoSansJP-Regular.ttf").as_slice();
    let font_bytes_bold = include_bytes!("fonts/NotoSansJP-Bold.ttf").as_slice();
    iced::application("Dashboard", Dashboard::update, Dashboard::view)
        .window(window_settings::settings())
        .font(font_bytes_regular)
        .font(font_bytes_bold)
        .run_with(Dashboard::new)
}

#[derive(Debug, Clone)]
struct DashboardState {
    students: Vec<StudentProfileData>,
    selected_student: Option<StudentProfileData>,
    selected_student_docs: Vec<File>,
    selected_student_doc: Option<File>,
    doc_to_save: Option<FileToSave>,
    is_loading: bool,
    error: Option<String>,
    current_view: View,
}

impl Default for DashboardState {
    fn default() -> Self {
        DashboardState {
            students: Vec::new(),
            selected_student: None,
            selected_student_docs: Vec::new(),
            selected_student_doc: None,
            doc_to_save: None,
            is_loading: false,
            error: None,
            current_view: View::default(),
        }
    }
}

#[derive(Debug, Clone)]
enum View {
    Home,
    Students,
    StudentProfile,
}

impl Default for View {
    fn default() -> Self {
        View::Home
    }
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
    SetStudents(Vec<StudentProfileData>),
    SetSelectedStudent(StudentProfileData),
    SelectAndViewStudent(StudentProfileData),
    SetDocuments(Vec<File>),
    FetchStudentDoc(File),
    DocumentSave(FileToSave),
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
                    .map_err(|e| e.to_string())
                    .unwrap();
                let turso = TursoQuery::new().await;
                // TODO: Too many unsafe unwraps
                let enc_file = turso.get_file(doc_id).await.unwrap();
                let decrypter = Decrypter::new(&enc_key, Some(&enc_file), &full_file_name).unwrap();
                let decrypted = decrypter.decrypt_symetric_file().unwrap().decrypted_data;
                FileToSave::new(decrypted, full_file_name, student_name, created_at)
            },
            move |ready_file| Message::DocumentSave(ready_file),
        )
    }
    fn save_file(file: FileToSave) -> Task<Message> {
        Task::perform(
            async move {
                let file_saver = FileSaver::set_root().unwrap();
                file_saver.save_individual(file).await.unwrap()
            },
            |_| Message::SetLoading(false),
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
                self.state.is_loading = false;
                Task::none()
            }
            Message::SetError(error) => {
                self.state.error = error;
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
                self.state.is_loading = true;
                Self::get_student_docs(student)
            }
            Message::SetDocuments(student_docs) => {
                self.state.selected_student_docs = student_docs;
                self.state.is_loading = false;
                Task::none()
            }
            Message::FetchStudentDoc(file) => {
                if let Some(selected_student) = &self.state.selected_student {
                    self.state.is_loading = true;
                    Self::get_student_doc(
                        file.document_id,
                        file.file_name,
                        file.created_at,
                        selected_student.display_name.clone(),
                    );
                } else {
                    self.state.error =
                        Some("Error decrypting and preparing file for download.".to_string());
                }
                Task::none()
            }
            Message::DocumentSave(file_to_download) => {
                self.state.is_loading = true;
                Self::save_file(file_to_download)
            }
            Message::Close => {
                std::process::exit(1);
            }
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
                    widget::column![
                        widget::text("Something went wrong..").size(40),
                        widget::text(err),
                        widget::button("Try again").on_press(Message::SetView(View::Students))
                    ]
                    .spacing(20)
                    .into()
                } else {
                    views::students_view(&self.state.students)
                }
            }
            View::StudentProfile => {
                if self.state.is_loading {
                    widget::text("Student Profile Loading...")
                        .size(50)
                        .center()
                        .into()
                } else if let Some(student) = &self.state.selected_student {
                    views::student_profile(student, &&self.state.selected_student_docs)
                } else {
                    widget::text("No student selected").size(50).center().into()
                }
            }
        };
        widget::column![nav_bar, content].into()
    }
}
