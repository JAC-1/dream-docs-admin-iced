use std::fmt::Binary;

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
use operations::{SupabaseQuery, TursoQuery, Decrypter, FileSaver, FileToSave};

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

#[derive(Debug)]
enum Dashboard {
    StudentProfileDataLoading,
    StudentDocsLoading,
    HomeView,
    StudentsView {
        students: Vec<StudentProfileData>,
    },
    StudentProfileview {
        student: StudentProfileData,
        docs: Vec<File>,
    },

    Errored(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    // StartFetchStudentDoc(File),
    StartFetchStudentDocs(StudentProfileData),
    FetchStudentDocs(Result<Vec<File>, String>, StudentProfileData),

    StartFetchStudentDoc(File, StudentProfileData),
    FetchStudentDoc(FileToSave),
    StudentDocSaved,
    StudentsLoaded(Result<Vec<StudentProfileData>, String>),
    NavigateToHome,
    NavigateToStudents,
    NavigatetoStudentProfile(StudentProfileData, Vec<File>),
    Close,
}

impl Dashboard {
    fn new() -> (Self, Task<Message>) {
        (Self::StudentProfileDataLoading, Self::load_students())
    }

    fn load_students() -> Task<Message> {
        Task::perform(
            async {
                SUPABASE_CLIENT
                    .all_students_info()
                    .await
                    .map_err(|e| e.to_string())
            },
            Message::StudentsLoaded,
        )
    }

    fn get_student_docs(student_id: String, student: StudentProfileData) -> Task<Message> {
        Task::perform(
            async move {
                SUPABASE_CLIENT
                    .get_student_document_info(student_id)
                    .await
                    .map_err(|e| e.to_string())
            },
            move |result| Message::FetchStudentDocs(result, student.clone()),
        )
    }

    fn get_student_doc(doc_id: String, full_file_name: String, display_name: String, created_at: DateTime<Local>) -> Task<Message> {
        Task::perform(
            async move {
                let enc_key = SUPABASE_CLIENT
                   .fetch_key(doc_id.clone())
                    .await
                    .map_err(|e| e.to_string()).unwrap();
                let turso = TursoQuery::new().await;
                let enc_file = turso.get_file(doc_id).await.unwrap();
                let decrypter = Decrypter::new(
                    &enc_key,
                    Some(&enc_file),
                    &full_file_name,
                ).unwrap();
                let decrypted = decrypter.decrypt_symetric_file().unwrap().decrypted_data;
                FileToSave::new(decrypted, full_file_name, display_name, created_at)
            },
            move |result| Message::FetchStudentDoc(result),
        )

    }
    fn save_file(file: FileToSave) -> Task<Message> {
        Task::perform(
            async move {
                let file_saver = FileSaver::set_root().unwrap();
                file_saver.save_individual(file).await.unwrap()
            },
            move |result| Message::StudentDocSaved,
        )
    }

    // TODO: Implement this into the vsiews
    #[allow(dead_code)]
    fn title(&self) -> String {
        match self {
            Dashboard::StudentDocsLoading => String::new(),
            Dashboard::StudentProfileDataLoading => String::new(),
            Dashboard::HomeView => String::from("Home - Dashboard"),
            Dashboard::StudentsView { .. } => String::from("Students - Dashboard"),
            Dashboard::StudentProfileview { .. } => String::from("Student Profile - Dashboard"),
            Dashboard::Errored(_) => String::from("Error - Dashboard"),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::StudentsLoaded(Ok(students)) => {
                *self = Dashboard::StudentsView { students };
                Task::none()
            }
            Message::StartFetchStudentDocs(student) => {
                *self = Dashboard::StudentDocsLoading;
                Self::get_student_docs(student.display_id.clone(), student)
            }
            Message::FetchStudentDocs(result, student) => {
                match result {
                    Ok(docs) => *self = Dashboard::StudentProfileview { student, docs },
                    Err(error) => *self = Dashboard::Errored(error),
                }
                Task::none()
            }
            Message::StartFetchStudentDoc(file, student) => {
                *self = Dashboard::StudentDocsLoading;
                Self::get_student_doc(file.document_id.clone(), file.file_name.clone(), student.display_name, file.created_at)
            }
            Message::FetchStudentDoc(file_to_save) => {
                *self = Dashboard::StudentDocsLoading;
                Self::save_file(file_to_save)
            }
            Message::StudentDocSaved => {
                Task::none()
            }
            Message::StudentsLoaded(Err(error)) => { // TODO: Refactor
                *self = Dashboard::Errored(error);
                Task::none()
            }
            Message::NavigateToHome => {
                *self = Dashboard::HomeView;
                Task::none()
            }
            Message::NavigateToStudents => match self {
                Dashboard::StudentProfileDataLoading => Task::none(),
                _ => {
                    *self = Dashboard::StudentProfileDataLoading;
                    Self::load_students()
                }
            },
            Message::NavigatetoStudentProfile(student, docs) => {
                *self = Dashboard::StudentProfileview { student, docs };
                Task::none()
            }
            Message::Close => {
                std::process::exit(1);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let nav_bar = navbar::nav_bar();

        let content = match self {
            Dashboard::StudentDocsLoading => widget::text("Loading..").size(50).center().into(),
            Dashboard::StudentProfileDataLoading => {
                widget::text("Loading..").size(50).center().into()
            }
            Dashboard::StudentsView { students } => views::students_view(students),
            Dashboard::HomeView => views::home_view(),
            Dashboard::StudentProfileview { student, docs } => {
                views::student_profile(student.clone(), docs.clone())
            }
            Dashboard::Errored(error_message) => widget::column![
                widget::text("Something went wrong..").size(40),
                widget::text(error_message),
                widget::button("Try again").on_press(Message::NavigateToStudents)
            ]
            .spacing(20)
            .into(),
        };
        widget::column![nav_bar, content].into()
    }
}
