use crate::Message;
use iced::widget::{button, row, Space};
use iced::{Element, Fill};

pub fn nav_bar() -> Element<'static, Message> {
    row![
        Space::with_width(Fill),
        button("Classes").on_press(Message::NavigateToHome).padding(20),
        button("Students")
            .on_press(Message::NavigateToStudents)
            .padding(20),
        Space::with_width(Fill),
        button("Logout")
            .on_press(Message::Close)
            .padding(20)
            .style(button::text)
    ]
    .width(Fill)
    .into()
}

