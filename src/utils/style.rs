use crate::utils::dom;
use leptos::prelude::*;

/// Injects a CSS style string into the document
pub fn inject_style(css: &str) {
    if let Some(style_element) = dom::create_style_element(css) {
        if let Some(document) = dom::document() {
            if let Some(head) = document.head() {
                let _ = head.append_child(&style_element);
            }
        }
    }
}

/// Creates a style sheet with the given name and CSS content
pub fn create_style_sheet(_name: &str, css: impl Into<String> + 'static) {
    let css = css.into();
    Effect::new(move |_| {
        inject_style(&css);
    });
}
