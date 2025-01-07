use crate::models::supabase_models::StudentProfileData;
use crate::Message;
use iced::advanced::graphics::core::font;
use iced::widget::{center, column, container, image, row, text, Container};
use iced::Length::FillPortion;
use iced::{Element, Font};

pub fn profile_info(student: &StudentProfileData) -> Element<'static, Message> {
    let student_info_column = |student: &StudentProfileData| {
        let make_text =
            // |content: String, size: u16| rich_text([span(content).font(NOTO_SANS_JP).size(size)]);
            |content: String, size: u16|  text::Text::new(content).font(Font {
                family: font::Family::Name("Noto Sans JP"),
                ..Default::default()}).size(size);

        column![
            make_text(student.display_name.to_string(), 42),
            make_text(student.display_id.to_string(), 28),
            make_text(student.display_name.to_string(), 18),
            make_text(
                student
                    .classes
                    .get("title")
                    .unwrap_or(&String::from("No Class"))
                    .to_string(),
                16
            ),
            make_text(
                student
                    .programs
                    .get("name")
                    .unwrap_or(&String::from("No Program"))
                    .to_string(),
                16
            ),
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
        center(student_info_column(student))
    ];

    Container::new(profile_row).max_height(400).into()
}
