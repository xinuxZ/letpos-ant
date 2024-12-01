use leptos::*;
use web_sys::{Document, Element, Window};

/// Returns the global window object
pub fn window() -> Option<Window> {
    web_sys::window()
}

/// Returns the document object
pub fn document() -> Option<Document> {
    window().and_then(|win| win.document())
}

/// Creates a style element with the given CSS content
pub fn create_style_element(css: &str) -> Option<Element> {
    document().and_then(|doc| {
        let element = doc.create_element("style").ok()?;
        element.set_text_content(Some(css));
        Some(element)
    })
}
