use super::ApiClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ConvertBlocksRequest {
    pub content_type: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
struct ConvertBlocksData {
    #[serde(rename = "first_level_block_ids")]
    first_level_block_ids: Vec<String>,
    #[serde(rename = "blocks")]
    blocks: Vec<Block>,
    #[serde(rename = "block_id_to_image_urls")]
    block_id_to_image_urls: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {
    pub block_id: String,
    pub block_type: i32,
    pub parent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<serde_json::Value>,
    #[serde(flatten)]
    pub content: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConvertBlocksResponse {
    pub first_level_block_ids: Vec<String>,
    pub blocks: Vec<Block>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id_to_image_urls: Option<serde_json::Value>,
}

pub struct ConvertBlocksApi {
    client: ApiClient,
}

impl ConvertBlocksApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    pub async fn convert_content_to_blocks(
        &self,
        content: &str,
        content_type: &str,
    ) -> Result<ConvertBlocksResponse> {
        // 验证参数
        if content.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "content 参数是必需的".to_string()
            ));
        }

        if content_type != "markdown" && content_type != "html" {
            return Err(crate::error::LarkError::ValidationError(
                "content_type 必须是 'markdown' 或 'html'".to_string()
            ));
        }

        let request = ConvertBlocksRequest {
            content_type: content_type.to_string(),
            content: content.to_string(),
        };

        let url = "https://open.larkoffice.com/open-apis/docx/v1/documents/blocks/convert";

        // 使用中间结构来处理嵌套响应
        let data: ConvertBlocksData = self.client
            .post(url, &request)
            .await?;

        Ok(ConvertBlocksResponse {
            first_level_block_ids: data.first_level_block_ids,
            blocks: data.blocks,
            block_id_to_image_urls: data.block_id_to_image_urls,
        })
    }
}