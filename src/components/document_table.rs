use crate::Message;
use crate::NOTO_SANS_JP;
use iced::font::Weight;
use iced::widget::{button, column, container, rich_text, row, span, text};
use iced::{Element, Fill, FillPortion, Font};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum DocumentStatus {
    Pending,
    Submitted,
    Complete,
    Declined,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Document {
    name: String,
    status: DocumentStatus,
    download: String,
}

pub fn student_documents_table() -> Element<'static, Message> {
    let bold_font = Font {
        weight: Weight::Bold,
        ..Default::default()
    };

    let header_text = |text: String| {
        rich_text([span(text).font(NOTO_SANS_JP).font(bold_font)]).width(FillPortion(1))
    };

    let document_header = row![
        header_text("Document".to_string()),
        header_text("Status".to_string()),
        header_text("Download".to_string())
    ];

    let document_row = |name: String, status: &DocumentStatus| {
        row![
            text(name).width(FillPortion(1)),
            status_indicator(status),
            button("Download").width(FillPortion(1))
        ]
    };

    let study_abroad_application =
        |status: &DocumentStatus| document_row("Personal Information Waver".to_string(), status);

    let personal_information_waver =
        |status: &DocumentStatus| document_row("Study Abroad Application".to_string(), status);
    container(column![
        document_header,
        study_abroad_application(&DocumentStatus::Complete),
        personal_information_waver(&DocumentStatus::Pending)
    ])
    .center_x(Fill)
    .padding(40)
    .into()
}

fn status_indicator(status: &DocumentStatus) -> Element<'static, Message> {
    let text = match status {
        DocumentStatus::Pending => "Pending ⚪",
        DocumentStatus::Submitted => "Submitted 🟡",
        DocumentStatus::Complete => "Complete 🟢",
        DocumentStatus::Declined => "Declined 🔴",
    };

    rich_text([span(text).font(NOTO_SANS_JP)])
        .width(FillPortion(1))
        .into()
}
