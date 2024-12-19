use crate::models::student::Student;
use crate::{Message, NOTO_SANS_JP};
use iced::widget::{center, column, container, image, rich_text, row, span, Container};
use iced::Element;
use iced::Length::FillPortion;

pub fn profile_info() -> Element<'static, Message> {
    let test_student = Student {
        kanji_name: "山田太郎".to_string(),
        kana_name: "やまだたろう".to_string(),
        english_name: "Taro Yamada".to_string(),
        class: "3A".to_string(),
        program: "Dream Builder".to_string(),
        completed_all_documents: true,
    };

    let student_info_column = |student: Student| {
        let make_text =
            |content: String, size: u16| rich_text([span(content).font(NOTO_SANS_JP).size(size)]);

        column![
            make_text(student.kanji_name.to_string(), 42),
            make_text(student.kana_name.to_string(), 28),
            make_text(student.english_name.to_string(), 18),
            make_text(student.class.to_string(), 16),
            make_text(student.program.to_string(), 16),
        ]
        .width(FillPortion(2))
    };

    let student_profile_image = |img: &str| {
        container(image(img).width(200).height(200))
            .style(|_| container::Style {
                border: iced::Border {
                    color: iced::Color::from_rgb(0., 0., 255.),
                    width: 10.,
                    radius: iced::border::Radius::new(15),
                },
                background: Some(iced::Background::Color(iced::Color::from_rgb(0., 0., 255.))),
                ..Default::default()
            })
            .width(200)
            .padding(30)
            .clip(true)
    };

    let profile_row = row![
        student_profile_image("src/ant.jpeg"),
        center(student_info_column(test_student))
    ];

    Container::new(profile_row).max_height(400).into()
}
