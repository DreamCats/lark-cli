use super::ApiClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct ReadFileRequest {
    pub file_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReadFileResponse {
    pub content: Vec<u8>,
    pub size: u64,
    pub encoding: String,
}

#[derive(Debug, Serialize)]
pub struct WriteFileRequest {
    pub file_path: String,
    pub content: Vec<u8>,
    pub overwrite: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WriteFileResponse {
    pub success: bool,
    pub size: u64,
    pub message: String,
}

pub struct FileApi {
    _client: ApiClient,
}

impl FileApi {
    pub fn new(client: ApiClient) -> Self {
        Self { _client: client }
    }

    /// 读取文件内容和大小
    pub async fn read_file(&self, file_path: &str) -> Result<ReadFileResponse> {
        // 验证参数
        if file_path.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "file_path 参数是必需的".to_string()
            ));
        }

        let path = Path::new(file_path);

        // 检查文件是否存在
        if !path.exists() {
            return Err(crate::error::LarkError::ValidationError(
                format!("文件不存在: {}", file_path)
            ));
        }

        // 检查是否为文件
        if !path.is_file() {
            return Err(crate::error::LarkError::ValidationError(
                format!("路径不是文件: {}", file_path)
            ));
        }

        // 获取文件大小
        let metadata = fs::metadata(path)
            .map_err(|e| crate::error::LarkError::IoError(format!("获取文件元数据失败: {}", e)))?;

        let size = metadata.len();

        // 读取文件内容
        let content = fs::read(path)
            .map_err(|e| crate::error::LarkError::IoError(format!("读取文件失败: {}", e)))?;

        // 验证读取的内容大小是否一致
        if content.len() as u64 != size {
            return Err(crate::error::LarkError::IoError(
                format!("文件读取不完整，期望大小: {}, 实际读取: {}", size, content.len())
            ));
        }

        Ok(ReadFileResponse {
            content,
            size,
            encoding: "binary".to_string(),
        })
    }

    /// 写入文件内容
    pub async fn write_file(&self, file_path: &str, content: Vec<u8>, overwrite: bool) -> Result<WriteFileResponse> {
        // 验证参数
        if file_path.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "file_path 参数是必需的".to_string()
            ));
        }

        let path = Path::new(file_path);

        // 检查文件是否已存在
        if path.exists() && !overwrite {
            return Err(crate::error::LarkError::ValidationError(
                format!("文件已存在，使用 --overwrite 参数覆盖: {}", file_path)
            ));
        }

        // 确保父目录存在
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| crate::error::LarkError::IoError(format!("创建父目录失败: {}", e)))?;
        }

        // 写入文件内容
        fs::write(path, &content)
            .map_err(|e| crate::error::LarkError::IoError(format!("写入文件失败: {}", e)))?;

        let size = content.len() as u64;

        Ok(WriteFileResponse {
            success: true,
            size,
            message: format!("文件写入成功: {}", file_path),
        })
    }
}