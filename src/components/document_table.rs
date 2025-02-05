use crate::models::supabase_models::*;
use crate::types::FileStatus;
use crate::Message;
use iced::widget::{button, column, pick_list, row, scrollable, span, text, Container, Space};
use iced::{Center, Element, Fill, FillPortion};

pub fn student_documents_table(docs: &Vec<File>) -> Element<'static, Message> {
    if docs.is_empty() {
        return Container::new(text("No Documents Found").size(24))
            .width(Fill)
            .height(Fill)
            .center(Fill)
            .into();
    }
    // let download_all_button = button("Download All").on_press(Message::DownloadAllDocs);
    // let mut main_container = column![download_all_button];
    let mut docs_container = column![].spacing(11);
    let mut doc_row = row![].spacing(11);
    let mut count = 0;

    for doc in docs.clone() {
        let doc_card = document_card(&doc);
        doc_row = doc_row.push(doc_card);
        count += 1;
        if count == 3 {
            docs_container = docs_container.push(doc_row.push(Space::new(20, 0)));
            doc_row = row![].spacing(11);
            count = 0;
        }
    }

    if count > 0 {
        docs_container = docs_container.push(doc_row);
    }

    let scrollable_docs = scrollable(docs_container);
    let main_container = Container::new(scrollable_docs)
        .padding([0, 78])
        .width(Fill)
        .center(Fill);
    main_container.into()
}

fn document_card(doc_info: &File) -> Element<'static, Message> {
    let date = doc_info
        .updated_at
        .clone()
        .format("%Y - %m - %d")
        .to_string();
    let doc_name = text(doc_info.task_type.to_string().clone()).size(20);
    let date = text(date).size(18);
    let status = status_indicator(doc_info.status.clone(), doc_info.document_id.clone());
    let download = button(text!("Download").size(14).height(29).width(Fill).center())
        .width(Fill)
        .on_press(Message::FetchStudentDoc(doc_info.clone()));

    let doc_info_column = column![doc_name, date].align_x(Center);

    let main_doc_column = column![doc_info_column, status, download]
        .padding(21)
        .spacing(10)
        .align_x(Center);

    Container::new(main_doc_column)
        .max_height(200)
        .height(200)
        .width(170)
        .max_width(170)
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
