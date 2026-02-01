use super::ApiClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct DeleteBlocksRequest {
    #[serde(rename = "start_index")]
    pub start_index: i32,
    #[serde(rename = "end_index")]
    pub end_index: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteBlocksResponse {
    #[serde(rename = "document_revision_id")]
    pub document_revision_id: i32,
    #[serde(rename = "client_token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

pub struct DeleteBlocksApi {
    client: ApiClient,
}

impl DeleteBlocksApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    pub async fn delete_blocks(
        &self,
        document_id: &str,
        block_id: &str,
        start_index: i32,
        end_index: i32,
        document_revision_id: Option<i32>,
        client_token: Option<String>,
    ) -> Result<DeleteBlocksResponse> {
        // 验证参数
        if document_id.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "document_id 参数是必需的".to_string()
            ));
        }

        if block_id.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "block_id 参数是必需的".to_string()
            ));
        }

        if start_index < 0 {
            return Err(crate::error::LarkError::ValidationError(
                "start_index 必须大于等于 0".to_string()
            ));
        }

        if end_index < 1 {
            return Err(crate::error::LarkError::ValidationError(
                "end_index 必须大于等于 1".to_string()
            ));
        }

        if start_index >= end_index {
            return Err(crate::error::LarkError::ValidationError(
                "start_index 必须小于 end_index".to_string()
            ));
        }

        // 构建请求体
        let request = DeleteBlocksRequest {
            start_index,
            end_index,
        };

        // 构建查询参数
        let mut params = HashMap::new();
        params.insert("document_revision_id".to_string(), document_revision_id.unwrap_or(-1).to_string());
        if let Some(token) = &client_token {
            params.insert("client_token".to_string(), token.clone());
        }

        // 构建URL
        let url = format!(
            "https://open.larkoffice.com/open-apis/docx/v1/documents/{}/blocks/{}/children/batch_delete",
            document_id, block_id
        );

        // 发送请求
        let data: DeleteBlocksResponse = self.client
            .delete_with_params(&url, Some(params), &request)
            .await?;

        Ok(data)
    }
}