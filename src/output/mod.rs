use serde::Serialize;
use crate::error::{LarkError, Result};

pub enum OutputFormat {
    Text,
    Json,
}

pub fn format_output<T>(data: &T, format: OutputFormat) -> Result<String>
where
    T: Serialize + std::fmt::Debug,
{
    match format {
        OutputFormat::Text => Ok(format!("{:#?}", data)),
        OutputFormat::Json => format_json(data),
    }
}

fn format_json<T>(data: &T) -> Result<String>
where
    T: Serialize + ?Sized,
{
    serde_json::to_string_pretty(data)
        .map_err(|e| LarkError::ParseError(format!("JSON序列化失败: {}", e)))
}

// 为 String 实现 Output
pub fn format_string_output(data: &str, format: OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Text => Ok(data.to_string()),
        OutputFormat::Json => serde_json::to_string(data)
            .map_err(|e| LarkError::ParseError(format!("JSON序列化失败: {}", e))),
    }
}