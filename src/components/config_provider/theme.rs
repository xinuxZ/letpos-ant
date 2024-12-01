use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Theme token configuration
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ThemeToken {
    /// Primary color
    pub primary_color: String,
    /// Success color
    pub success_color: String,
    /// Warning color
    pub warning_color: String,
    /// Error color
    pub error_color: String,
    /// Font size base
    pub font_size_base: String,
    /// Border radius base
    pub border_radius_base: String,
    /// Line height
    pub line_height: f32,
    /// Background color
    pub background: String,
    /// Text color
    pub text_color: String,
    /// Border color
    pub border_color: String,
    /// Box shadow
    pub box_shadow: String,
    /// Disabled opacity
    pub disabled_opacity: f32,
    /// Disabled background
    pub disabled_bg: String,
    /// Disabled color
    pub disabled_color: String,
}

/// Theme algorithm configuration
#[derive(Clone, Debug)]
pub struct ThemeAlgorithmConfig {
    /// Theme algorithm
    pub algorithm: Vec<ThemeAlgorithm>,
    /// Theme token
    pub token: ThemeToken,
    /// Component tokens
    pub components: HashMap<String, ComponentToken>,
}

/// Theme algorithm types
#[derive(Clone, Debug, PartialEq)]
pub enum ThemeAlgorithm {
    /// Default theme
    Default,
    /// Dark theme
    Dark,
    /// Compact theme
    Compact,
}

/// Component token configuration
#[derive(Clone, Debug, PartialEq)]
pub struct ComponentToken {
    /// Token values
    pub token: HashMap<String, String>,
}

impl Default for ThemeToken {
    fn default() -> Self {
        Self {
            primary_color: "#1890ff".to_string(),
            success_color: "#52c41a".to_string(),
            warning_color: "#faad14".to_string(),
            error_color: "#f5222d".to_string(),
            font_size_base: "14px".to_string(),
            border_radius_base: "4px".to_string(),
            line_height: 1.5715,
            background: "#ffffff".to_string(),
            text_color: "rgba(0, 0, 0, 0.85)".to_string(),
            border_color: "#d9d9d9".to_string(),
            box_shadow: "0 2px 8px rgba(0, 0, 0, 0.15)".to_string(),
            disabled_opacity: 0.6,
            disabled_bg: "#f5f5f5".to_string(),
            disabled_color: "rgba(0, 0, 0, 0.25)".to_string(),
        }
    }
}

impl ThemeToken {
    /// Apply theme algorithm
    pub fn apply_algorithm(&mut self, algorithm: &ThemeAlgorithm) {
        match algorithm {
            ThemeAlgorithm::Dark => self.apply_dark_theme(),
            ThemeAlgorithm::Compact => self.apply_compact_theme(),
            ThemeAlgorithm::Default => {}
        }
    }

    fn apply_dark_theme(&mut self) {
        self.primary_color = "#177ddc".to_string();
        self.background = "#141414".to_string();
        self.text_color = "rgba(255, 255, 255, 0.85)".to_string();
        self.border_color = "#434343".to_string();
        self.success_color = "#49aa19".to_string();
        self.warning_color = "#d89614".to_string();
        self.error_color = "#a61d24".to_string();
        self.box_shadow = "0 2px 8px rgba(0, 0, 0, 0.45)".to_string();
        self.disabled_bg = "#262626".to_string();
        self.disabled_color = "rgba(255, 255, 255, 0.25)".to_string();
    }

    fn apply_compact_theme(&mut self) {
        self.font_size_base = "12px".to_string();
        self.border_radius_base = "2px".to_string();
        self.line_height = 1.66667;
    }
}

/// 保留现有的 ThemeConfig 结构体，但添加新的字段
/// Theme configuration for customizing the visual appearance of components
#[derive(Clone, Debug)]
pub struct ThemeConfig {
    /// Theme token
    pub token: ThemeToken,
    /// Theme algorithm
    pub algorithm: Option<Vec<ThemeAlgorithm>>,
    /// Component tokens
    pub components: HashMap<String, ComponentToken>,
    /// Primary color used throughout the interface
    pub primary_color: String,
    /// Color used for informational elements
    pub info_color: String,
    /// Color used for successful operations
    pub success_color: String,
    /// Color used for warning states
    pub warning_color: String,
    /// Color used for error states
    pub error_color: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        let token = ThemeToken::default();
        Self {
            primary_color: token.primary_color.clone(),
            info_color: token.primary_color.clone(),
            success_color: token.success_color.clone(),
            warning_color: token.warning_color.clone(),
            error_color: token.error_color.clone(),
            token,
            algorithm: None,
            components: HashMap::new(),
        }
    }
}
