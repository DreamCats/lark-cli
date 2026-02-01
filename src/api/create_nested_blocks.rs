use super::ApiClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct CreateNestedBlocksRequest {
    pub index: i32,
    pub children_id: Vec<String>,
    pub descendants: Vec<DescendantBlock>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DescendantBlock {
    #[serde(rename = "block_id")]
    pub block_id: String,
    #[serde(rename = "block_type")]
    pub block_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading1: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading2: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading3: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading4: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading5: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading6: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading7: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading8: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading9: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bullet: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordered: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub todo: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitable: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callout: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub divider: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_column: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mindnote: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagram: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jira_issue: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_card: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub okr: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub okr_objective: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub okr_key_result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub okr_progress: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub board: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isv: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_catalog: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_page_list: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_synced: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_synced: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_template: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agenda: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agenda_item: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agenda_item_title: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agenda_item_content: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iframe: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undefined: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_cell: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockIdRelation {
    #[serde(rename = "block_id")]
    pub block_id: String,
    #[serde(rename = "temporary_block_id")]
    pub temporary_block_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNestedBlocksResponse {
    #[serde(rename = "block_id_relations")]
    pub block_id_relations: Vec<BlockIdRelation>,
    #[serde(rename = "children")]
    pub children: Vec<serde_json::Value>,
    #[serde(rename = "client_token")]
    pub client_token: Option<String>,
    #[serde(rename = "document_revision_id")]
    pub document_revision_id: i32,
}

pub struct CreateNestedBlocksApi {
    client: ApiClient,
}

impl CreateNestedBlocksApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create_nested_blocks(
        &self,
        document_id: &str,
        block_id: &str,
        children_id: Vec<String>,
        descendants: Vec<DescendantBlock>,
        index: Option<i32>,
        document_revision_id: Option<i32>,
        client_token: Option<String>,
    ) -> Result<CreateNestedBlocksResponse> {
        // 验证参数
        if document_id.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "document_id 参数是必需的".to_string()
            ));
        }

        if children_id.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "children_id 参数是必需的".to_string()
            ));
        }

        if descendants.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "descendants 参数是必需的".to_string()
            ));
        }

        let index = index.unwrap_or(0);
        // index 现在支持负数，用于在指定位置前面插入块

        // 构建请求体
        let request = CreateNestedBlocksRequest {
            index,
            children_id,
            descendants,
        };

        // 调试：打印请求体
        if std::env::var("DEBUG_REQUEST").is_ok() {
            eprintln!("CreateNestedBlocks Request: {}", serde_json::to_string_pretty(&request)?);
        }

        // 构建查询参数
        let mut params = HashMap::new();
        if let Some(rev_id) = document_revision_id {
            params.insert("document_revision_id".to_string(), rev_id.to_string());
        }
        if let Some(token) = &client_token {
            params.insert("client_token".to_string(), token.clone());
        }

        // 构建URL
        let actual_block_id = if block_id.is_empty() {
            document_id
        } else {
            block_id
        };
        let url = format!(
            "https://open.larkoffice.com/open-apis/docx/v1/documents/{}/blocks/{}/descendant",
            document_id, actual_block_id
        );

        // 发送请求
        let data: CreateNestedBlocksResponse = self.client
            .post_with_params(&url, Some(params), &request)
            .await?;

        Ok(data)
    }
}