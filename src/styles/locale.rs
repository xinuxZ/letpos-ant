use crate::utils::style::create_style_sheet;

/// Applies the locale-specific styles
pub fn apply_locale() {
    let style = r#"
        .ant-locale {
            display: contents;
        }
    "#;
    create_style_sheet("locale", style);
}
