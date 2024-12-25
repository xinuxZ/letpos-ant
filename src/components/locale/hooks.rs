// use super::context::use_locale_config;
use super::*;
use crate::components::locale::{
    defaults::{get_en_us_texts, get_zh_cn_texts},
    types::{Language, LocaleText},
};
use leptos::prelude::*;

/// Hook to access locale configuration
pub fn use_locale_config() -> Option<LocaleContext> {
    use_context::<LocaleContext>()
}

/// Hook to get current language
pub fn use_language() -> Option<ReadSignal<Language>> {
    use_locale_config().map(|ctx| {
        let (read, _) = create_signal(ctx.get().language);
        read
    })
}

/// Hook to get current locale text
pub fn use_locale_text() -> Option<ReadSignal<LocaleText>> {
    use_locale_config().map(|ctx| {
        let config = ctx.get();
        let (read, _) = create_signal(config.texts.unwrap_or_else(|| {
            // 根据当前语言获取默认文本
            match config.language {
                Language::EnUS => get_en_us_texts(),
                Language::ZhCN => get_zh_cn_texts(),
            }
        }));
        read
    })
}

/// Hook to format dates
pub fn use_date_formatter() -> impl Fn(chrono::DateTime<chrono::Utc>, Option<&str>) -> String {
    let locale_config = use_locale_config().expect("LocaleProvider not found");

    move |date, format| {
        let config = locale_config.get();
        format_date(&date, format.unwrap_or_default(), &config)
    }
}

/// Hook to format numbers
pub fn use_number_formatter() -> impl Fn(f64, Option<NumberFormatType>) -> String {
    let locale_config = use_locale_config().expect("LocaleProvider not found");

    move |number, format_type| {
        let config = locale_config.get();
        format_number(number, &config.number_format, format_type)
    }
}
