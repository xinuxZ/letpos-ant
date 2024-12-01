use leptos::prelude::*;
// use std::collections::HashMap;

/// 定义语言类型
/// Supported languages for localization
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Language {
    /// Chinese (Simplified)
    ZhCN,
    /// English (United States)
    EnUS,
}

/// Localized text content for all components
#[derive(Clone, Debug)]
pub struct LocaleText {
    /// Current locale identifier
    pub locale: String,
    /// Default placeholder text
    pub placeholder: String,
    /// Placeholder text for select components
    pub select_placeholder: String,

    /// Modal dialog OK button text
    pub ok_text: String,
    /// Modal dialog cancel button text
    pub cancel_text: String,
    /// Modal dialog simple OK button text
    pub just_ok_text: String,

    /// Popconfirm OK button text
    pub ok: String,
    /// Popconfirm cancel button text
    pub cancel: String,
    /// Popconfirm yes button text
    pub yes: String,
    /// Popconfirm no button text
    pub no: String,

    /// Form-related localized texts
    pub form: FormLocale,
    /// Table-related localized texts
    pub table: TableLocale,
    /// Upload-related localized texts
    pub upload: UploadLocale,
    /// Empty state localized texts
    pub empty: EmptyLocale,
}
/// Form-specific localization
#[derive(Clone, Debug)]
pub struct FormLocale {
    /// Text for optional fields
    pub optional: String,
    /// Text for required fields
    pub required: String,
    /// Form validation messages
    pub validate_messages: ValidateMessages,
}

/// Validation messages for form fields
#[derive(Clone, Debug)]
pub struct ValidateMessages {
    /// Default validation error message
    pub default_message: String,
    /// Required field validation message
    pub required: String,
    /// Enum validation message
    pub enum_message: String,
    /// Whitespace validation message
    pub whitespace: String,
    /// Date validation messages
    pub date: DateValidateMessages,
    /// Type validation messages
    pub types: TypeValidateMessages,
    /// String validation messages
    pub string: StringValidateMessages,
    /// Number validation messages
    pub number: NumberValidateMessages,
    /// Array validation messages
    pub array: ArrayValidateMessages,
}

/// Date validation message translations
#[derive(Clone, Debug)]
pub struct DateValidateMessages {
    /// Date format error message
    pub format: String,
    /// Date parsing error message
    pub parse: String,
    /// Invalid date error message
    pub invalid: String,
}

/// Type validation message translations
#[derive(Clone, Debug)]
pub struct TypeValidateMessages {
    /// String type validation message
    pub string: String,
    /// Method type validation message
    pub method: String,
    /// Array type validation message
    pub array: String,
    /// Object type validation message
    pub object: String,
    /// Number type validation message
    pub number: String,
    /// Date type validation message
    pub date: String,
    /// Boolean type validation message
    pub boolean: String,
    /// Integer type validation message
    pub integer: String,
    /// Float type validation message
    pub float: String,
    /// RegExp type validation message
    pub regexp: String,
    /// Email type validation message
    pub email: String,
    /// URL type validation message
    pub url: String,
    /// Hexadecimal type validation message
    pub hex: String,
}

/// String validation message translations
#[derive(Clone, Debug)]
pub struct StringValidateMessages {
    /// String length validation message
    pub len: String,
    /// String minimum length validation message
    pub min: String,
    /// String maximum length validation message
    pub max: String,
    /// String length range validation message
    pub range: String,
}

/// Number validation message translations
#[derive(Clone, Debug)]
pub struct NumberValidateMessages {
    /// Number exact value validation message
    pub len: String,
    /// Number minimum value validation message
    pub min: String,
    /// Number maximum value validation message
    pub max: String,
    /// Number range validation message
    pub range: String,
}

/// Array validation message translations
#[derive(Clone, Debug)]
pub struct ArrayValidateMessages {
    /// Array length validation message
    pub len: String,
    /// Array minimum length validation message
    pub min: String,
    /// Array maximum length validation message
    pub max: String,
    /// Array length range validation message
    pub range: String,
}

/// Table-specific localization
#[derive(Clone, Debug)]
pub struct TableLocale {
    /// Filter menu title
    pub filter_title: String,
    /// Filter confirm button text
    pub filter_confirm: String,
    /// Filter reset button text
    pub filter_reset: String,
    /// Empty filter message
    pub filter_empty: String,
    /// Select all rows text
    pub select_all: String,
    /// Invert selection text
    pub select_invert: String,
    /// Clear selection text
    pub select_none: String,
    /// Select all data text
    pub selection_all: String,
    /// Sort menu title
    pub sort_title: String,
    /// Expand row text
    pub expand_text: String,
    /// Collapse row text
    pub collapse_text: String,
    /// Trigger descending sort text
    pub trigger_desc: String,
    /// Trigger ascending sort text
    pub trigger_asc: String,
    /// Cancel sort text
    pub cancel_sort: String,
}

/// Upload component localization
#[derive(Clone, Debug)]
pub struct UploadLocale {
    /// File uploading status text
    pub uploading: String,
    /// Remove file button text
    pub remove_file: String,
    /// Upload error message
    pub upload_error: String,
    /// Preview file button text
    pub preview_file: String,
    /// Download file button text
    pub download_file: String,
}

/// Empty state localization
#[derive(Clone, Debug)]
pub struct EmptyLocale {
    /// Empty state description text
    pub description: String,
}

/// 获取中文文本
fn get_zh_cn() -> LocaleText {
    LocaleText {
        locale: "zh_CN".to_string(),
        placeholder: "请选择".to_string(),
        select_placeholder: "请选择".to_string(),

        ok_text: "确定".to_string(),
        cancel_text: "取消".to_string(),
        just_ok_text: "知道了".to_string(),

        ok: "确定".to_string(),
        cancel: "取消".to_string(),
        yes: "是".to_string(),
        no: "否".to_string(),

        form: FormLocale {
            optional: "（可选）".to_string(),
            required: "（必填）".to_string(),
            validate_messages: ValidateMessages {
                default_message: "字段验证错误".to_string(),
                required: "请输入%s".to_string(),
                enum_message: "%s必须是其中一个[%s]".to_string(),
                whitespace: "%s不能为空".to_string(),
                // ... 其他验证消息
                date: DateValidateMessages {
                    format: "%s日期格式无效".to_string(),
                    parse: "%s不能转换为日期".to_string(),
                    invalid: "%s是一个无效日期".to_string(),
                },
                types: TypeValidateMessages {
                    string: "%s不是一个有效的字符串".to_string(),
                    method: "%s不是一个有效的函数".to_string(),
                    array: "%s不是一个有效的数组".to_string(),
                    object: "%s不是一个有效的对象".to_string(),
                    number: "%s不是一个有效的数字".to_string(),
                    date: "%s不是一个有效的日期".to_string(),
                    boolean: "%s不是一个有效的布尔值".to_string(),
                    integer: "%s不是一个有效的整数".to_string(),
                    float: "%s不是一个有效的浮点数".to_string(),
                    regexp: "%s不是一个有效的正则表达式".to_string(),
                    email: "%s不是一个有效的邮箱".to_string(),
                    url: "%s不是一个有效的链接".to_string(),
                    hex: "%s不是一个有效的十六进制".to_string(),
                },
                string: StringValidateMessages {
                    len: "%s须为%s个字符".to_string(),
                    min: "%s最少%s个字符".to_string(),
                    max: "%s最多%s个字符".to_string(),
                    range: "%s须在%s-%s个字符之间".to_string(),
                },
                number: NumberValidateMessages {
                    len: "%s必须等于%s".to_string(),
                    min: "%s最小值为%s".to_string(),
                    max: "%s最大值为%s".to_string(),
                    range: "%s须在%s-%s之间".to_string(),
                },
                array: ArrayValidateMessages {
                    len: "%s须为%s个条目".to_string(),
                    min: "%s最少%s个条目".to_string(),
                    max: "%s最多%s个条目".to_string(),
                    range: "%s须在%s-%s个条目之间".to_string(),
                },
            },
        },

        table: TableLocale {
            filter_title: "筛选".to_string(),
            filter_confirm: "确定".to_string(),
            filter_reset: "重置".to_string(),
            filter_empty: "无筛选项".to_string(),
            select_all: "全选当页".to_string(),
            select_invert: "反选当页".to_string(),
            select_none: "清空所有".to_string(),
            selection_all: "全选所有".to_string(),
            sort_title: "排序".to_string(),
            expand_text: "展开行".to_string(),
            collapse_text: "关闭行".to_string(),
            trigger_desc: "点击降序".to_string(),
            trigger_asc: "点击升序".to_string(),
            cancel_sort: "取消排序".to_string(),
        },

        upload: UploadLocale {
            uploading: "文件上传中...".to_string(),
            remove_file: "删除文件".to_string(),
            upload_error: "上传错误".to_string(),
            preview_file: "预览文件".to_string(),
            download_file: "下载文件".to_string(),
        },

        empty: EmptyLocale {
            description: "暂无数据".to_string(),
        },
    }
}

/// 获取英文文本
fn get_en_us() -> LocaleText {
    LocaleText {
        locale: "en_US".to_string(),
        placeholder: "Please select".to_string(),
        select_placeholder: "Please select".to_string(),

        ok_text: "OK".to_string(),
        cancel_text: "Cancel".to_string(),
        just_ok_text: "OK".to_string(),

        ok: "OK".to_string(),
        cancel: "Cancel".to_string(),
        yes: "Yes".to_string(),
        no: "No".to_string(),

        // ... 其他英文文本
        form: FormLocale {
            optional: "(optional)".to_string(),
            required: "(required)".to_string(),
            validate_messages: ValidateMessages {
                default_message: "Field validation error".to_string(),
                required: "Please enter %s".to_string(),
                enum_message: "%s must be one of [%s]".to_string(),
                whitespace: "%s cannot be empty".to_string(),
                // ... 其他验证消息
                date: DateValidateMessages {
                    format: "%s date format is invalid".to_string(),
                    parse: "%s cannot be converted to a date".to_string(),
                    invalid: "%s is an invalid date".to_string(),
                },
                types: TypeValidateMessages {
                    string: "%s is not a valid string".to_string(),
                    method: "%s is not a valid function".to_string(),
                    array: "%s is not a valid array".to_string(),
                    object: "%s is not a valid object".to_string(),
                    number: "%s is not a valid number".to_string(),
                    date: "%s is not a valid date".to_string(),
                    boolean: "%s is not a valid boolean".to_string(),
                    integer: "%s is not a valid integer".to_string(),
                    float: "%s is not a valid float".to_string(),
                    regexp: "%s is not a valid regular expression".to_string(),
                    email: "%s is not a valid email".to_string(),
                    url: "%s is not a valid URL".to_string(),
                    hex: "%s is not a valid hexadecimal".to_string(),
                },
                string: StringValidateMessages {
                    len: "%s must be %s characters".to_string(),
                    min: "%s must be at least %s characters".to_string(),
                    max: "%s cannot be longer than %s characters".to_string(),
                    range: "%s must be between %s-%s characters".to_string(),
                },
                number: NumberValidateMessages {
                    len: "%s must equal %s".to_string(),
                    min: "%s cannot be less than %s".to_string(),
                    max: "%s cannot be greater than %s".to_string(),
                    range: "%s must be between %s-%s".to_string(),
                },
                array: ArrayValidateMessages {
                    len: "%s must be exactly %s in length".to_string(),
                    min: "%s cannot be less than %s in length".to_string(),
                    max: "%s cannot be greater than %s in length".to_string(),
                    range: "%s must be between %s-%s in length".to_string(),
                },
            },
        },

        table: TableLocale {
            filter_title: "Filter menu".to_string(),
            filter_confirm: "OK".to_string(),
            filter_reset: "Reset".to_string(),
            filter_empty: "No filters".to_string(),
            select_all: "Select current page".to_string(),
            select_invert: "Invert current page".to_string(),
            select_none: "Clear all data".to_string(),
            selection_all: "Select all data".to_string(),
            sort_title: "Sort".to_string(),
            expand_text: "Expand row".to_string(),
            collapse_text: "Collapse row".to_string(),
            trigger_desc: "Click to sort descending".to_string(),
            trigger_asc: "Click to sort ascending".to_string(),
            cancel_sort: "Click to cancel sorting".to_string(),
        },

        upload: UploadLocale {
            uploading: "Uploading...".to_string(),
            remove_file: "Remove file".to_string(),
            upload_error: "Upload error".to_string(),
            preview_file: "Preview file".to_string(),
            download_file: "Download file".to_string(),
        },

        empty: EmptyLocale {
            description: "No data".to_string(),
        },
    }
}

/// 获取本地化文本
pub fn use_locale_text(language: Language) -> LocaleText {
    match language {
        Language::ZhCN => get_zh_cn(),
        Language::EnUS => get_en_us(),
    }
}

/// 创建 Provider 组件
/// Locale provider component for internationalization
#[component]
pub fn LocaleProvider(
    /// The language to use for localization
    #[prop(default = Language::ZhCN)]
    language: Language,
    /// Child components
    children: Children,
) -> impl IntoView {
    let locale = RwSignal::new(language);
    provide_context(locale);

    view! {
        <div class="ant-locale">
            {children()}
        </div>
    }
}

/// 获取当前语言的 hook
pub fn use_locale() -> Option<ReadSignal<Language>> {
    use_context::<RwSignal<Language>>().map(|locale| locale.read_only())
}

/// 获取当前语言文本的 hook
pub fn use_locale_text_with_context() -> Option<LocaleText> {
    use_locale().map(|locale| use_locale_text(locale.get()))
}
