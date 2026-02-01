use super::ApiClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct GetBlocksRequest {
    #[serde(rename = "page_size")]
    pub page_size: i32,
    #[serde(rename = "page_token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "document_revision_id")]
    pub document_revision_id: i32,
    #[serde(rename = "user_id_type")]
    pub user_id_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockItem {
    #[serde(rename = "block_id")]
    pub block_id: String,
    #[serde(rename = "block_type")]
    pub block_type: i32,
    #[serde(rename = "parent_id")]
    pub parent_id: String,
    #[serde(rename = "children")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<String>>,
    #[serde(flatten)]
    pub content: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetBlocksResponse {
    pub items: Vec<BlockItem>,
    #[serde(rename = "page_token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}

pub struct GetBlocksApi {
    client: ApiClient,
}

impl GetBlocksApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    pub async fn get_document_blocks(
        &self,
        document_id: &str,
        page_size: Option<i32>,
        page_token: Option<String>,
        document_revision_id: Option<i32>,
        user_id_type: Option<String>,
    ) -> Result<GetBlocksResponse> {
        // 验证参数
        if document_id.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "document_id 参数是必需的".to_string()
            ));
        }

        let page_size = page_size.unwrap_or(500);
        if !(1..=500).contains(&page_size) {
            return Err(crate::error::LarkError::ValidationError(
                "page_size 参数必须在 1-500 之间".to_string()
            ));
        }

        let user_id_type = user_id_type.unwrap_or_else(|| "open_id".to_string());
        let valid_user_id_types = ["open_id", "union_id", "user_id"];
        if !valid_user_id_types.contains(&user_id_type.as_str()) {
            return Err(crate::error::LarkError::ValidationError(
                format!("user_id_type 必须是以下值之一：{}", valid_user_id_types.join(", "))
            ));
        }

        let document_revision_id = document_revision_id.unwrap_or(-1);

        // 构建查询参数
        let mut params = HashMap::new();
        params.insert("page_size".to_string(), page_size.to_string());
        params.insert("document_revision_id".to_string(), document_revision_id.to_string());
        params.insert("user_id_type".to_string(), user_id_type.clone());

        if let Some(token) = &page_token {
            params.insert("page_token".to_string(), token.clone());
        }

        // 构建URL
        let url = format!(
            "https://open.larkoffice.com/open-apis/docx/v1/documents/{}/blocks",
            document_id
        );

        // 发送请求
        let data: GetBlocksResponse = self.client
            .get(&url, Some(params))
            .await?;

        Ok(data)
    }

    /// 获取文档的所有块（自动处理分页）
    pub async fn get_all_document_blocks(
        &self,
        document_id: &str,
        document_revision_id: Option<i32>,
        user_id_type: Option<String>,
    ) -> Result<Vec<BlockItem>> {
        let mut all_blocks = Vec::new();
        let mut page_token: Option<String> = None;
        let page_size = 500; // 使用最大分页大小

        loop {
            let response = self.get_document_blocks(
                document_id,
                Some(page_size),
                page_token.clone(),
                document_revision_id,
                user_id_type.clone(),
            ).await?;

            all_blocks.extend(response.items);

            if !response.has_more {
                break;
            }

            page_token = response.page_token;
        }

        Ok(all_blocks)
    }
}