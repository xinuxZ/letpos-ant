//! 本地化上下文组件

use crate::components::locale::types::{Language, LocaleConfig, LocaleContext};
use leptos::prelude::*;

/// 本地化提供器组件
#[component]
pub fn LocaleProvider(
    /// 当前语言
    #[prop(into)]
    current_language: Signal<Language>,
    /// 子元素
    children: Children,
) -> impl IntoView {
    // 创建本地化配置信号
    let locale_config = create_rw_signal(LocaleConfig::default());

    // 监听语言变化并更新配置
    create_effect(move |_| {
        locale_config.update(|config| {
            config.language = current_language.get();
        });
    });

    // 提供上下文
    provide_context(locale_config);

    view! {
        {children()}
    }
}
