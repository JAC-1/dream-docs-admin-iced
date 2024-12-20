use crate::operations::supabase_opp::all_students_info;
use anyhow::Result;
use iced::widget;
use iced::{Element, Font, Task};
use tokio::runtime;

mod components;
mod custom_settings;
mod models;
mod operations;
mod sample_data;
mod styles;
mod types;
use components::{navbar, views};
use custom_settings::window_settings;
use models::supabase_models::StudentProfileData;

pub static NOTO_SANS_JP: Font = Font::with_name("Noto Sans JP");

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
    Loading,
    HomeView,
    StudentsView { students: Vec<StudentProfileData> },
    StudentProfileview { student: StudentProfileData },
    Errored(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    StudentsLoaded(Result<Vec<StudentProfileData>, String>),
    NavigateToHome,
    NavigateToStudents,
    NavigatetoStudentProfile(StudentProfileData),
    Close,
}

impl Dashboard {
    fn new() -> (Self, Task<Message>) {
        (Self::Loading, Self::load_students())
    }
    fn load_students() -> Task<Message> {
        Task::perform(
            async {
                let rt = runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                rt.block_on(all_students_info())
                    .map_err(|err| err.to_string())
            },
            Message::StudentsLoaded,
        )
        // data and adds it to the enum
    }
    fn title(&self) -> String {
        match self {
            Dashboard::Loading => String::from("Loading - Dashboard"),
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
            Message::StudentsLoaded(Err(error)) => {
                *self = Dashboard::Errored(error);
                Task::none()
            }
            Message::NavigateToHome => {
                *self = Dashboard::HomeView;
                Task::none()
            }
            Message::NavigateToStudents => match self {
                Dashboard::Loading => Task::none(),
                _ => {
                    *self = Dashboard::Loading;
                    Self::load_students()
                }
            },
            Message::NavigatetoStudentProfile(student) => {
                *self = Dashboard::StudentProfileview { student };
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
            Dashboard::Loading => widget::text("Loading..").size(50).into(),
            Dashboard::StudentsView { .. } => views::students_view(),
            Dashboard::HomeView => views::home_view(),
            Dashboard::StudentProfileview { .. } => views::student_profile(),
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
