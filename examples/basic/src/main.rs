use ant_leptos::components::{
    config_provider::{ConfigProvider, ThemeMode},
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

    // 创建主题状态
    let (theme_mode, set_theme_mode) = create_signal(ThemeMode::Light);

    // 切换主题的处理函数
    let toggle_theme = move |_| {
        set_theme_mode.update(|mode| {
            *mode = match mode {
                ThemeMode::Light => ThemeMode::Dark,
                ThemeMode::Dark => ThemeMode::Light,
                ThemeMode::Compact => ThemeMode::Light,
            }
        });
    };

    view! {
        // 配置提供器
        <ConfigProvider>
            // 国际化配置
            <LocaleProvider>
                // 主题配置
                <Theme theme_mode=theme_mode>
                    <div class="demo-container">
                        <h1>"Ant Leptos Demo"</h1>

                        // 版本信息
                        <div>
                            <p>"Current Version: " {VERSION}</p>
                        </div>

                        // 主题演示
                        <div>
                            <h2>"Theme Demo"</h2>
                            <div class="button-group">
                                <button class="ant-btn ant-btn-primary">"Primary Button"</button>
                                <button
                                    class="ant-btn"
                                    on:click=toggle_theme
                                >
                                    "Switch to " {move || match theme_mode.get() {
                                        ThemeMode::Light => "Dark",
                                        ThemeMode::Dark => "Light",
                                        ThemeMode::Compact => "Compact",
                                    }} " Theme"
                                </button>
                            </div>
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
