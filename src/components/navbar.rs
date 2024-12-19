use crate::Message;
use iced::widget::{button, horizontal_space, row};
use iced::{Element, Fill};

pub fn nav_bar() -> Element<'static, Message> {
    row![
        horizontal_space(),
        button("Classes")
            .on_press(Message::NavigateToHome)
            .padding(20)
            .style(button::text),
        button("Students")
            .on_press(Message::NavigateToStudents)
            .padding(20)
            .style(button::text),
        button("Logout")
            .on_press(Message::Close)
            .padding(20)
            .style(button::text)
    ]
    .width(Fill)
    .into()
}
