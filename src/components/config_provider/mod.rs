mod algorithm;
/// Context management for configuration
mod context;
/// Hooks for accessing configuration
mod hooks;
/// Theme system implementation and customization
pub mod theme;

pub use algorithm::*;
pub use context::*;
pub use hooks::*;
pub use theme::*;

use crate::components::theme::ThemeConfig;
use leptos::prelude::*;
use std::sync::Arc;
use web_sys::Element;

/// Configuration options for the ConfigProvider component
#[derive(Clone, Debug)]
pub struct Config {
    /// CSS class prefix for all components
    pub prefix_cls: String,
    /// Text direction for the application
    pub direction: Direction,
    /// Default size for components
    pub size: ComponentSize,
    /// Theme configuration
    pub theme: Theme,
    /// Whether to show warnings in console
    pub warning_config: WarningConfig,
    /// Icon configuration
    pub icon_prefix_cls: String,
    /// Whether components use CSS variables
    pub use_css_vars: bool,
    /// Form validation messages
    pub form: FormConfig,
    /// Input component configuration
    pub input: InputConfig,
    /// Select component configuration
    pub select: SelectConfig,
    /// Pagination configuration
    pub pagination: PaginationConfig,
    /// Modal dialog configuration
    pub modal: ModalConfig,
    /// Space configuration
    pub space: SpaceConfig,
    /// Virtual scroll configuration
    pub r#virtual: VirtualConfig,
    /// Get container for popup elements
    pub popup_container: Option<PopupContainer>,
    /// Auto insert CSS into document
    pub auto_insert_space_in_button: bool,
    /// Component token config
    pub component_token: ComponentToken,
    /// Component size
    pub component_size: Option<ComponentSize>,
    /// Component disabled state
    pub component_disabled: bool,
}

/// Text direction options
#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    /// Left to right
    Ltr,
    /// Right to left
    Rtl,
}

/// Component size options
#[derive(Clone, Debug, PartialEq)]
pub enum ComponentSize {
    /// Small size
    Small,
    /// Medium size
    Middle,
    /// Large size
    Large,
}

/// Theme configuration
#[derive(Clone, Debug)]
pub struct Theme {
    /// Primary color
    pub primary_color: String,
    /// Info color
    pub info_color: String,
    /// Success color
    pub success_color: String,
    /// Warning color
    pub warning_color: String,
    /// Error color
    pub error_color: String,
}

/// Warning configuration
#[derive(Clone, Debug)]
pub struct WarningConfig {
    /// Whether to show warnings
    pub show_warnings: bool,
}

/// Form configuration
#[derive(Clone, Debug)]
pub struct FormConfig {
    /// Validation messages
    pub validate_messages: ValidateMessages,
    /// Whether to scroll to first error field
    pub scroll_to_first_error: bool,
    /// Whether to require fields have a value
    pub require_mark: bool,
}

/// Input component configuration
#[derive(Clone, Debug)]
pub struct InputConfig {
    /// Whether to automatically focus the first field
    pub auto_focus: bool,
    /// Custom border color
    pub border_color: Option<String>,
}

/// Select component configuration
#[derive(Clone, Debug)]
pub struct SelectConfig {
    /// Empty content
    pub empty_text: String,
    /// Loading text
    pub loading_text: String,
}

/// Pagination configuration
#[derive(Clone, Debug)]
pub struct PaginationConfig {
    /// Show size changer
    pub show_size_changer: bool,
    /// Show quick jumper
    pub show_quick_jumper: bool,
}

/// Modal dialog configuration
#[derive(Clone, Debug)]
pub struct ModalConfig {
    /// Default close button text
    pub close_text: String,
    /// Default OK button text
    pub ok_text: String,
}

/// Space configuration
#[derive(Clone, Debug)]
pub struct SpaceConfig {
    /// Default size
    pub size: SpaceSize,
}

/// Virtual scroll configuration
#[derive(Clone, Debug)]
pub struct VirtualConfig {
    /// Height of each row
    pub item_height: f64,
}

/// Space size options
#[derive(Clone, Debug, PartialEq)]
pub enum SpaceSize {
    /// Small spacing
    Small,
    /// Medium spacing
    Middle,
    /// Large spacing
    Large,
    /// Custom spacing
    Custom(f64),
}

/// Form validation messages
#[derive(Clone, Debug)]
pub struct ValidateMessages {
    /// Required field message
    pub required: String,
    /// Types validation messages
    pub types: ValidateTypeMessages,
    /// Length validation messages
    pub length: ValidateLengthMessages,
}

/// Type validation messages
#[derive(Clone, Debug)]
pub struct ValidateTypeMessages {
    /// String type message
    pub string: String,
    /// Number type message
    pub number: String,
    /// Array type message
    pub array: String,
}

/// Length validation messages
#[derive(Clone, Debug)]
pub struct ValidateLengthMessages {
    /// Min length message
    pub min: String,
    /// Max length message
    pub max: String,
    /// Range length message
    pub range: String,
}

/// Popup container configuration
#[derive(Clone)]
pub struct PopupContainer {
    /// Function to get container element
    pub get_popup_container: Option<Arc<dyn Fn() -> Option<Element> + Send + Sync + 'static>>,
}

impl std::fmt::Debug for PopupContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PopupContainer")
            .field("get_popup_container", &self.get_popup_container.is_some())
            .finish()
    }
}

/// Component token configuration
#[derive(Clone, Debug)]
pub struct ComponentToken {
    /// Motion duration
    pub motion_duration_mid: u32,
    /// Motion duration for fast animations
    pub motion_duration_fast: u32,
    /// Motion ease in out
    pub motion_ease_in_out: String,
    /// Motion ease out
    pub motion_ease_out: String,
    /// Motion base
    pub motion_base: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prefix_cls: "ant".to_string(),
            direction: Direction::Ltr,
            size: ComponentSize::Middle,
            theme: Theme::default(),
            warning_config: WarningConfig::default(),
            icon_prefix_cls: "anticon".to_string(),
            use_css_vars: true,
            form: FormConfig::default(),
            input: InputConfig::default(),
            select: SelectConfig::default(),
            pagination: PaginationConfig::default(),
            modal: ModalConfig::default(),
            space: SpaceConfig::default(),
            r#virtual: VirtualConfig::default(),
            popup_container: None,
            auto_insert_space_in_button: true,
            component_token: ComponentToken::default(),
            component_size: None,
            component_disabled: false,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary_color: "#1890ff".to_string(),
            info_color: "#1890ff".to_string(),
            success_color: "#52c41a".to_string(),
            warning_color: "#faad14".to_string(),
            error_color: "#ff4d4f".to_string(),
        }
    }
}

impl Default for WarningConfig {
    fn default() -> Self {
        Self {
            show_warnings: true,
        }
    }
}

impl Default for FormConfig {
    fn default() -> Self {
        Self {
            validate_messages: ValidateMessages::default(),
            scroll_to_first_error: true,
            require_mark: true,
        }
    }
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            auto_focus: false,
            border_color: None,
        }
    }
}

impl Default for SelectConfig {
    fn default() -> Self {
        Self {
            empty_text: "No data".to_string(),
            loading_text: "Loading...".to_string(),
        }
    }
}

impl Default for PaginationConfig {
    fn default() -> Self {
        Self {
            show_size_changer: true,
            show_quick_jumper: false,
        }
    }
}

impl Default for ModalConfig {
    fn default() -> Self {
        Self {
            close_text: "Cancel".to_string(),
            ok_text: "OK".to_string(),
        }
    }
}

impl Default for SpaceConfig {
    fn default() -> Self {
        Self {
            size: SpaceSize::Small,
        }
    }
}

impl Default for VirtualConfig {
    fn default() -> Self {
        Self { item_height: 24.0 }
    }
}

impl Default for ValidateMessages {
    fn default() -> Self {
        Self {
            required: "${label} is required".to_string(),
            types: ValidateTypeMessages::default(),
            length: ValidateLengthMessages::default(),
        }
    }
}

impl Default for ValidateTypeMessages {
    fn default() -> Self {
        Self {
            string: "${label} is not a valid string".to_string(),
            number: "${label} is not a valid number".to_string(),
            array: "${label} is not a valid array".to_string(),
        }
    }
}

impl Default for ValidateLengthMessages {
    fn default() -> Self {
        Self {
            min: "${label} must be at least ${min} characters".to_string(),
            max: "${label} cannot be longer than ${max} characters".to_string(),
            range: "${label} must be between ${min} and ${max} characters".to_string(),
        }
    }
}

impl Default for PopupContainer {
    fn default() -> Self {
        Self {
            get_popup_container: None,
        }
    }
}

impl Default for ComponentToken {
    fn default() -> Self {
        Self {
            motion_duration_mid: 200,
            motion_duration_fast: 100,
            motion_ease_in_out: "cubic-bezier(0.645, 0.045, 0.355, 1)".to_string(),
            motion_ease_out: "cubic-bezier(0.215, 0.61, 0.355, 1)".to_string(),
            motion_base: 100,
        }
    }
}

impl Config {
    /// Set theme mode
    pub fn set_theme_mode(&mut self, mode: ThemeMode) {
        match mode {
            ThemeMode::Dark => {
                self.theme.primary_color = "#177ddc".to_string();
                self.theme.success_color = "#49aa19".to_string();
                self.theme.warning_color = "#d89614".to_string();
                self.theme.error_color = "#a61d24".to_string();
            }
            ThemeMode::Light => {
                self.theme = Theme::default();
            }
            ThemeMode::Compact => {
                // 保持颜色不变，只改变尺寸相关的配置
                self.size = ComponentSize::Small;
            }
        }
    }

    /// Get popup container element
    pub fn get_popup_container(&self) -> Option<Element> {
        self.popup_container
            .as_ref()
            .and_then(|pc| pc.get_popup_container.as_ref())
            .and_then(|f| f())
    }
}

/// Theme mode options
#[derive(Clone, Debug, PartialEq)]
pub enum ThemeMode {
    /// Dark theme
    Dark,
    /// Light theme
    Light,
    /// Compact theme
    Compact,
}

/// Global configuration provider component
#[component]
pub fn ConfigProvider(
    /// Optional configuration
    #[prop(optional)]
    config: Option<Config>,
    /// Theme configuration
    #[prop(optional)]
    theme: Option<ThemeConfig>,
    /// Component size
    #[prop(optional)]
    size: Option<ComponentSize>,
    /// Direction
    #[prop(optional)]
    direction: Option<Direction>,
    /// Whether components are disabled
    #[prop(optional)]
    disabled: Option<bool>,
    /// Child components
    children: Children,
) -> impl IntoView {
    let mut base_config = config.unwrap_or_default();
    let theme_clone = theme.clone(); // 克隆 theme 以便后续使用

    // 应用主题配置
    if let Some(theme_config) = theme {
        let mut token = theme_config.token.clone();

        // 应用主题算法
        if let Some(algorithms) = &theme_config.algorithm {
            for algorithm in algorithms {
                token.apply_algorithm(algorithm);
            }
        }

        let css_generator = CSSVariablesGenerator::new(token.clone())
            .with_components(theme_config.components.clone());

        // 注入 CSS 变量
        if base_config.use_css_vars {
            let css = css_generator.generate();
            if let Some(style_element) = crate::utils::dom::create_style_element(&css) {
                if let Some(document) = crate::utils::dom::document() {
                    if let Some(head) = document.head() {
                        let _ = head.append_child(&style_element);
                    }
                }
            }
        }

        // 更新配置中的主题
        base_config.theme = Theme {
            primary_color: token.primary_color.clone(),
            info_color: token.primary_color.clone(),
            success_color: token.success_color,
            warning_color: token.warning_color,
            error_color: token.error_color,
        };
    }

    // Apply props to config
    if let Some(ref size) = size {
        base_config.component_size = Some(size.clone());
    }
    if let Some(dir) = direction {
        base_config.direction = dir;
    }
    if let Some(disabled) = disabled {
        base_config.component_disabled = disabled;
    }

    let config = RwSignal::new(base_config);

    // Generate theme from config
    let theme_ctx = RwSignal::new(
        theme_clone
            .map(|t| Theme {
                primary_color: t.token.primary_color.clone(),
                info_color: t.token.primary_color.clone(),
                success_color: t.token.success_color.clone(),
                warning_color: t.token.warning_color.clone(),
                error_color: t.token.error_color.clone(),
            })
            .unwrap_or_default(),
    );

    let size_ctx = RwSignal::new(size);

    provide_context(config);
    provide_context(theme_ctx);
    provide_context(size_ctx);

    // Apply CSS variables if enabled
    Effect::new(move |_| {
        let current_config = config.get_untracked();
        if current_config.use_css_vars {
            let theme = current_config.theme;
            let token = current_config.component_token;
            let style = format!(
                r#"
                :root {{
                    --ant-primary-color: {};
                    --ant-info-color: {};
                    --ant-success-color: {};
                    --ant-warning-color: {};
                    --ant-error-color: {};
                    --ant-motion-duration-mid: {}ms;
                    --ant-motion-duration-fast: {}ms;
                    --ant-motion-ease-in-out: {};
                    --ant-motion-ease-out: {};
                    --ant-motion-base: {}ms;
                }}
            "#,
                theme.primary_color,
                theme.info_color,
                theme.success_color,
                theme.warning_color,
                theme.error_color,
                token.motion_duration_mid,
                token.motion_duration_fast,
                token.motion_ease_in_out,
                token.motion_ease_out,
                token.motion_base,
            );

            if let Some(style_element) = crate::utils::dom::create_style_element(&style) {
                if let Some(document) = crate::utils::dom::document() {
                    if let Some(head) = document.head() {
                        let _ = head.append_child(&style_element);
                    }
                }
            }
        }
    });

    let class = move || {
        let current_config = config.get_untracked();
        let mut classes = vec!["ant-config-provider"];

        if current_config.component_disabled {
            classes.push("ant-config-provider-disabled");
        }

        if let Some(size) = current_config.component_size {
            match size {
                ComponentSize::Small => classes.push("ant-config-provider-sm"),
                ComponentSize::Large => classes.push("ant-config-provider-lg"),
                _ => {}
            }
        }

        classes.join(" ")
    };

    view! {
        <div class={class()}>
            {children()}
        </div>
    }
}

/// Hook to access the current configuration
pub fn use_config() -> Option<ReadSignal<Config>> {
    use_context::<RwSignal<Config>>().map(|config| config.read_only())
}

/// Generate theme colors based on primary color and algorithm
pub fn generate_colors(primary_color: &str, algorithm: Option<&[ThemeAlgorithm]>) -> Theme {
    let mut theme = Theme {
        primary_color: primary_color.to_string(),
        info_color: primary_color.to_string(),
        success_color: "#52c41a".to_string(),
        warning_color: "#faad14".to_string(),
        error_color: "#ff4d4f".to_string(),
    };

    if let Some(algorithms) = algorithm {
        for algo in algorithms {
            match algo {
                ThemeAlgorithm::Dark => {
                    theme.primary_color = "#177ddc".to_string();
                    theme.success_color = "#49aa19".to_string();
                    theme.warning_color = "#d89614".to_string();
                    theme.error_color = "#a61d24".to_string();
                }
                ThemeAlgorithm::Compact => {
                    // Compact 主题不改变颜色
                }
                ThemeAlgorithm::Default => {}
            }
        }
    }

    theme
}
