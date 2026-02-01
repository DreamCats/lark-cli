use serde::Deserialize;
use crate::api::ApiClient;
use crate::error::{LarkError, Result};

#[derive(Debug, Deserialize)]
pub struct DocumentContent {
    pub content: String,
}

pub struct DocxApi {
    client: ApiClient,
}

impl DocxApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    /// 获取文档纯文本内容
    pub async fn get_document_raw_content(
        &self,
        document_id: &str,
    ) -> Result<String> {
        let url = format!(
            "https://open.larkoffice.com/open-apis/docx/v1/documents/{}/raw_content",
            document_id
        );

        tracing::debug!("获取文档内容: {}", url);

        let auth_header = self.client.auth_manager.get_auth_header().await?;

        let response = self.client.client
            .get(&url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json; charset=utf-8")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LarkError::NetworkError(format!(
                "获取文档内容失败，状态码: {}",
                response.status()
            )));
        }

        let text = response.text().await?;
        tracing::debug!("收到文档内容，长度: {}", text.len());

        // 解析JSON响应
        let api_response: crate::api::ApiResponse<DocumentContent> = serde_json::from_str(&text)?;

        if api_response.code != 0 {
            return Err(LarkError::ApiError {
                code: api_response.code,
                message: api_response.msg,
            });
        }

        api_response.data
            .map(|d| d.content)
            .ok_or_else(|| LarkError::ParseError("响应数据字段为空".to_string()))
    }
}