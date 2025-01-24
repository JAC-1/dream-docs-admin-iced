use std::ops::Deref;

use crate::models::supabase_models::*;
use crate::operations::FileToSave;
use crate::types::FileStatus;
use crate::Message;
use iced::advanced::graphics::core::font;
use iced::widget::{button, column, rich_text, row, scrollable, span, text};
use iced::{Element, FillPortion, Font};

pub fn student_documents_table(
    docs: &Vec<File>,
    student: &StudentProfileData,
) -> Element<'static, Message> {
    let header_text = |text: String| {
        rich_text([span(text).font(Font {
            family: iced::font::Family::Name("Noto Sans Jp"),
            weight: font::Weight::Bold,
            ..Default::default()
        })])
        .width(FillPortion(1))
    };

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
            button("Download")
                .width(FillPortion(1))
                .on_press(Message::FetchStudentDoc(doc.clone()))
        ]
    };

    let mut main_container = column![document_header];
    let mut docs_container = column![];

    for doc in docs.clone() {
        let doc_row = document_row(&doc);
        docs_container = docs_container.push(doc_row);
    }
    let scrollable_docs = scrollable(docs_container);
    main_container = main_container.push(scrollable_docs);
    main_container.into()
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
