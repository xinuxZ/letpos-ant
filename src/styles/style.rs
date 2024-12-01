use crate::utils::style::create_style_sheet;

/// Applies global styles for the component library
pub fn global_style() {
    let style = r#"
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto,
                'Helvetica Neue', Arial, 'Noto Sans', sans-serif;
            font-size: var(--ant-font-size-base);
            line-height: 1.5715;
            color: rgba(0, 0, 0, 0.85);
        }
    "#;

    create_style_sheet("global", style);
}
