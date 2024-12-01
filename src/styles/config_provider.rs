use crate::utils::style::create_style_sheet;

/// Applies the configuration provider styles
pub fn apply_styles() {
    let style = r#"
        .ant-config-provider {
            width: 100%;
            height: 100%;
            display: contents;
        }

        .ant-config-provider-disabled * {
            cursor: not-allowed !important;
            user-select: none !important;
            pointer-events: none !important;
        }

        .ant-config-provider-sm {
            font-size: 12px;
        }

        .ant-config-provider-lg {
            font-size: 16px;
        }

        [data-theme='dark'] {
            --ant-primary-color: #177ddc;
            --ant-success-color: #49aa19;
            --ant-warning-color: #d89614;
            --ant-error-color: #a61d24;
            color-scheme: dark;
        }

        [data-theme='compact'] {
            --ant-font-size-base: 12px;
            --ant-line-height-base: 1.66667;
        }
    "#;
    create_style_sheet("config-provider", style);
}
