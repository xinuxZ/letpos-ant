use crate::components::config_provider::{ComponentSize, Config, Theme};
use leptos::prelude::*;
// use std::collections::HashMap;

/// Default prefix for all components
pub const ANT_PREFIX: &str = "ant";

/// Configuration context type
pub type ConfigContext = RwSignal<Config>;

/// Theme context type
pub type ThemeContext = RwSignal<Theme>;

/// Size context type
pub type SizeContext = RwSignal<Option<ComponentSize>>;

/// Get global prefix
pub fn get_prefix_cls() -> Option<String> {
    use_context::<ConfigContext>().map(|ctx| ctx.get_untracked().prefix_cls)
}

/// Get component prefix
pub fn get_component_cls(suffix: &str) -> String {
    get_prefix_cls()
        .map(|prefix| format!("{}-{}", prefix, suffix))
        .unwrap_or_else(|| format!("{}-{}", ANT_PREFIX, suffix))
}

/// Get current theme
pub fn use_theme() -> Option<ReadSignal<Theme>> {
    use_context::<ThemeContext>().map(|ctx| ctx.read_only())
}

/// Get current size
pub fn use_size() -> Option<ReadSignal<Option<ComponentSize>>> {
    use_context::<SizeContext>().map(|ctx| ctx.read_only())
}
