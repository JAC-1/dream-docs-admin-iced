use iced::border::Radius;
use iced::theme::Theme;
use iced::widget::button::{Status, Style};
use iced::{Background, Border, Shadow};

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

pub fn student_row(theme: &Theme, status: Status) -> Style {
    match status {
        Status::Active | Status::Pressed => Style {
            background: Some(Background::Color(
                theme.extended_palette().background.base.color,
            )),
            text_color: theme.extended_palette().background.base.text,
            border: Border::default(),
            shadow: Shadow::default(),
        },
        Status::Hovered => Style {
            background: Some(Background::Color(
                theme.extended_palette().background.weak.color,
            )),
            text_color: theme.extended_palette().background.base.text,
            border: Border::default(),
            shadow: Shadow::default(),
        },
        Status::Disabled => Style {
            background: Some(Background::Color(
                theme.extended_palette().background.weak.color,
            )),
            text_color: theme.extended_palette().background.base.text,
            border: Border::default(),
            shadow: Shadow::default(),
        },
    }
}
