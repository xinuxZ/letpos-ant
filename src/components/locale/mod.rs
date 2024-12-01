use leptos::prelude::*;
use std::collections::HashMap;

/// Configuration for localization
#[derive(Clone, Debug)]
pub struct LocaleConfig {
    /// Current locale code (e.g., "zh_CN")
    pub locale: String,
    /// Key-value pairs of localized messages
    pub messages: HashMap<String, String>,
}

impl Default for LocaleConfig {
    fn default() -> Self {
        let mut messages = HashMap::new();
        messages.insert("ok".to_string(), "确定".to_string());
        messages.insert("cancel".to_string(), "取消".to_string());

        Self {
            locale: "zh_CN".to_string(),
            messages,
        }
    }
}

/// Locale provider component for internationalization
#[component]
pub fn Locale(
    /// Optional locale configuration
    #[prop(optional)]
    config: Option<LocaleConfig>,
    /// Child components
    children: Children,
) -> impl IntoView {
    let locale = RwSignal::new(config.unwrap_or_default());

    provide_context(locale);

    view! {
        <div class="ant-locale">
            {children()}
        </div>
    }
}

/// Hook to access the current locale configuration
pub fn use_locale() -> Option<ReadSignal<LocaleConfig>> {
    use_context::<RwSignal<LocaleConfig>>().map(|locale| locale.read_only())
}
