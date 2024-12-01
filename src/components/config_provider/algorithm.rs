use super::theme::{ComponentToken, ThemeToken};
use std::collections::HashMap;

/// CSS variables generator
pub struct CSSVariablesGenerator {
    /// Theme token
    pub token: ThemeToken,
    /// Component tokens
    pub components: HashMap<String, ComponentToken>,
    /// CSS selector
    pub selector: String,
    /// Prefix for variables
    pub prefix: String,
}

impl CSSVariablesGenerator {
    /// Create new generator
    pub fn new(token: ThemeToken) -> Self {
        Self {
            token,
            components: HashMap::new(),
            selector: ":root".to_string(),
            prefix: "ant".to_string(),
        }
    }

    /// Set CSS selector
    pub fn with_selector(mut self, selector: impl Into<String>) -> Self {
        self.selector = selector.into();
        self
    }

    /// Set variable prefix
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }

    /// Add component tokens
    pub fn with_components(mut self, components: HashMap<String, ComponentToken>) -> Self {
        self.components = components;
        self
    }

    /// Generate CSS variables
    pub fn generate(&self) -> String {
        let mut css = String::new();

        // 生成全局变量
        css.push_str(&self.generate_global_vars());

        // 生成组件变量
        css.push_str(&self.generate_component_vars());

        // 生成暗色主题变量
        css.push_str(&self.generate_dark_theme_vars());

        // 生成紧凑主题变量
        css.push_str(&self.generate_compact_theme_vars());

        css
    }

    fn generate_global_vars(&self) -> String {
        format!(
            r#"
            {} {{
                --{}-primary-color: {};
                --{}-success-color: {};
                --{}-warning-color: {};
                --{}-error-color: {};
                --{}-font-size-base: {};
                --{}-border-radius-base: {};
                --{}-line-height: {};
                --{}-background: {};
                --{}-text-color: {};
                --{}-border-color: {};
                --{}-box-shadow: {};
                --{}-disabled-opacity: {};
                --{}-disabled-bg: {};
                --{}-disabled-color: {};

                /* 派生变量 */
                --{}-primary-color-hover: {};
                --{}-primary-color-active: {};
                --{}-primary-color-outline: {};
                --{}-primary-1: {};
                --{}-primary-2: {};
                --{}-primary-3: {};
                --{}-primary-4: {};
                --{}-primary-5: {};
                --{}-primary-6: {};
                --{}-primary-7: {};
                --{}-primary-8: {};
                --{}-primary-9: {};
                --{}-primary-10: {};
            }}
            "#,
            self.selector,
            self.prefix,
            self.token.primary_color,
            self.prefix,
            self.token.success_color,
            self.prefix,
            self.token.warning_color,
            self.prefix,
            self.token.error_color,
            self.prefix,
            self.token.font_size_base,
            self.prefix,
            self.token.border_radius_base,
            self.prefix,
            self.token.line_height,
            self.prefix,
            self.token.background,
            self.prefix,
            self.token.text_color,
            self.prefix,
            self.token.border_color,
            self.prefix,
            self.token.box_shadow,
            self.prefix,
            self.token.disabled_opacity,
            self.prefix,
            self.token.disabled_bg,
            self.prefix,
            self.token.disabled_color,
            // 派生变量
            self.prefix,
            self.generate_hover_color(&self.token.primary_color),
            self.prefix,
            self.generate_active_color(&self.token.primary_color),
            self.prefix,
            self.generate_outline_color(&self.token.primary_color),
            // 色板变量
            self.prefix,
            self.generate_palette_color(&self.token.primary_color, 1),
            self.prefix,
            self.generate_palette_color(&self.token.primary_color, 2),
            self.prefix,
            self.generate_palette_color(&self.token.primary_color, 3),
            self.prefix,
            self.generate_palette_color(&self.token.primary_color, 4),
            self.prefix,
            self.generate_palette_color(&self.token.primary_color, 5),
            self.prefix,
            self.generate_palette_color(&self.token.primary_color, 6),
            self.prefix,
            self.generate_palette_color(&self.token.primary_color, 7),
            self.prefix,
            self.generate_palette_color(&self.token.primary_color, 8),
            self.prefix,
            self.generate_palette_color(&self.token.primary_color, 9),
            self.prefix,
            self.generate_palette_color(&self.token.primary_color, 10),
        )
    }

    fn generate_component_vars(&self) -> String {
        let mut css = String::new();

        for (component, token) in &self.components {
            css.push_str(&format!(
                r#"
                /* {} Component Variables */
                {} {{
                "#,
                component, self.selector
            ));

            for (key, value) in &token.token {
                css.push_str(&format!(
                    "    --{}-{}-{}: {};\n",
                    self.prefix, component, key, value
                ));
            }

            css.push_str("}\n");
        }

        css
    }

    fn generate_dark_theme_vars(&self) -> String {
        format!(
            r#"
            [data-theme='dark'] {{
                --{}-body-background: #141414;
                --{}-component-background: #1f1f1f;
                --{}-text-color: rgba(255, 255, 255, 0.85);
                --{}-text-color-secondary: rgba(255, 255, 255, 0.45);
                --{}-border-color-base: #434343;
                --{}-border-color-split: #303030;
                --{}-background-color-light: rgba(255, 255, 255, 0.04);
                --{}-background-color-base: rgba(255, 255, 255, 0.08);
            }}
            "#,
            self.prefix,
            self.prefix,
            self.prefix,
            self.prefix,
            self.prefix,
            self.prefix,
            self.prefix,
            self.prefix
        )
    }

    fn generate_compact_theme_vars(&self) -> String {
        format!(
            r#"
            [data-theme='compact'] {{
                --{}-font-size-base: 12px;
                --{}-font-size-lg: 14px;
                --{}-font-size-sm: 12px;
                --{}-padding-lg: 16px;
                --{}-padding-md: 12px;
                --{}-padding-sm: 8px;
                --{}-padding-xs: 4px;
                --{}-border-radius-base: 2px;
            }}
            "#,
            self.prefix,
            self.prefix,
            self.prefix,
            self.prefix,
            self.prefix,
            self.prefix,
            self.prefix,
            self.prefix
        )
    }

    // 颜色处理辅助函数
    fn generate_hover_color(&self, color: &str) -> String {
        // 实现颜色变亮逻辑
        format!("color-mix(in srgb, {}, white 15%)", color)
    }

    fn generate_active_color(&self, color: &str) -> String {
        // 实现颜色变暗逻辑
        format!("color-mix(in srgb, {}, black 15%)", color)
    }

    fn generate_outline_color(&self, color: &str) -> String {
        // 实现轮廓颜色逻辑
        format!("color-mix(in srgb, {}, transparent 80%)", color)
    }

    fn generate_palette_color(&self, color: &str, index: u8) -> String {
        // 实现色板颜色生成逻辑
        match index {
            1 => format!("color-mix(in srgb, {}, white 90%)", color),
            2 => format!("color-mix(in srgb, {}, white 80%)", color),
            3 => format!("color-mix(in srgb, {}, white 70%)", color),
            4 => format!("color-mix(in srgb, {}, white 60%)", color),
            5 => format!("color-mix(in srgb, {}, white 40%)", color),
            6 => color.to_string(),
            7 => format!("color-mix(in srgb, {}, black 20%)", color),
            8 => format!("color-mix(in srgb, {}, black 40%)", color),
            9 => format!("color-mix(in srgb, {}, black 60%)", color),
            10 => format!("color-mix(in srgb, {}, black 80%)", color),
            _ => color.to_string(),
        }
    }
}
