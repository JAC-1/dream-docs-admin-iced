use crate::models::supabase_models::StudentProfileData;
use crate::Message;
use iced::advanced::graphics::core::font;
use iced::widget::{column, row, text, Container, Space};
use iced::{Center, Element, Fill, Font};

pub fn profile_info(student: &StudentProfileData) -> Element<'static, Message> {
    let make_text = |content: String, size: u16| {
        text::Text::new(content)
            .font(Font {
                family: font::Family::Name("Noto Sans JP"),
                ..Default::default()
            })
            .size(size)
    };

    let name = make_text(student.display_name.to_string(), 88);
    let program = make_text(
        student
            .programs
            .get("name")
            .unwrap_or(&String::from("No Program"))
            .to_string(),
        18,
    );
    let class = make_text(
        student
            .classes
            .get("title")
            .unwrap_or(&String::from("No Class"))
            .to_string(),
        16,
    );
    let class_name = text!("R4-3M").size(12);

    let space = Space::new(90, 0); // "Nudges" the row so to center it over the document
                                   // section
    let class_column = column![class, class_name].padding([30, 20]);
    let name_column = column![name, program].align_x(Center);
    let main = row![space, name_column, class_column];

    let name_container = Container::new(main).width(1024).center_x(Fill).padding(115);

    name_container.into()

    // let student_info_column = |student: &StudentProfileData| {
    //     let make_text = |content: String, size: u16| {
    //         text::Text::new(content)
    //             .font(Font {
    //                 family: font::Family::Name("Noto Sans JP"),
    //                 ..Default::default()
    //             })
    //             .size(size)
    //     };
    //
    //     column![
    //         make_text(student.display_name.to_string(), 42),
    //         make_text(student.display_id.to_string(), 28),
    //         make_text(student.display_name.to_string(), 18),
    //         make_text(
    //             student
    //                 .classes
    //                 .get("title")
    //                 .unwrap_or(&String::from("No Class"))
    //                 .to_string(),
    //             16
    //         ),
    //         make_text(
    //             student
    //                 .programs
    //                 .get("name")
    //                 .unwrap_or(&String::from("No Program"))
    //                 .to_string(),
    //             16
    //         ),
    //     ]
    //     .width(FillPortion(2))
    // };
    //
    // let student_profile_image = |img: &str| {
    //     container(image(img).width(200).height(200))
    //         .style(|_| container::Style {
    //             border: iced::Border {
    //                 color: iced::Color::from_rgb(0., 0., 255.),
    //                 width: 10.,
    //                 radius: iced::border::Radius::new(15),
    //             },
    //             background: Some(iced::Background::Color(iced::Color::from_rgb(0., 0., 255.))),
    //             ..Default::default()
    //         })
    //         .width(200)
    //         .padding(30)
    //         .clip(true)
    // };
}
