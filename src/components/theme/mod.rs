use crate::components::config_provider::theme::{ComponentToken, ThemeAlgorithm, ThemeToken};
use leptos::prelude::*;
use std::collections::HashMap;

/// Theme configuration options
#[derive(Clone, Debug, PartialEq)]
pub struct ThemeConfig {
    /// Theme token containing basic design tokens
    pub token: ThemeToken,
    /// Theme algorithm for dynamic theme generation
    pub algorithm: Option<Vec<ThemeAlgorithm>>,
    /// Component tokens for component-specific customization
    pub components: HashMap<String, ComponentToken>,
    /// Hash id for CSS-in-JS solutions
    pub hash_id: Option<String>,
    /// CSS variables prefix
    pub css_vars_prefix: Option<String>,
    /// CSS variables inline strategy
    pub css_vars_inline: bool,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            token: ThemeToken::default(),
            algorithm: None,
            components: HashMap::new(),
            hash_id: None,
            css_vars_prefix: None,
            css_vars_inline: false,
        }
    }
}

/// Theme context value
#[derive(Clone, Debug, PartialEq)]
pub struct ThemeContextValue {
    /// Theme configuration
    pub theme: ThemeConfig,
    /// Theme hash id
    pub hash_id: Option<String>,
    /// CSS variables prefix
    pub css_vars_prefix: Option<String>,
    /// CSS variables inline strategy
    pub css_vars_inline: bool,
}

/// Theme provider component for customizing the visual appearance
#[component]
pub fn Theme(
    /// Optional theme configuration
    #[prop(optional)]
    config: Option<ThemeConfig>,
    /// Whether to inherit parent theme
    #[prop(optional)]
    inherit: Option<bool>,
    /// Child components
    children: Children,
) -> impl IntoView {
    let parent_theme = use_context::<RwSignal<ThemeContextValue>>();
    let theme_config = config.unwrap_or_default();
    let theme_config_clone = theme_config.clone(); // 克隆用于 memo

    // 合并父主题和当前主题
    let merged_theme = Memo::new(move |_| {
        if inherit.unwrap_or(true) {
            if let Some(parent) = parent_theme {
                let parent_value = parent.get();
                ThemeContextValue {
                    theme: ThemeConfig {
                        token: theme_config_clone.token.clone(),
                        algorithm: theme_config_clone
                            .algorithm
                            .clone()
                            .or(parent_value.theme.algorithm),
                        components: {
                            let mut components = parent_value.theme.components.clone();
                            components.extend(theme_config_clone.components.clone());
                            components
                        },
                        hash_id: theme_config_clone
                            .hash_id
                            .clone()
                            .or(parent_value.theme.hash_id),
                        css_vars_prefix: theme_config_clone
                            .css_vars_prefix
                            .clone()
                            .or(parent_value.theme.css_vars_prefix),
                        css_vars_inline: theme_config_clone.css_vars_inline
                            || parent_value.theme.css_vars_inline,
                    },
                    hash_id: theme_config_clone
                        .clone()
                        .hash_id
                        .clone()
                        .or(parent_value.hash_id),
                    css_vars_prefix: theme_config_clone
                        .css_vars_prefix
                        .clone()
                        .or(parent_value.css_vars_prefix),
                    css_vars_inline: theme_config_clone.css_vars_inline
                        || parent_value.css_vars_inline,
                }
            } else {
                ThemeContextValue {
                    theme: theme_config_clone.clone(),
                    hash_id: theme_config_clone.hash_id.clone(),
                    css_vars_prefix: theme_config_clone.css_vars_prefix.clone(),
                    css_vars_inline: theme_config_clone.css_vars_inline,
                }
            }
        } else {
            ThemeContextValue {
                theme: theme_config_clone.clone(),
                hash_id: theme_config_clone.hash_id.clone(),
                css_vars_prefix: theme_config_clone.css_vars_prefix.clone(),
                css_vars_inline: theme_config_clone.css_vars_inline,
            }
        }
    });

    let theme_context = RwSignal::new(ThemeContextValue {
        theme: theme_config.clone(),
        hash_id: theme_config.hash_id.clone(),
        css_vars_prefix: theme_config.css_vars_prefix.clone(),
        css_vars_inline: theme_config.css_vars_inline,
    });

    provide_context(theme_context);

    // 生成并注入 CSS 变量
    Effect::new(move |_| {
        let current_theme = merged_theme.get();
        if current_theme.css_vars_inline {
            let css_generator = {
                let mut generator = crate::components::config_provider::CSSVariablesGenerator::new(
                    current_theme.theme.token.clone(),
                );

                generator = generator.with_components(current_theme.theme.components.clone());

                if let Some(prefix) = &current_theme.css_vars_prefix {
                    generator = generator.with_prefix(prefix.clone());
                }

                generator
            };

            let css = css_generator.generate();
            if let Some(style_element) = crate::utils::dom::create_style_element(&css) {
                if let Some(document) = crate::utils::dom::document() {
                    if let Some(head) = document.head() {
                        let _ = head.append_child(&style_element);
                    }
                }
            }
        }
    });

    view! {
        <div class="ant-theme">
            {children()}
        </div>
    }
}

/// Hook to access the current theme context
pub fn use_theme() -> Option<ReadSignal<ThemeContextValue>> {
    use_context::<RwSignal<ThemeContextValue>>().map(|ctx| ctx.read_only())
}
