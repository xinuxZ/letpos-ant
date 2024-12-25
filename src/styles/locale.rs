use crate::utils::style::create_style_sheet;

/// Applies locale-specific styles
pub fn apply_locale() {
    let style = r#"
        .ant-locale {
            display: contents;
        }

        [dir="rtl"] .ant-locale {
            direction: rtl;
        }

        .ant-language-switcher {
            display: inline-flex;
            gap: 8px;
        }

        .ant-language-switcher button {
            padding: 4px 8px;
            border: 1px solid #d9d9d9;
            border-radius: 2px;
            background: transparent;
            cursor: pointer;
        }

        .ant-language-switcher button:hover {
            color: var(--ant-primary-color);
            border-color: var(--ant-primary-color);
        }
    "#;

    create_style_sheet("locale", style);
}

pub use apply_locale as apply_locale_styles;
