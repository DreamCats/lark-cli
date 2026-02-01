use super::ApiClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct GetMessageHistoryRequest {
    #[serde(rename = "container_id_type")]
    pub container_id_type: String,
    #[serde(rename = "container_id")]
    pub container_id: String,
    #[serde(rename = "start_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "end_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "sort_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<String>,
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "page_token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetMessageHistoryResponse {
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[serde(rename = "page_token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "items")]
    pub items: Vec<MessageInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageInfo {
    #[serde(rename = "message_id")]
    pub message_id: String,
    #[serde(rename = "root_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_id: Option<String>,
    #[serde(rename = "parent_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "thread_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(rename = "msg_type")]
    pub msg_type: String,
    #[serde(rename = "create_time")]
    pub create_time: String,
    #[serde(rename = "update_time")]
    pub update_time: String,
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "updated")]
    pub updated: bool,
    #[serde(rename = "chat_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(rename = "sender")]
    pub sender: SenderInfo,
    #[serde(rename = "body")]
    pub body: MessageBody,
    #[serde(rename = "mentions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<MentionInfo>>,
    #[serde(rename = "upper_message_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_message_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SenderInfo {
    pub id: String,
    #[serde(rename = "id_type")]
    pub id_type: String,
    #[serde(rename = "sender_type")]
    pub sender_type: String,
    #[serde(rename = "tenant_key")]
    pub tenant_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageBody {
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MentionInfo {
    pub key: String,
    pub id: String,
    #[serde(rename = "id_type")]
    pub id_type: String,
    pub name: String,
    #[serde(rename = "tenant_key")]
    pub tenant_key: String,
}

pub struct GetMessageHistoryApi {
    client: ApiClient,
}

impl GetMessageHistoryApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get_message_history(
        &self,
        container_id_type: String,
        container_id: String,
        start_time: Option<String>,
        end_time: Option<String>,
        sort_type: Option<String>,
        page_size: Option<i32>,
        page_token: Option<String>,
    ) -> Result<GetMessageHistoryResponse> {
        // 验证参数
        let valid_container_types = ["chat", "thread"];
        if !valid_container_types.contains(&container_id_type.as_str()) {
            return Err(crate::error::LarkError::ValidationError(
                format!("container_id_type 必须是以下值之一：{}", valid_container_types.join(", "))
            ));
        }

        if container_id.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "container_id 参数是必需的".to_string()
            ));
        }

        if let Some(ref sort) = sort_type {
            let valid_sort_types = ["ByCreateTimeAsc", "ByCreateTimeDesc"];
            if !valid_sort_types.contains(&sort.as_str()) {
                return Err(crate::error::LarkError::ValidationError(
                    format!("sort_type 必须是以下值之一：{}", valid_sort_types.join(", "))
                ));
            }
        }

        if let Some(size) = page_size {
            if !(1..=50).contains(&size) {
                return Err(crate::error::LarkError::ValidationError(
                    "page_size 必须在1到50之间".to_string()
                ));
            }
        }

        // 构建查询参数
        let mut params = HashMap::new();
        params.insert("container_id_type".to_string(), container_id_type.clone());
        params.insert("container_id".to_string(), container_id.clone());

        if let Some(start) = start_time {
            params.insert("start_time".to_string(), start);
        }

        if let Some(end) = end_time {
            params.insert("end_time".to_string(), end);
        }

        if let Some(sort) = sort_type {
            params.insert("sort_type".to_string(), sort);
        }

        if let Some(size) = page_size {
            params.insert("page_size".to_string(), size.to_string());
        }

        if let Some(token) = page_token {
            params.insert("page_token".to_string(), token);
        }

        // 构建URL
        let url = "https://open.larkoffice.com/open-apis/im/v1/messages";

        // 发送请求
        let data: GetMessageHistoryResponse = self.client
            .get(url, Some(params))
            .await?;

        Ok(data)
    }
}