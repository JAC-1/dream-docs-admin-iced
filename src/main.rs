use iced::widget::{button, column, row, Container, Row, Space, Text};
use iced::{window::settings::Settings, Element, Length::Fill, Theme};

mod components;

fn main() -> iced::Result {
    // iced::run("Dashboard", Dashboard::update, Dashboard::view).themeI
    iced::application("Dashboard", Dashboard::update, Dashboard::view)
        .theme(Dashboard::theme)
        .window(window_settings())
        .run()
}

pub struct Dashboard {
    pub view: View,
}

fn window_settings() -> Settings {
    let icon = iced::window::icon::from_file("C:\\Users\\Justin\\Pictures\\fireflyanpanman.jpg");
    if let Ok(icon) = icon {
        Settings {
            transparent: true,
            decorations: false,
            icon: Some(icon),
            ..Settings::default()
        }
    } else {
        Settings::default()
    }
}

impl Default for Dashboard {
    fn default() -> Self {
        Self { view: View::Home }
    }
}

impl Dashboard {
    fn view(&self) -> Element<Message> {
        let nav_bar_menu = row![
            Space::with_width(Fill),
            button("Home").on_press(Message::NavigateToHome).padding(20),
            button("Hello World")
                .on_press(Message::NavigateToHelloWorld)
                .padding(20),
            Space::with_width(Fill),
            button("Logout")
                .on_press(Message::Close)
                .padding(20)
                .style(button::text)
        ]
        .width(Fill);

        let main_content = match self.view {
            View::Home => self.home_view(),
            View::HelloWorld => self.hello_world_view(),
        };

        column![nav_bar_menu, main_content].into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NavigateToHome => self.view = View::Home,
            Message::NavigateToHelloWorld => self.view = View::HelloWorld,
            Message::Close => std::process::exit(0),
        }
    }

    fn home_view(&self) -> Element<Message> {
        let card = |title: String| {
            Container::new(Text::new(title).size(20))
                .padding(20)
                .width(30)
        };
        let cards = Row::new()
            .spacing(10)
            .padding(20)
            .push(card("1".to_string()))
            .push(card("2".to_string()))
            .push(card("3".to_string()));
        Container::new(cards).center_x(Fill).center_y(Fill).into()
    }

    fn hello_world_view(&self) -> Element<Message> {
        Container::new(Text::new("Hello World").size(50).center())
            .center_y(Fill)
            .center_x(Fill)
            .into()
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
    NavigateToHelloWorld,
    Close,
}
