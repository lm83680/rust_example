use rust_i18n::t;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ErrorCode {
    InvalidEmailFormat,
}

rust_i18n::i18n!("app");

impl ErrorCode {
    pub fn message(&self, lang: Option<&str>) -> String {
        let lang = lang.unwrap_or("en"); // 默认语言为 en
        match self {
            ErrorCode::InvalidEmailFormat => format!("{}",t!("InvalidEmailFormat",locale=lang))
        }
    }
}
