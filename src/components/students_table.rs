use crate::models::supabase_models::StudentProfileData;
use crate::{Message, NOTO_SANS_JP};
use iced::advanced::graphics::core::font;
use iced::widget::{button, horizontal_rule, row, text, Column};
use iced::Alignment::Center;
use iced::Length::{Fill, FillPortion};
use iced::{Element, Font};

// You can leverage the From trait to build Padding conveniently:
// let padding = Padding::from(20);              // 20px on all sides
// let padding = Padding::from([10, 20]);        // top/bottom, left/right

pub fn students_table(// student_profile_data: Option<StudentProfileData>,
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

    let test_student = StudentProfileData {
        display_id: "123980981".to_string(),
        display_name: "Taro".to_string(),
        class: "Amazing Class".to_string(),
        program: "Amazing Program".to_string(),
    };

    fn create_student_row(student: StudentProfileData) -> Element<'static, Message> {
        button(row![
            text(student.display_id.clone())
                .font(NOTO_SANS_JP)
                .width(FillPortion(1)),
            text(student.display_name.clone())
                .font(NOTO_SANS_JP)
                .width(FillPortion(1)),
            text(student.class.clone())
                .font(NOTO_SANS_JP)
                .width(FillPortion(1)),
            text(student.program.clone())
                .font(NOTO_SANS_JP)
                .width(FillPortion(1)),
            text!("未完了") // Document status added
                .font(NOTO_SANS_JP)
                .width(FillPortion(1)),
        ])
        .style(iced::widget::button::secondary)
        .on_press(Message::NavigatetoStudentProfile(student))
        .into()
    }

    let table = Column::new().push(table_header).push(horizontal_rule(20));
    let test_row = create_student_row(test_student);
    table.push(test_row).into()
}
