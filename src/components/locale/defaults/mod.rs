//! 默认的本地化文本定义
//!
//! 这个模块包含了不同语言的默认文本定义，目前支持：
//! - 英语（美国）
//! - 中文（简体）

pub mod en_us;
pub mod zh_cn;

pub use en_us::get_default_texts as get_en_us_texts;
pub use zh_cn::get_default_texts as get_zh_cn_texts;
