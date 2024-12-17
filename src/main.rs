use iced::widget::column;
use iced::{Element, Theme};

mod components;
mod custom_settings;
use components::{navbar, views};
use custom_settings::window_settings;

fn main() -> iced::Result {
    iced::application("Dashboard", Dashboard::update, Dashboard::view)
        .theme(Dashboard::theme)
        .window(window_settings::settings())
        .run()
}

struct Dashboard {
    view: View,
}

impl Default for Dashboard {
    fn default() -> Self {
        Self { view: View::Home }
    }
}

impl Dashboard {
    fn view(&self) -> Element<Message> {
        let nav_bar_menu = navbar::nav_bar();

        let main_content = match self.view {
            View::Home => views::home_view(),
            View::HelloWorld => views::students_view(),
        };

        column![nav_bar_menu, main_content].into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NavigateToHome => self.view = View::Home,
            Message::NavigateToStudents => self.view = View::HelloWorld,
            Message::Close => std::process::exit(0),
        }
    }

    fn theme(&self) -> Theme {
        Theme::Oxocarbon
    }
}

enum View {
    Home,
    HelloWorld,
}

#[derive(Debug, Clone)]
pub enum Message {
    NavigateToHome,
    NavigateToStudents,
    Close,
}
