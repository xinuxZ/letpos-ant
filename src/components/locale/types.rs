//! 本地化相关的类型定义

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// 支持的语言类型
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    /// 中文（简体）
    ZhCN,
    /// 英文（美国）
    EnUS,
}

/// 本地化文本内容
#[derive(Clone, Debug, PartialEq)]
pub struct LocaleText {
    /// 当前语言代码
    pub locale: String,
    /// 默认占位符文本
    pub placeholder: String,
    /// Select 组件占位符
    pub select_placeholder: String,

    /// 模态框文本
    pub modal: ModalLocaleText,
    /// Popconfirm 组件文本
    pub popconfirm: PopconfirmLocaleText,
    /// 表单组件文本
    pub form: FormLocaleText,
    /// 表格组件文本
    pub table: TableLocaleText,
    /// 上传组件文本
    pub upload: UploadLocaleText,
    /// 空状态文本
    pub empty: EmptyLocaleText,
}

impl Default for LocaleText {
    fn default() -> Self {
        Self {
            locale: String::new(),
            placeholder: String::new(),
            select_placeholder: String::new(),
            modal: ModalLocaleText {
                ok_text: String::new(),
                cancel_text: String::new(),
                just_ok_text: String::new(),
            },
            popconfirm: PopconfirmLocaleText {
                ok: String::new(),
                cancel: String::new(),
                yes: String::new(),
                no: String::new(),
            },
            form: FormLocaleText {
                optional: String::new(),
                required: String::new(),
            },
            table: TableLocaleText {
                filter_title: String::new(),
                empty_text: String::new(),
            },
            upload: UploadLocaleText {
                upload_text: String::new(),
                remove_text: String::new(),
            },
            empty: EmptyLocaleText {
                description: String::new(),
            },
        }
    }
}

/// 模态框本地化文本
#[derive(Clone, Debug, PartialEq)]
pub struct ModalLocaleText {
    /// 确认按钮文本
    pub ok_text: String,
    /// 取消按钮文本
    pub cancel_text: String,
    /// 仅确认按钮文本
    pub just_ok_text: String,
}

/// Popconfirm 组件本地化文本
#[derive(Clone, Debug, PartialEq)]
pub struct PopconfirmLocaleText {
    /// 确认按钮文本
    pub ok: String,
    /// 取消按钮文本
    pub cancel: String,
    /// 是按钮文本
    pub yes: String,
    /// 否按钮文本
    pub no: String,
}

/// 表单组件本地化文本
#[derive(Clone, Debug, PartialEq)]
pub struct FormLocaleText {
    /// 可选字段文本
    pub optional: String,
    /// 必填字段文本
    pub required: String,
}

/// 表格组件本地化文本
#[derive(Clone, Debug, PartialEq)]
pub struct TableLocaleText {
    /// 筛选菜单标题
    pub filter_title: String,
    /// 空数据文本
    pub empty_text: String,
}

/// 上传组件本地化文本
#[derive(Clone, Debug, PartialEq)]
pub struct UploadLocaleText {
    /// 上传按钮文本
    pub upload_text: String,
    /// 移除文件文本
    pub remove_text: String,
}

/// 空状态本地化文本
#[derive(Clone, Debug, PartialEq)]
pub struct EmptyLocaleText {
    /// 空状态描述
    pub description: String,
}

/// RTL 配置
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RTLConfig {
    /// 是否启用 RTL
    pub enabled: bool,
}

/// 数字格式化配置
#[derive(Clone, Debug, PartialEq)]
pub struct NumberFormatConfig {
    /// 货币符号
    pub currency_symbol: String,
}

impl Default for NumberFormatConfig {
    fn default() -> Self {
        Self {
            currency_symbol: "$".to_string(),
        }
    }
}

/// 本地化配置
#[derive(Clone, Debug, PartialEq)]
pub struct LocaleConfig {
    /// 当前语言
    pub language: Language,
    /// 数字格式化设置
    pub number_format: NumberFormatConfig,
    /// 自定义本地化文本
    pub texts: Option<LocaleText>,
    /// RTL 支持配置
    pub rtl: RTLConfig,
}

impl Default for LocaleConfig {
    fn default() -> Self {
        Self {
            language: Language::ZhCN,
            number_format: NumberFormatConfig::default(),
            texts: None,
            rtl: RTLConfig::default(),
        }
    }
}

/// 本地化上下文类型
pub type LocaleContext = RwSignal<LocaleConfig>;
