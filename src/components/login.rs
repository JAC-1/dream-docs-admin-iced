use crate::Message;
use iced::widget::{button, row, text_input};
use iced::{Element, Fill};

pub fn login(password_content: &str) -> Element<'static, Message> {
    let password_input =
        text_input("Enter password..", password_content).on_input(Message::SetPasswordInputChange);
    let button = button("Login").on_press(Message::SetLogin(password_content.to_string()));
    row![password_input, button].width(Fill).padding(10).into()
}
