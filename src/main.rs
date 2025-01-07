use anyhow::Result;
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
use operations::SupabaseQuery;

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
    StartFetchStudentDocs(StudentProfileData),
    StudentsLoaded(Result<Vec<StudentProfileData>, String>),
    FetchStudentDocs(Result<Vec<File>, String>, StudentProfileData),
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

    // TODO: Implement this into the vsiews
    #[allow(dead_code)]
    fn title(&self) -> String {
        match self {
            Dashboard::StudentDocsLoading => String::new(),
            Dashboard::StudentProfileDataLoading => String::new(),
            Dashboard::HomeView => String::from("Home - Dashboard"),
            Dashboard::StudentsView { .. } => String::from("Students - Dashboard"),
            Dashboard::StudentProfileview { .. } => String::from("Sudent Profile - Dashboard"),
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
            Message::FetchStudentDocs(Ok(docs), student) => {
                *self = Dashboard::StudentProfileview { student, docs };
                Task::none()
            }
            Message::FetchStudentDocs(Err(error), _) => {
                *self = Dashboard::Errored(error);
                Task::none()
            }
            Message::StudentsLoaded(Err(error)) => {
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
                views::student_profile(student, docs)
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
