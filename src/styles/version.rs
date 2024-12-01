use crate::utils::style::create_style_sheet;

/// Applies styles for the version component
pub fn apply_version_style() {
    let style = r#"
        .ant-version {
            font-size: 14px;
            color: rgba(0, 0, 0, 0.45);
        }
    "#;

    create_style_sheet("version", style);
}
