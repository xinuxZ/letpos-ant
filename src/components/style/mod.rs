//! 全局样式组件
//!
//! 提供全局样式的初始化和管理功能

use leptos::prelude::*;
use web_sys::window;

/// 全局样式组件
#[component]
pub fn Style(
    /// 子元素
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    view! {
        {children.map(|children| children())}
    }
}

/// 全局样式管理器
pub struct StyleManager;

impl StyleManager {
    /// 初始化全局样式
    pub fn init_global_style() {
        let window = window().expect("Failed to get window");
        let document = window.document().expect("Failed to get document");

        // 创建 style 元素
        let style = document
            .create_element("style")
            .expect("Failed to create style element");

        // 设置样式内容
        style.set_text_content(Some(include_str!("../../styles/global.css")));

        // 将 style 元素添加到 head 中
        document
            .head()
            .expect("Failed to get head element")
            .append_child(&style.as_ref())
            .expect("Failed to append style element");
    }
}
