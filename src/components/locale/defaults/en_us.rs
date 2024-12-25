//! 英语（美国）的默认文本定义

use crate::components::locale::types::*;

/// 获取英语（美国）的默认文本
pub fn get_default_texts() -> LocaleText {
    LocaleText {
        locale: "en-US".to_string(),
        placeholder: "Please select".to_string(),
        select_placeholder: "Please select".to_string(),
        modal: ModalLocaleText {
            ok_text: "OK".to_string(),
            cancel_text: "Cancel".to_string(),
            just_ok_text: "OK".to_string(),
        },
        popconfirm: PopconfirmLocaleText {
            ok: "OK".to_string(),
            cancel: "Cancel".to_string(),
            yes: "Yes".to_string(),
            no: "No".to_string(),
        },
        form: FormLocaleText {
            optional: "Optional".to_string(),
            required: "Required".to_string(),
        },
        table: TableLocaleText {
            filter_title: "Filter".to_string(),
            empty_text: "No data".to_string(),
        },
        upload: UploadLocaleText {
            upload_text: "Click to upload".to_string(),
            remove_text: "Remove".to_string(),
        },
        empty: EmptyLocaleText {
            description: "No data".to_string(),
        },
    }
}
