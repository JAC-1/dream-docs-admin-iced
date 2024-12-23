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
use once_cell::sync::Lazy;
use operations::SupabaseQuery;

pub static NOTO_SANS_JP: Font = Font::with_name("Noto Sans JP");
static SUPABASE_CLIENT: Lazy<SupabaseQuery> = Lazy::new(|| SupabaseQuery::new());

fn main() -> iced::Result {
    // let runtime = runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap();
    // let _guard = runtime.enter();
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
    //TODO: Implement students in the view
    #[allow(dead_code)]
    StudentsView {
        students: Vec<StudentProfileData>,
    },

    // TODO: Implement student profile data in the view
    #[allow(dead_code)]
    StudentProfileview {
        student: StudentProfileData,
    },
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
                SUPABASE_CLIENT
                    .all_students_info()
                    .await
                    .map_err(|err| err.to_string())
            },
            Message::StudentsLoaded,
        )
    }

    // TODO: Implement this into the views
    #[allow(dead_code)]
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
            Dashboard::StudentsView { students } => views::students_view(students),
            Dashboard::HomeView => views::home_view(),
            Dashboard::StudentProfileview { student } => views::student_profile(student.clone()),
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
