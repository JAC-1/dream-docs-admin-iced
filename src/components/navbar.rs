use crate::Message;
use iced::widget::{button, horizontal_space, row, Text};
use iced::{Element, Fill, Font};

pub fn nav_bar() -> Element<'static, Message> {
    let nav_button = |title: String, message: Message| {
        button(Text::new(title).font(Font {
            family: iced::font::Family::Name("Noto Sans JP"),
            weight: iced::font::Weight::Bold,
            ..Default::default()
        }))
        .padding(20)
        .style(button::text)
        .on_press(message)
    };
    row![
        horizontal_space(),
        nav_button(String::from("Classes"), Message::NavigateToHome),
        nav_button(String::from("Students"), Message::NavigateToStudents),
        nav_button(String::from("閉じる"), Message::Close),
    ]
    .width(Fill)
    .padding(10)
    .into()
}
