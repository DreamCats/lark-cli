use super::ApiClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct AddPermissionRequest {
    pub member_type: String,
    pub member_id: String,
    pub perm: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perm_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub collaborator_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddPermissionResponse {
    pub member: PermissionMember,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PermissionMember {
    pub member_type: String,
    pub member_id: String,
    pub perm: String,
    #[serde(default)]
    pub perm_type: Option<String>,
    #[serde(rename = "type")]
    pub collaborator_type: Option<String>,
}

pub struct PermissionApi {
    client: ApiClient,
}

impl PermissionApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    pub async fn add_permission_member(
        &self,
        token: &str,
        doc_type: &str,
        request: AddPermissionRequest,
        need_notification: Option<bool>,
    ) -> Result<AddPermissionResponse> {
        let mut params = HashMap::new();
        params.insert("type".to_string(), doc_type.to_string());

        if let Some(notification) = need_notification {
            params.insert("need_notification".to_string(), notification.to_string());
        }

        let url = format!("https://open.larkoffice.com/open-apis/drive/v1/permissions/{}/members", token);

        // tracing::debug!("Request params: {:?}", params);
        // tracing::debug!("Request body: {:?}", serde_json::to_string(&request)?);

        let response = self.client
            .post_with_params(&url, Some(params), &request)
            .await?;

        Ok(response)
    }
}