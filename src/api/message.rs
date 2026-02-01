use super::ApiClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct SendMessageRequest {
    #[serde(rename = "receive_id")]
    pub receive_id: String,
    #[serde(rename = "msg_type")]
    pub msg_type: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "uuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SendMessageResponse {
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

pub struct MessageApi {
    client: ApiClient,
}

impl MessageApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    pub async fn send_message(
        &self,
        receive_id: &str,
        receive_id_type: &str,
        msg_type: &str,
        content: &str,
        uuid: Option<String>,
    ) -> Result<SendMessageResponse> {
        // 验证参数
        if receive_id.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "receive_id 参数是必需的".to_string()
            ));
        }

        if msg_type.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "msg_type 参数是必需的".to_string()
            ));
        }

        if content.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "content 参数是必需的".to_string()
            ));
        }

        // 验证 receive_id_type
        let valid_id_types = ["open_id", "union_id", "user_id", "email", "chat_id"];
        if !valid_id_types.contains(&receive_id_type) {
            return Err(crate::error::LarkError::ValidationError(
                format!("receive_id_type 必须是以下值之一：{}", valid_id_types.join(", "))
            ));
        }

        // 验证 msg_type
        let valid_msg_types = [
            "text", "post", "image", "file", "audio", "media", "sticker",
            "interactive", "share_chat", "share_user", "system"
        ];
        if !valid_msg_types.contains(&msg_type) {
            return Err(crate::error::LarkError::ValidationError(
                format!("msg_type 必须是以下值之一：{}", valid_msg_types.join(", "))
            ));
        }

        // 构建请求体
        let request = SendMessageRequest {
            receive_id: receive_id.to_string(),
            msg_type: msg_type.to_string(),
            content: content.to_string(),
            uuid,
        };

        // 构建查询参数
        let mut params = HashMap::new();
        params.insert("receive_id_type".to_string(), receive_id_type.to_string());

        // 构建URL
        let url = "https://open.larkoffice.com/open-apis/im/v1/messages";

        // 发送请求
        let data: SendMessageResponse = self.client
            .post_with_params(url, Some(params), &request)
            .await?;

        Ok(data)
    }
}