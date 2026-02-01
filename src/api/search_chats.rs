use super::ApiClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct SearchChatsRequest {
    #[serde(rename = "user_id_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "page_token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchChatsResponse {
    #[serde(rename = "items")]
    pub items: Vec<ChatInfo>,
    #[serde(rename = "page_token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatInfo {
    #[serde(rename = "chat_id")]
    pub chat_id: String,
    #[serde(rename = "avatar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "owner_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "owner_id_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id_type: Option<String>,
    #[serde(rename = "external")]
    pub external: bool,
    #[serde(rename = "tenant_key")]
    pub tenant_key: String,
    #[serde(rename = "chat_status")]
    pub chat_status: String,
}

pub struct SearchChatsApi {
    client: ApiClient,
}

impl SearchChatsApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    pub async fn search_chats(
        &self,
        user_id_type: Option<String>,
        query: Option<String>,
        page_token: Option<String>,
        page_size: Option<i32>,
    ) -> Result<SearchChatsResponse> {
        // 验证参数
        if let Some(ref q) = query {
            if q.len() > 64 {
                return Err(crate::error::LarkError::ValidationError(
                    "查询关键词长度不能超过64个字符".to_string()
                ));
            }
        }

        if let Some(size) = page_size {
            if !(1..=100).contains(&size) {
                return Err(crate::error::LarkError::ValidationError(
                    "page_size 必须在1到100之间".to_string()
                ));
            }
        }

        // 验证 user_id_type
        if let Some(ref id_type) = user_id_type {
            let valid_id_types = ["open_id", "union_id", "user_id"];
            if !valid_id_types.contains(&id_type.as_str()) {
                return Err(crate::error::LarkError::ValidationError(
                    format!("user_id_type 必须是以下值之一：{}", valid_id_types.join(", "))
                ));
            }
        }

        // 构建查询参数
        let mut params = HashMap::new();

        if let Some(id_type) = user_id_type {
            params.insert("user_id_type".to_string(), id_type);
        }

        if let Some(q) = query {
            params.insert("query".to_string(), q);
        }

        if let Some(token) = page_token {
            params.insert("page_token".to_string(), token);
        }

        if let Some(size) = page_size {
            params.insert("page_size".to_string(), size.to_string());
        }

        // 构建URL
        let url = "https://open.larkoffice.com/open-apis/im/v1/chats/search";

        // 发送请求
        let data: SearchChatsResponse = self.client
            .get(url, Some(params))
            .await?;

        Ok(data)
    }
}