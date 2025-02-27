use crate::Message;
use iced::widget::{button, row, text_input};
use iced::{Element, Fill};

pub fn login(password_content: &str, salt_content: &str) -> Element<'static, Message> {
    let password_input =
        text_input("Enter password..", password_content).on_input(Message::SetPasswordInputChange);
    let salt_input = text_input("Enter salt..", salt_content).on_input(Message::SetSaltInputChange);
    let button = button("Login").on_press(Message::SetLogin(
        password_content.to_string(),
        salt_content.to_string(),
    ));
    row![password_input, salt_input, button]
        .width(Fill)
        .padding(10)
        .into()
}
