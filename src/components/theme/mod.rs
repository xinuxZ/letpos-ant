//! 主题组件
//!
//! 提供主题配置和切换功能

use crate::components::config_provider::ThemeMode;
use leptos::prelude::*;
use web_sys::window;

/// 主题组件
#[component]
pub fn Theme(
    /// 主题模式
    #[prop(into)]
    theme_mode: ReadSignal<ThemeMode>,
    /// 子元素
    children: Children,
) -> impl IntoView {
    // 监听主题变化并更新 DOM
    create_effect(move |_| {
        let mode = theme_mode.get();
        let document = window().unwrap().document().unwrap();
        let root = document.document_element().unwrap();
        root.set_attribute(
            "theme-mode",
            match mode {
                ThemeMode::Light => "light",
                ThemeMode::Dark => "dark",
                ThemeMode::Compact => "compact",
            },
        )
        .unwrap();
    });

    view! {
        {children()}
    }
}
