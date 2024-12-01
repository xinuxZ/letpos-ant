use crate::components::config_provider::theme::{ComponentToken, ThemeAlgorithm, ThemeToken};
use leptos::prelude::*;
use std::collections::HashMap;

/// Theme configuration options
#[derive(Clone, Debug)]
pub struct ThemeConfig {
    /// Theme token
    pub token: ThemeToken,
    /// Theme algorithm
    pub algorithm: Option<Vec<ThemeAlgorithm>>,
    /// Component tokens
    pub components: HashMap<String, ComponentToken>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        let token = ThemeToken::default();
        Self {
            token,
            algorithm: None,
            components: HashMap::new(),
        }
    }
}

/// Theme provider component for customizing the visual appearance
#[component]
pub fn Theme(
    /// Optional theme configuration
    #[prop(optional)]
    config: Option<ThemeConfig>,
    /// Child components
    children: Children,
) -> impl IntoView {
    let theme = RwSignal::new(config.unwrap_or_default());

    provide_context(theme);

    view! {
        <div class="ant-theme">
            {children()}
        </div>
    }
}
