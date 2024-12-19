use crate::{Message, NOTO_SANS_JP};
use iced::advanced::graphics::core::font;
use iced::widget::{button, horizontal_rule, row, text, vertical_rule, vertical_space, Column};
use iced::Alignment::Center;
use iced::Length::{Fill, FillPortion};
use iced::{Element, Font};

pub fn students_table() -> Element<'static, Message> {
    fn create_header_text(text: String) -> Element<'static, Message> {
        text::Text::new(text)
            .size(20)
            .font(Font {
                family: font::Family::Name("Noto Sans JP"),
                weight: font::Weight::Bold,
                ..Default::default()
            })
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
    .padding(3);

    let mut table = Column::new().push(table_header).push(horizontal_rule(20));
    // Using the sample student data directly
    table = table.push(
        button(row![
            text!("山田太郎").font(NOTO_SANS_JP).width(FillPortion(1)),
            text!("やまだたろう") // Kana name added
                .font(NOTO_SANS_JP)
                .width(FillPortion(1)),
            text!("Taro Yamada")
                .font(NOTO_SANS_JP)
                .width(FillPortion(1)),
            text!("Amazing Class")
                .font(NOTO_SANS_JP)
                .width(FillPortion(1)),
            text!("Amazing Program")
                .font(NOTO_SANS_JP)
                .width(FillPortion(1)),
            text!("未完了") // Document status added
                .font(NOTO_SANS_JP)
                .width(FillPortion(1)),
        ])
        .style(iced::widget::button::secondary)
        .on_press(Message::NavigateToStudentProfile),
    );
    table.into()
}
