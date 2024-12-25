use ant_leptos::components::{
    config_provider::ConfigProvider,
    locale::LocaleProvider,
    style::{Style, StyleManager},
    theme::Theme,
    version::VERSION,
};
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    // 初始化全局样式
    StyleManager::init_global_style();

    view! {
        // 配置提供器
        <ConfigProvider>
            // 国际化配置
            <LocaleProvider>
                // 主题配置
                <Theme>
                    <div class="demo-container">
                        <h1>"Ant Leptos Demo"</h1>

                        // 版本信息
                        <div>
                            <p>"Current Version: " {VERSION}</p>
                        </div>

                        // 主题演示
                        <div>
                            <h2>"Theme Demo"</h2>
                            <button class="ant-btn ant-btn-primary">"Primary Button"</button>
                        </div>

                        // 国际化演示
                        <div>
                            <h2>"Locale Demo"</h2>
                            <p>"Current language: English"</p>
                        </div>
                    </div>
                </Theme>
            </LocaleProvider>
        </ConfigProvider>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
