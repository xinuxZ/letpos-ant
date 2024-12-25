/// Configuration provider styles
pub mod config_provider;
/// Locale styles
pub mod locale;
/// Global styles
pub mod style;
/// Theme styles
pub mod theme;
/// Version styles
pub mod version;

pub use config_provider::apply_styles;
pub use locale::apply_locale_styles as apply_locale;
pub use style::global_style;
pub use theme::apply_theme;
pub use version::apply_version_style;
