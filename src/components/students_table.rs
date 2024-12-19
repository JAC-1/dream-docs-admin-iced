use crate::sample_data::get_sample_data;
use crate::{Message, NOTO_SANS_JP};
use iced::widget::{button, rich_text, row, span, Column};
use iced::Alignment::Center;
use iced::Length::{Fill, FillPortion};
use iced::{font, Element, Font};

pub fn students_table() -> Element<'static, Message> {
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
    table.into()
}
