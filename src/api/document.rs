use super::ApiClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CreateDocumentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDocumentResponse {
    pub document: DocumentInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentInfo {
    pub document_id: String,
    pub revision_id: i32,
    pub title: String,
    #[serde(skip_deserializing)]
    pub url: Option<String>,
}

pub struct DocumentApi {
    client: ApiClient,
}

impl DocumentApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    pub async fn create_document(
        &self,
        folder_token: Option<String>,
        title: Option<String>,
    ) -> Result<DocumentInfo> {
        // 验证标题长度
        if let Some(ref t) = title {
            if t.len() > 800 {
                return Err(crate::error::LarkError::ValidationError(
                    "文档标题不能超过800个字符".to_string()
                ));
            }
        }

        let request = CreateDocumentRequest {
            folder_token,
            title,
        };

        let url = "https://open.larkoffice.com/open-apis/docx/v1/documents";

        let response: CreateDocumentResponse = self.client
            .post(url, &request)
            .await?;

        // 添加文档URL
        let mut document_info = response.document;
        document_info.url = Some(format!("https://bytedance.larkoffice.com/docx/{}", document_info.document_id));

        Ok(document_info)
    }
}