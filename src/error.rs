use std::fmt;

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum LarkError {
    ConfigError(String),
    AuthError(String),
    ApiError { code: i32, message: String },
    NetworkError(String),
    IoError(String),
    ParseError(String),
    HttpError(reqwest::Error),
    ValidationError(String),
}

impl fmt::Display for LarkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LarkError::ConfigError(msg) => write!(f, "配置错误: {}", msg),
            LarkError::AuthError(msg) => write!(f, "认证错误: {}", msg),
            LarkError::ApiError { code, message } => {
                write!(f, "API错误 ({}): {}", code, message)
            }
            LarkError::NetworkError(msg) => write!(f, "网络错误: {}", msg),
            LarkError::IoError(msg) => write!(f, "IO错误: {}", msg),
            LarkError::ParseError(msg) => write!(f, "解析错误: {}", msg),
            LarkError::HttpError(err) => write!(f, "HTTP错误: {}", err),
            LarkError::ValidationError(msg) => write!(f, "验证错误: {}", msg),
        }
    }
}

impl std::error::Error for LarkError {}

impl From<std::io::Error> for LarkError {
    fn from(err: std::io::Error) -> Self {
        LarkError::IoError(err.to_string())
    }
}

impl From<reqwest::Error> for LarkError {
    fn from(err: reqwest::Error) -> Self {
        LarkError::HttpError(err)
    }
}

impl From<serde_json::Error> for LarkError {
    fn from(err: serde_json::Error) -> Self {
        LarkError::ParseError(format!("JSON解析失败: {}", err))
    }
}

pub type Result<T> = std::result::Result<T, LarkError>;