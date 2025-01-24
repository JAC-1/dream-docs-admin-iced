use crate::models::supabase_models::*;

use crate::{Message, View, NOTO_SANS_JP};
use iced::advanced::graphics::core::font;
use iced::widget::{button, horizontal_rule, row, text, Column, Scrollable};
use iced::Alignment::Center;
use iced::Length::{Fill, FillPortion};
use iced::{Element, Font, Padding};

// You can leverage the From trait to build Padding conveniently:
// let padding = Padding::from(20);              // 20px on all sides
// let padding = Padding::from([10, 20]);        // top/bottom, left/right

pub fn students_table(
    students_profile_data: &Vec<StudentProfileData>,
) -> Element<'static, Message> {
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

    let table_headers = ["学籍番号", "表示名", "クラス", "プログラム", "書類完了"];

    let table_header = row(table_headers
        .iter()
        .map(|header| create_header_text(header.to_string()))
        .collect::<Vec<_>>())
    .align_y(Center)
    .width(Fill)
    .padding(3);

    fn create_student_row(student: &StudentProfileData) -> Element<'static, Message> {
        let padding = Padding::from([2, 0]);
        // TODO: FEATURE - text buttons for click to copy to clipboard functionality
        button(row![
            text(student.display_id.clone())
                .font(NOTO_SANS_JP)
                .width(FillPortion(1)),
            text(student.display_name.clone())
                .font(NOTO_SANS_JP)
                .center()
                .width(FillPortion(1)),
            text(
                student
                    .classes
                    .get("title")
                    .unwrap_or(&String::from("No class"))
                    .clone()
            )
            .font(NOTO_SANS_JP)
            .width(FillPortion(1)),
            text(
                student
                    .programs
                    .get("name")
                    .unwrap_or(&String::from("No Class"))
                    .clone()
            )
            .font(NOTO_SANS_JP)
            .width(FillPortion(1)),
            text!("未完了") // Document status added
                .font(NOTO_SANS_JP)
                .center()
                .width(FillPortion(1)),
        ])
        .style(iced::widget::button::secondary)
        .on_press(Message::SelectAndViewStudent(student.clone()))
        .padding(padding)
        .into()
    }

    let table = Column::new().push(table_header).push(horizontal_rule(20));
    let mut students_table = Column::new();
    for student in students_profile_data {
        let student_row = create_student_row(student);
        students_table = students_table.push(student_row);
    }
    let scrollable_students_table = Scrollable::new(students_table);
    table.push(scrollable_students_table).into()
}
