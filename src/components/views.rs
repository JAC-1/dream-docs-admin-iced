use super::{document_table::student_documents_table, student_profile_info::profile_info};
use crate::sample_data::get_sample_data;
use crate::styles::button_styles::custom_program_button;
use crate::Message;
use crate::NOTO_SANS_JP;
use iced::advanced::graphics::core::font;
use iced::widget::text::Wrapping;
use iced::widget::{
    button, column, container, rich_text, row, span, Button, Column, Container, Text,
};
use iced::Length::FillPortion;
use iced::{Center, Element, Fill, Font};

pub fn home_view() -> Element<'static, Message> {
    let card = |title: String, description: String| {
        // General card to display programs (Dream builder, Term, Long Term)
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

pub fn students_view() -> Element<'static, Message> {
    fn create_header_text(text: String) -> Element<'static, Message> {
        rich_text([span(text).size(20).font(Font {
            weight: font::Weight::Bold,
            ..Font::default()
        })])
        .width(FillPortion(1))
        .into()
    }

    let table_headers = [
        "名前",
        "かな",
        "ローマ字",
        "クラス",
        "プログラム",
        "書類完了",
    ];

    let table_header = row(table_headers
        .iter()
        .map(|header| create_header_text(header.to_string()))
        .collect::<Vec<_>>())
    .align_y(Center)
    .width(Fill)
    .padding(25);

    let data_rows = get_sample_data();
    let mut table = Column::new().push(table_header);
    for value in data_rows.into_values() {
        table = table.push(
            button(row![
                rich_text([span(value.kanji_name.to_string()).font(NOTO_SANS_JP),])
                    .width(FillPortion(1)),
                rich_text([span(value.kana_name.to_string()).font(NOTO_SANS_JP),])
                    .width(FillPortion(1)),
                rich_text([span(value.english_name.to_string()).font(NOTO_SANS_JP),])
                    .width(FillPortion(1)),
                rich_text([span(value.class.to_string()).font(NOTO_SANS_JP),])
                    .width(FillPortion(1)),
                rich_text([span(value.program.to_string()).font(NOTO_SANS_JP),])
                    .width(FillPortion(1)),
                rich_text([span(value.completed_all_documents.to_string()).font(NOTO_SANS_JP),])
                    .width(FillPortion(1)),
            ])
            .style(iced::widget::button::secondary)
            .on_press(Message::NavigateToStudentProfile),
        );
    }
    Container::new(table)
        .width(Fill)
        .center_x(Fill)
        .padding(20)
        .into()
}

pub fn student_profile() -> Element<'static, Message> {
    let profile_container = Container::new(profile_info()).max_height(400);
    let document_table = student_documents_table();
    container(column![profile_container, document_table])
        .center_x(Fill)
        .into()
}
