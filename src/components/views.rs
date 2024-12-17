use crate::Message;
use iced::border::Radius;
use iced::daemon::DefaultStyle;
use iced::widget::button::{Status, Style};
use iced::widget::text::Wrapping;
use iced::widget::{column, row, Button, Container, Text};
use iced::{Background, Border, Color, Shadow};
use iced::{Element, Fill, Theme};

// use super::button::ProgramsButton;
//

pub fn custom_program_button(theme: &Theme, status: Status) -> Style {
    match status {
        Status::Active | Status::Pressed => Style {
            background: Some(Background::Color(
                theme.extended_palette().primary.base.color,
            )),
            text_color: theme.extended_palette().primary.base.text,
            border: Border {
                radius: Radius::from(15.0),
                ..Border::default()
            },
            shadow: Shadow::default(),
        },
        Status::Hovered => Style {
            background: Some(Background::Color(
                theme.extended_palette().primary.weak.color,
            )),
            text_color: theme.extended_palette().primary.base.text,
            border: Border {
                radius: Radius::from(15.0),
                ..Border::default()
            },
            shadow: Shadow::default(),
        },
        Status::Disabled => Style {
            background: Some(Background::Color(
                theme.extended_palette().background.weak.color,
            )),
            text_color: theme.extended_palette().background.base.text,
            border: Border {
                radius: Radius::from(15.0),
                ..Border::default()
            },
            shadow: Shadow::default(),
        },
    }
}

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
    Container::new(Text::new("Hello World").size(50).center())
        .center_y(Fill)
        .center_x(Fill)
        .into()
}
