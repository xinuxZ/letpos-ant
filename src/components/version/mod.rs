use leptos::prelude::*;

/// Current version of the component library
pub const VERSION: &str = "0.0.1";

/// Version display component
#[component]
pub fn Version() -> impl IntoView {
    view! {
        <span class="ant-version">
            {"ant-leptos v"}{VERSION}
        </span>
    }
}
