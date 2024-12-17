use iced::widget::column;
use iced::{Element, Font, Theme};

mod components;
mod custom_settings;
mod styles;
use components::{navbar, views};
use custom_settings::window_settings;

fn main() -> iced::Result {
    let font_bytes = include_bytes!("fonts/NotoSansJP-Variable.ttf").as_slice();
    iced::application("Dashboard", Dashboard::update, Dashboard::view)
        .theme(Dashboard::theme)
        .window(window_settings::settings())
        .font(font_bytes)
        .run()
}

const NOTO_SANS_JP: Font = Font::with_name("NOTO_SANS_JP");

struct Dashboard {
    view: View,
}

impl Default for Dashboard {
    fn default() -> Self {
        Self {
            view: View::StudentProfile,
        }
    }
}

impl Dashboard {
    fn view(&self) -> Element<Message> {
        let nav_bar_menu = navbar::nav_bar();

        let main_content = match self.view {
            View::Home => views::home_view(),
            View::Students => views::students_view(),
            View::StudentProfile => views::student_profile(),
        };

        column![nav_bar_menu, main_content].into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NavigateToHome => self.view = View::Home,
            Message::NavigateToStudents => self.view = View::Students,
            Message::Close => std::process::exit(0),
            Message::NavigateToStudentProfile => self.view = View::StudentProfile,
        }
    }

    fn theme(&self) -> Theme {
        Theme::Oxocarbon
    }
}

enum View {
    Home,
    Students,
    StudentProfile,
}

#[derive(Debug, Clone)]
pub enum Message {
    NavigateToHome,
    NavigateToStudents,
    NavigateToStudentProfile,
    Close,
}
