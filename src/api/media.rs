use super::ApiClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct UploadMediaRequest {
    pub file_name: String,
    pub parent_type: String,
    pub parent_node: String,
    pub file_content: Vec<u8>,
    pub size: u64,
    pub checksum: Option<String>,
    pub extra: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UploadMediaResponse {
    pub file_token: String,
}

pub struct MediaApi {
    client: ApiClient,
}

impl MediaApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    /// 上传媒体文件
    pub async fn upload_media(&self, request: UploadMediaRequest) -> Result<UploadMediaResponse> {
        // 验证参数
        if request.file_name.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "file_name 参数是必需的".to_string()
            ));
        }

        if request.file_name.len() > 250 {
            return Err(crate::error::LarkError::ValidationError(
                "文件名不能超过250个字符".to_string()
            ));
        }

        let valid_parent_types = ["doc_image", "docx_image", "sheet_image", "doc_file", "docx_file"];
        if !valid_parent_types.contains(&request.parent_type.as_str()) {
            return Err(crate::error::LarkError::ValidationError(
                format!("parent_type 必须是以下之一: {}", valid_parent_types.join(", "))
            ));
        }

        if request.parent_node.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "parent_node 参数是必需的".to_string()
            ));
        }

        if request.size > 20_971_520 { // 20MB限制
            return Err(crate::error::LarkError::ValidationError(
                "文件大小超过20MB限制".to_string()
            ));
        }

        if request.file_content.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "file_content 不能为空".to_string()
            ));
        }

        // 构建multipart/form-data请求
        let boundary = "----7MA4YWxkTrZu0gW";

        // 构建表单数据
        let mut body_lines = Vec::new();

        // 先添加参数字段（文件字段必须在最后）
        let mut fields = std::collections::HashMap::new();
        fields.insert("file_name", request.file_name.clone());
        fields.insert("parent_type", request.parent_type.clone());
        fields.insert("parent_node", request.parent_node.clone());
        fields.insert("size", request.file_content.len().to_string());

        if let Some(ref checksum) = request.checksum {
            fields.insert("checksum", checksum.clone());
        }
        if let Some(ref extra) = request.extra {
            fields.insert("extra", extra.clone());
        }

        // 添加参数字段
        for (name, value) in fields {
            body_lines.push(format!("--{}", boundary));
            body_lines.push(format!(r#"Content-Disposition: form-data; name="{}""#, name));
            body_lines.push("".to_string());
            body_lines.push(value);
        }

        // 添加文件字段（必须在最后）
        body_lines.push(format!("--{}", boundary));
        body_lines.push(format!(r#"Content-Disposition: form-data; name="file"; filename="{}""#, request.file_name));
        body_lines.push("Content-Type: application/octet-stream".to_string());
        body_lines.push("".to_string());

        // 构建请求体
        let text_part = body_lines.join("\r\n") + "\r\n";
        let mut body = text_part.into_bytes();
        body.extend_from_slice(&request.file_content);

        let end_boundary = format!("\r\n--{}--\r\n", boundary);
        body.extend_from_slice(end_boundary.as_bytes());

        // 设置请求头
        let mut headers = std::collections::HashMap::new();
        headers.insert("Content-Type".to_string(), format!("multipart/form-data; boundary={}", boundary));

        // 发送请求
        let response = self.client
            .post_form_data("https://open.larkoffice.com/open-apis/im/v1/files", body, headers)
            .await?;

        // 解析响应
        let upload_response: UploadMediaResponse = serde_json::from_value(response)?;
        Ok(upload_response)
    }

    /// 从文件路径上传媒体文件
    pub async fn upload_media_from_file(
        &self,
        file_path: &str,
        parent_type: &str,
        parent_node: &str,
        checksum: Option<String>,
        extra: Option<String>,
    ) -> Result<UploadMediaResponse> {
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

        // 获取文件名
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| crate::error::LarkError::ValidationError(
                "无法获取文件名".to_string()
            ))?
            .to_string();

        // 读取文件内容
        let file_content = fs::read(path)
            .map_err(|e| crate::error::LarkError::IoError(format!("读取文件失败: {}", e)))?;

        let size = file_content.len() as u64;

        let request = UploadMediaRequest {
            file_name,
            parent_type: parent_type.to_string(),
            parent_node: parent_node.to_string(),
            file_content,
            size,
            checksum,
            extra,
        };

        self.upload_media(request).await
    }
}