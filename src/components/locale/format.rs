use super::types::{LocaleConfig, NumberFormatConfig};
use chrono::{DateTime, Utc};

/// Number format types
#[derive(Clone, Debug)]
pub enum NumberFormatType {
    /// Decimal number
    Decimal,
    /// Currency
    Currency,
    /// Percentage
    Percentage,
}

/// Format date according to locale
pub fn format_date(date: &DateTime<Utc>, format: &str, config: &LocaleConfig) -> String {
    // TODO: 实现日期格式化
    date.format(format).to_string()
}

/// Format number according to locale
pub fn format_number(
    number: f64,
    config: &NumberFormatConfig,
    format_type: Option<NumberFormatType>,
) -> String {
    // TODO: 实现数字格式化
    match format_type {
        Some(NumberFormatType::Currency) => {
            format!("{}{}", config.currency_symbol, number)
        }
        Some(NumberFormatType::Percentage) => {
            format!("{}%", number * 100.0)
        }
        _ => number.to_string(),
    }
}
