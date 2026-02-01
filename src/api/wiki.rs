use serde::{Deserialize, Serialize};
use crate::api::ApiClient;
use crate::error::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct KnowledgeSpaceNode {
    pub creator: String,
    pub has_child: bool,
    pub node_create_time: String,
    pub node_creator: String,
    pub node_token: String,
    pub node_type: String,
    pub obj_create_time: String,
    pub obj_edit_time: String,
    pub obj_token: String,
    pub obj_type: String,
    pub origin_node_token: String,
    pub origin_space_id: String,
    pub owner: String,
    pub parent_node_token: String,
    pub space_id: String,
    pub title: String,
}

#[derive(Debug, Deserialize)]
struct WikiNodeResponse {
    node: KnowledgeSpaceNode,
}


pub struct WikiApi {
    client: ApiClient,
}

impl WikiApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    /// 获取知识空间节点信息
    pub async fn get_knowledge_space_node(
        &self,
        token: &str,
        obj_type: Option<&str>,
    ) -> Result<KnowledgeSpaceNode> {
        let mut params = std::collections::HashMap::new();
        params.insert("token".to_string(), token.to_string());

        if let Some(obj_type) = obj_type {
            params.insert("obj_type".to_string(), obj_type.to_string());
        }

        let url = "https://open.larkoffice.com/open-apis/wiki/v2/spaces/get_node";

        // 手动发送请求并处理嵌套的响应结构
        let auth_header = self.client.auth_manager.get_auth_header().await?;

        let mut request = self.client.client
            .get(url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json; charset=utf-8");

        request = request.query(&params);

        tracing::debug!("Sending GET request to: {}", url);

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(crate::error::LarkError::NetworkError(format!(
                "HTTP request failed, status code: {}, message: {}",
                response.status(),
                response.text().await?,
            )));
        }

        let text = response.text().await?;
        tracing::debug!("Received response: {}", text);

        // 解析完整的 API 响应
        let api_response: crate::api::ApiResponse<WikiNodeResponse> = serde_json::from_str(&text)?;

        if api_response.code != 0 {
            return Err(crate::error::LarkError::ApiError {
                code: api_response.code,
                message: api_response.msg,
            });
        }

        api_response.data.map(|d| d.node).ok_or_else(|| {
            crate::error::LarkError::ParseError("Response data field is empty".to_string())
        })
    }
}