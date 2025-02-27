use super::{
    document_table::student_documents_table, student_profile_info::profile_info,
    students_table::students_table,
};
use crate::components::login::login;
use crate::models::supabase_models::*;
use crate::styles::button_styles::custom_program_button;
use crate::Message;
use iced::widget::text::Wrapping;
use iced::widget::{column, container, row, Button, Container, Text};
use iced::{Element, Fill};

pub fn home_view() -> Element<'static, Message> {
    let card = |title: String, description: String| {
        Button::new(column![
            Text::new(title).size(20).center().wrapping(Wrapping::Word),
            Text::new(description).size(16).wrapping(Wrapping::Word),
        ])
        .padding(20)
        .width(300)
        .height(200)
        .on_press(Message::Close)
        .style(custom_program_button)
    };

    let cards = column![
        row![
            card(
                "Dream Builder Program".to_string(),
                "An amazing description of the dream builder program".to_string()
            ),
            card(
                "Term Program".to_string(),
                "An amazing description of the term program".to_string()
            ),
        ]
        .spacing(30)
        .padding(30),
        row![
            card(
                "Long Term Program".to_string(),
                "An amazing description of the long term program with an incredibly long explination that consits of random words spelled correctly and incorrectly".to_string()
            ),
            card(
                "Austrialia".to_string(),
                "An amazing description of the austrialia program".to_string()
            ),
        ]
        .spacing(30)
        .padding(30)
    ]
    .padding(10);

    Container::new(cards).center_x(Fill).center_y(Fill).into()
}

pub fn students_view(student_profile_data: &Vec<StudentProfileData>) -> Element<'static, Message> {
    let table = students_table(student_profile_data);
    Container::new(table)
        .width(Fill)
        .center_x(Fill)
        .padding(20)
        .into()
}

pub fn student_profile(
    student: &StudentProfileData,
    docs: &Vec<File>,
) -> Element<'static, Message> {
    let profile_container = Container::new(profile_info(student)).max_height(400);
    let document_table = student_documents_table(docs);
    container(column![profile_container, document_table])
        .center_x(Fill)
        .into()
}

pub fn login_view(password_content: &str, salt_content: &str) -> Element<'static, Message> {
    let login = login(password_content, salt_content);
    Container::new(login).center(Fill).into()
}
