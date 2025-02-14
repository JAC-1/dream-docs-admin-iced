use crate::models::supabase_models::*;
use crate::types::FileStatus;
use crate::Message;
use iced::widget::{button, column, pick_list, row, scrollable, text, Container, Space};
use iced::{Alignment, Center, Element, Fill};

pub fn student_documents_table(docs: &Vec<File>) -> Element<'static, Message> {
    if docs.is_empty() {
        return Container::new(text("No Documents Found").size(24))
            .width(Fill)
            .height(Fill)
            .center(Fill)
            .into();
    }
    let download_all_button = button("Download All")
        .on_press(Message::DownloadAllDocs)
        .padding(10);
    let header = row![Space::new(0, 0), download_all_button]
        .align_y(Center)
        .padding(10);
    let mut docs_container = column![].spacing(11);
    let mut doc_row = row![].spacing(11);
    let mut count = 0;

    for doc in docs.clone() {
        let doc_card = document_card(&doc);
        doc_row = doc_row.push(doc_card);
        count += 1;
        // Change count check value to increase the number of cards per row
        if count == 4 {
            docs_container = docs_container.push(doc_row.push(Space::new(20, 0)));
            doc_row = row![].spacing(11);
            count = 0;
        }
    }

    if count > 0 {
        docs_container = docs_container.push(doc_row);
    }

    let scrollable_docs = scrollable(docs_container);
    let main_column = column![header, scrollable_docs]
        .width(Fill)
        .height(Fill)
        .align_x(Center);
    let main_container = Container::new(main_column)
        .padding([0, 78])
        .width(Fill)
        .center(Fill);
    main_container.into()
}

fn document_card(doc_info: &File) -> Element<'static, Message> {
    let clean_doc_info = doc_info.task_type.to_string().clone().replace("_", " ");
    let shortened_doc_title =  truncate_with_ellipsis(doc_info.file_name.to_string().clone(), 20);


    let date = doc_info
        .updated_at
        .clone()
        .format("%Y - %m - %d")
        .to_string();
    let time = text(doc_info.updated_at.clone().format("%H:%M:%S").to_string()).size(12);
    let doc_name = text(clean_doc_info).size(20);
    let file_name = text(shortened_doc_title).size(12);
    let date = text(date).size(16);
    let status = status_indicator(doc_info.status.clone(), doc_info.document_id.clone());
    let download = button(text!("Download").size(14).height(29).width(Fill).center())
        .width(Fill)
        .on_press(Message::FetchStudentDoc(doc_info.clone()));

    let doc_info_column = column![doc_name, file_name, date, time].align_x(Center);

    let main_doc_column = column![doc_info_column, status, download]
        .padding(21)
        .spacing(10)
        .align_x(Center);

    Container::new(main_doc_column)
        .max_height(250)
        .height(250)
        .width(200)
        .max_width(200)
        .style(|_| iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb8(
                248, 248, 248,
            ))),
            ..Default::default()
        })
        .into()
}

fn status_indicator(status: FileStatus, document_id: String) -> Element<'static, Message> {
    // let text = match status {
    //     FileStatus::New => "Pending ⚪",
    //     FileStatus::Pending => "Submitted 🟡",
    //     FileStatus::Approved => "Approved 🟢",
    //     FileStatus::Declined => "Declined 🔴",
    // };
    pick_list(&FileStatus::ALL[..], Some(status), move |selected_status| {
        Message::DocumentStatusSelected(selected_status, document_id.clone())
    })
    .into()
    // rich_text([span("Error! Please consult Justin")])
    //     .width(FillPortion(1))
    //     .into()
}

fn truncate_with_ellipsis(s: String, max_len: usize) -> String {
    if s.len() <= max_len {
       s
    } else {
        let mut truncated = s[..max_len].to_string();
        truncated.push_str("...");
        truncated
    }
}
