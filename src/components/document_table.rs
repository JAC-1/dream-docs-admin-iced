use crate::models::supabase_models::*;
use crate::types::FileStatus;
use crate::Message;
use iced::advanced::graphics::core::font;
use iced::widget::{button, column, rich_text, row, span, text};
use iced::{Element, FillPortion, Font};

pub fn student_documents_table(docs: &Vec<File>) -> Element<'static, Message> {
    let header_text = |text: String| {
        rich_text([span(text).font(Font {
            family: iced::font::Family::Name("Noto Sans Jp"),
            weight: font::Weight::Bold,
            ..Default::default()
        })])
        .width(FillPortion(1))
    };
    // id: String,
    // document_id: String,
    // file_path: String,
    // file_name: String,
    // file_size: i64,
    // task_type: TaskType,
    // mime_type: String,
    // status: FileStatus,
    // status_message: Option<String>,
    // user_id: String,
    // processing_attempts: i32,
    // created_at: DateTime<Local>,
    // updated_at: DateTime<Local>,

    let document_header = row![
        header_text("Document ID".to_string()),
        header_text("Task Type".to_string()),
        header_text("Status".to_string()),
        header_text("Mime Type".to_string()),
        header_text("Status Message".to_string()),
        header_text("Created At".to_string()),
        header_text("Updated At".to_string()),
        header_text("Download".to_string())
    ];

    let document_row = |doc: &File| {
        row![
            text(doc.document_id.clone()).width(FillPortion(1)), // Document ID
            text(doc.task_type.to_str().to_string()).width(FillPortion(1)), // Task Type
            status_indicator(&doc.status),                       // Status
            text(doc.mime_type.clone()).width(FillPortion(1)),   // Mime Type
            text(doc.status_message.clone().unwrap_or_default()).width(FillPortion(1)), // Status Message
            text(doc.created_at.to_string()).width(FillPortion(1)), // Created At
            text(doc.updated_at.to_string()).width(FillPortion(1)), // Updated At
            button("Download").width(FillPortion(1))                // Download
        ]
    };

    // let study_abroad_application =
    //     |status: &DocumentStatus| document_row("Personal Information Waver".to_string(), status);

    // let personal_information_waver =
    //     |status: &DocumentStatus| document_row("Study Abroad Application".to_string(), status);
    // container(column![
    //     document_header,
    //     study_abroad_application(&FileStatus::Approved),
    //     personal_information_waver(&FileStatus::Pending)
    // ])
    // .center_x(Fill)
    // .padding(40)
    // .into()
    let mut container = column![document_header];
    container = container.push(text("Sorry Nothing"));
    for doc in docs {
        let doc_row = document_row(&doc);
        container = container.push(doc_row);
    }
    container.into()

    // container(column![document_header])
    //     .center_x(Fill)
    //     .padding(40)
    //     .into()
}

fn status_indicator(status: &FileStatus) -> Element<'static, Message> {
    let text = match status {
        FileStatus::New => "Pending ⚪",
        FileStatus::Pending => "Submitted 🟡",
        FileStatus::Approved => "Approved 🟢",
        FileStatus::Declined => "Declined 🔴",
    };

    rich_text([span(text)]).width(FillPortion(1)).into()
}
