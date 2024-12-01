use crate::utils::style::create_style_sheet;

/// Applies the theme styles with CSS variables
pub fn apply_theme() {
    let style = r#"
        :root {
            --ant-primary-color: #1890ff;
            --ant-success-color: #52c41a;
            --ant-warning-color: #faad14;
            --ant-error-color: #f5222d;
            --ant-font-size-base: 14px;
            --ant-border-radius-base: 4px;
        }
    "#;

    create_style_sheet("theme", style);
}
