use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported languages
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    /// Chinese (Simplified)
    ZhCN,
    /// English (US)
    EnUS,
}

/// Localized text content
#[derive(Clone, Debug, PartialEq)]
pub struct LocaleText {
    /// Current locale code
    pub locale: String,
    /// Default placeholder text
    pub placeholder: String,
    /// Select component placeholder
    pub select_placeholder: String,

    /// Modal dialog texts
    pub modal: ModalLocaleText,
    /// Popconfirm component texts
    pub popconfirm: PopconfirmLocaleText,
    /// Form component texts
    pub form: FormLocaleText,
    /// Table component texts
    pub table: TableLocaleText,
    /// Upload component texts
    pub upload: UploadLocaleText,
    /// Empty state texts
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

/// Modal dialog localized texts
#[derive(Clone, Debug, PartialEq)]
pub struct ModalLocaleText {
    /// OK button text
    pub ok_text: String,
    /// Cancel button text
    pub cancel_text: String,
    /// Just OK button text
    pub just_ok_text: String,
}

/// Popconfirm component localized texts
#[derive(Clone, Debug, PartialEq)]
pub struct PopconfirmLocaleText {
    /// OK button text
    pub ok: String,
    /// Cancel button text
    pub cancel: String,
    /// Yes button text
    pub yes: String,
    /// No button text
    pub no: String,
}

/// Form component localized texts
#[derive(Clone, Debug, PartialEq)]
pub struct FormLocaleText {
    /// Optional field text
    pub optional: String,
    /// Required field text
    pub required: String,
}

/// Table component localized texts
#[derive(Clone, Debug, PartialEq)]
pub struct TableLocaleText {
    /// Filter menu title
    pub filter_title: String,
    /// Empty data text
    pub empty_text: String,
}

/// Upload component localized texts
#[derive(Clone, Debug, PartialEq)]
pub struct UploadLocaleText {
    /// Upload button text
    pub upload_text: String,
    /// Remove file text
    pub remove_text: String,
}

/// Empty state localized texts
#[derive(Clone, Debug, PartialEq)]
pub struct EmptyLocaleText {
    /// Empty state description
    pub description: String,
}

/// RTL configuration
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RTLConfig {
    /// Whether RTL is enabled
    pub enabled: bool,
}

/// Number format configuration
#[derive(Clone, Debug, PartialEq)]
pub struct NumberFormatConfig {
    /// Currency symbol
    pub currency_symbol: String,
}

impl Default for NumberFormatConfig {
    fn default() -> Self {
        Self {
            currency_symbol: "$".to_string(),
        }
    }
}

/// Locale configuration
#[derive(Clone, Debug, PartialEq)]
pub struct LocaleConfig {
    /// Current language
    pub language: Language,
    /// Number format settings
    pub number_format: NumberFormatConfig,
    /// Custom locale texts
    pub texts: Option<LocaleText>,
    /// RTL support configuration
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

/// Locale context type for configuration
pub type LocaleContext = RwSignal<LocaleConfig>;
