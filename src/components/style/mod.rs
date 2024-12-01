use crate::utils::dom;
use leptos::prelude::*;

/// Global style configuration
#[derive(Clone, Debug)]
pub struct GlobalStyle {
    /// CSS content to be injected
    pub css: String,
}

/// Style component for injecting CSS into the document
#[component]
pub fn Style(
    /// CSS content to inject
    css: String,
) -> impl IntoView {
    Effect::new(move |_| {
        if let Some(style_element) = dom::create_style_element(&css) {
            if let Some(document) = dom::document() {
                if let Some(head) = document.head() {
                    let _ = head.append_child(&style_element);
                }
            }
        }
    });

    view! { <></> }
}
