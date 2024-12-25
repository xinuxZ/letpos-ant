use super::*;
use leptos::prelude::*;

/// Locale provider component with enhanced configuration
#[component]
pub fn LocaleProvider(
    /// Locale configuration
    #[prop(optional)]
    config: Option<LocaleConfig>,
    /// Child components
    children: Children,
) -> impl IntoView {
    let locale_config = RwSignal::new(config.unwrap_or_default());
    provide_context(locale_config);

    // 监听配置变化并应用 RTL
    Effect::new(move |_| {
        let config = locale_config.get();
        if config.rtl.enabled {
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                if let Some(element) = doc.document_element() {
                    let _ = element.set_attribute("dir", "rtl");
                }
            }
        }
    });

    view! {
        <div class="ant-locale">
            {children()}
        </div>
    }
}
