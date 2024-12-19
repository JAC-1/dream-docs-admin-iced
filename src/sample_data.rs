use crate::models::student::Student;
use std::collections::HashMap;

pub fn get_sample_data() -> HashMap<&'static str, Student> {
    let mut rows = HashMap::new();
    rows.insert(
        "1",
        Student {
            kanji_name: "山田太郎".to_string(),
            kana_name: "やまだたろう".to_string(),
            english_name: "Taro Yamada".to_string(),
            class: "3A".to_string(),
            program: "Dream Builder".to_string(),
            completed_all_documents: true,
        },
    );
    rows.insert(
        "2",
        Student {
            kanji_name: "佐藤花子".to_string(),
            kana_name: "さとうはなこ".to_string(),
            english_name: "Hanako Sato".to_string(),
            class: "2B".to_string(),
            program: "Term Program".to_string(),
            completed_all_documents: false,
        },
    );
    rows.insert(
        "3",
        Student {
            kanji_name: "鈴木一郎".to_string(),
            kana_name: "すずきいちろう".to_string(),
            english_name: "Ichiro Suzuki".to_string(),
            class: "1C".to_string(),
            program: "Long Term".to_string(),
            completed_all_documents: true,
        },
    );
    rows
}
