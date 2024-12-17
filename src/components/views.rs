use crate::styles::button_styles::custom_program_button;
use crate::Message;
use crate::NOTO_SANS_JP;
use iced::advanced::graphics::core::font;
use iced::border::Radius;
use iced::color;
use iced::widget::text::Wrapping;
use iced::widget::{
    button, column, container, image, rich_text, row, span, Button, Column, Container, Text,
};
use iced::Background;
use iced::Length::FillPortion;
use iced::Shadow;
use iced::{Center, Element, Fill, Font};
use std::collections::HashMap;

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

// TODO::  Students are rows of students (columns = ["Kanji", "kana", "English", "Class", "Program"])
// TODO :  Has input bar for future search
// TODO::  Clicking the student displays the student's profile

struct Student {
    kanji_name: String,
    kana_name: String,
    english_name: String,
    class: String,
    program: String,
    completed_all_documents: bool,
}

pub fn students_view() -> Element<'static, Message> {
    let table_header = row![
        rich_text([span("名前").size(20).font(Font {
            weight: font::Weight::Bold,
            ..Font::default()
        })])
        .width(FillPortion(1)),
        rich_text([span("かな").size(20).font(Font {
            weight: font::Weight::Bold,
            ..Font::default()
        })])
        .width(FillPortion(1)),
        rich_text([span("ローマ字").size(20).font(Font {
            weight: font::Weight::Bold,
            ..Font::default()
        })])
        .width(FillPortion(1)),
        rich_text([span("クラス".to_string()).size(20).font(Font {
            weight: font::Weight::Bold,
            ..Font::default()
        })])
        .width(FillPortion(1)),
        rich_text([span("プログラム".to_string()).size(20).font(Font {
            weight: font::Weight::Bold,
            ..Font::default()
        })])
        .width(FillPortion(1)),
        rich_text([span("書類完了".to_string()).size(20).font(Font {
            weight: font::Weight::Bold,
            ..Font::default()
        })])
        .width(FillPortion(1)),
    ]
    .align_y(Center)
    .width(Fill)
    .padding(25);

    let mut rows = HashMap::new();
    rows.insert(
        "1",
        Student {
            kanji_name: "山田太郎".to_string(),
            kana_name: "やまだたろう".to_string(),
            english_name: "Taro Yamada".to_string(),
            class: "3A".to_string(),
            program: "Dream Builder".to_string(),
            completed_all_documents: true,
        },
    );
    rows.insert(
        "2",
        Student {
            kanji_name: "佐藤花子".to_string(),
            kana_name: "さとうはなこ".to_string(),
            english_name: "Hanako Sato".to_string(),
            class: "2B".to_string(),
            program: "Term Program".to_string(),
            completed_all_documents: false,
        },
    );
    rows.insert(
        "3",
        Student {
            kanji_name: "鈴木一郎".to_string(),
            kana_name: "すずきいちろう".to_string(),
            english_name: "Ichiro Suzuki".to_string(),
            class: "1C".to_string(),
            program: "Long Term".to_string(),
            completed_all_documents: true,
        },
    );

    let mut table = Column::new().push(table_header);
    for value in rows.into_values() {
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
    let test_student = Student {
        kanji_name: "山田太郎".to_string(),
        kana_name: "やまだたろう".to_string(),
        english_name: "Taro Yamada".to_string(),
        class: "3A".to_string(),
        program: "Dream Builder".to_string(),
        completed_all_documents: true,
    };
    let student_info_column = |student: Student| {
        column![
            rich_text([span(student.kanji_name.to_string()).font(NOTO_SANS_JP),])
                .width(FillPortion(1)),
            rich_text([span(student.kana_name.to_string()).font(NOTO_SANS_JP),])
                .width(FillPortion(1)),
            rich_text([span(student.english_name.to_string()).font(NOTO_SANS_JP),])
                .width(FillPortion(1)),
            rich_text([span(student.class.to_string()).font(NOTO_SANS_JP),]).width(FillPortion(1)),
            rich_text([span(student.program.to_string()).font(NOTO_SANS_JP),])
                .width(FillPortion(1)),
        ]
        .width(FillPortion(1))
    };
    let student_profile_image =
        |img: &str| column![image(img).width(250).height(250)].width(FillPortion(3));

    let profile_row = row![
        student_profile_image("src/ant.jpeg"),
        student_info_column(test_student)
    ];
    Container::new(profile_row)
        .style(|_| container::Style {
            border: iced::Border {
                color: color!(0xC0, 0xC0, 0xC0),
                width: 3.0,
                radius: Radius::new(5),
            },
            shadow: Shadow {
                color: color!(0, 0, 0, 1.),
                offset: iced::Vector { x: 10., y: 3. },
                blur_radius: 10.,
            },
            background: Some(Background::Color(color!(0, 0, 0, 1.))),
            ..Default::default()
        })
        .width(600)
        .into()
}
