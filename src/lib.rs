//! Ant Design components for Leptos framework
#![deny(missing_docs)]

/// Components module containing all Ant Design components
pub mod components;
/// Styles module containing component styles
pub mod styles;
/// Utility functions and helpers
pub mod utils;

// Re-export commonly used components
pub use components::{
    config_provider::ConfigProvider,
    locale::LocaleProvider,
    style::Style,
    theme::Theme, // 现在这个导入应该是正确的
    version::Version,
};

/// Current version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
