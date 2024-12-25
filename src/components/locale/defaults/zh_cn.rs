//! 中文（简体）的默认文本定义

use crate::components::locale::types::*;

/// 获取中文（简体）的默认文本
pub fn get_default_texts() -> LocaleText {
    LocaleText {
        locale: "zh-CN".to_string(),
        placeholder: "请选择".to_string(),
        select_placeholder: "请选择".to_string(),
        modal: ModalLocaleText {
            ok_text: "确定".to_string(),
            cancel_text: "取消".to_string(),
            just_ok_text: "确定".to_string(),
        },
        popconfirm: PopconfirmLocaleText {
            ok: "确定".to_string(),
            cancel: "取消".to_string(),
            yes: "是".to_string(),
            no: "否".to_string(),
        },
        form: FormLocaleText {
            optional: "可选".to_string(),
            required: "必填".to_string(),
        },
        table: TableLocaleText {
            filter_title: "筛选".to_string(),
            empty_text: "暂无数据".to_string(),
        },
        upload: UploadLocaleText {
            upload_text: "点击上传".to_string(),
            remove_text: "移除".to_string(),
        },
        empty: EmptyLocaleText {
            description: "暂无数据".to_string(),
        },
    }
}
