mod defaults;
mod format;
mod hooks;
mod provider;
mod types;

pub use defaults::*;
pub use format::*;
pub use hooks::*;
pub use provider::*;
pub use types::*;

// use defaults::{get_en_us, get_zh_cn};
use types::{Language, LocaleText};

// // 内部使用的辅助函数
// fn get_default_texts(language: Language) -> LocaleText {
//     match language {
//         Language::ZhCN => get_zh_cn(),
//         Language::EnUS => get_en_us(),
//     }
// }
