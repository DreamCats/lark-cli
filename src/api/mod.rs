use std::collections::HashMap;
use serde::Deserialize;
use crate::auth::AuthManager;
use crate::error::{LarkError, Result};

pub mod wiki;
pub mod docx;
pub mod permission;
pub mod document;
pub mod convert_blocks;
pub mod create_nested_blocks;
pub mod get_blocks;
pub mod batch_update_blocks;
pub mod delete_blocks;
pub mod file;
pub mod media;
pub mod download_media;
pub mod get_board_image;
pub mod message;
pub mod search_chats;
pub mod get_message_history;
pub mod import_documents;
pub mod block_converter;
pub mod board;
pub mod create_board_notes;
pub mod get_user_info;

pub use wiki::WikiApi;
pub use docx::DocxApi;
pub use permission::PermissionApi;
pub use document::DocumentApi;
pub use convert_blocks::ConvertBlocksApi;
pub use create_nested_blocks::{CreateNestedBlocksApi, DescendantBlock};
pub use get_blocks::GetBlocksApi;
pub use batch_update_blocks::BatchUpdateBlocksApi;
pub use delete_blocks::DeleteBlocksApi;
pub use file::FileApi;
pub use media::MediaApi;
pub use download_media::DownloadMediaApi;
pub use get_board_image::GetBoardImageApi;
pub use message::MessageApi;
pub use search_chats::SearchChatsApi;
pub use get_message_history::GetMessageHistoryApi;
pub use import_documents::ImportDocumentsApi;
pub use board::BoardApi;
pub use create_board_notes::CreateBoardNotesApi;
pub use get_user_info::GetUserInfoApi;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

#[derive(Clone)]
pub struct ApiClient {
    pub(crate) auth_manager: AuthManager,
    pub(crate) client: reqwest::Client,
}

impl ApiClient {
    pub fn new(auth_manager: AuthManager) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            auth_manager,
            client,
        }
    }

    /// Send GET request
    pub async fn get<T>(&self, url: &str, params: Option<HashMap<String, String>>) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let auth_header = self.auth_manager.get_auth_header().await?;

        let mut request = self.client
            .get(url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json; charset=utf-8");

        if let Some(p) = params {
            request = request.query(&p);
        }

        tracing::debug!("Sending GET request to: {:?}", request);

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(LarkError::NetworkError(format!(
                "HTTP request failed, status code: {}, response body: {}",
                response.status(),
                response.text().await?, 
            )));
        }

        let text = response.text().await?;
        tracing::debug!("Received response: {}", text);

        let api_response: ApiResponse<T> = serde_json::from_str(&text)?;

        if api_response.code != 0 {
            return Err(LarkError::ApiError {
                code: api_response.code,
                message: api_response.msg,
            });
        }

        api_response.data.ok_or_else(|| {
            LarkError::ParseError("Response data field is empty".to_string())
        })
    }

    /// Send POST request
    #[allow(dead_code)]
    pub async fn post<T, B>(&self, url: &str, body: &B) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        B: serde::Serialize,
    {
        let auth_header = self.auth_manager.get_auth_header().await?;

        tracing::debug!("Sending POST request to: {}", url);
        tracing::debug!("Request body: {:?}", serde_json::to_string(body)?);

        let response = self.client
            .post(url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json; charset=utf-8")
            .json(body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LarkError::NetworkError(format!(
                "HTTP request failed, status code: {}, response body: {}",
                response.status(),
                response.text().await?
            )));
        }

        let text = response.text().await?;
        tracing::debug!("Received response: {}", text);

        let api_response: ApiResponse<T> = serde_json::from_str(&text)?;

        if api_response.code != 0 {
            return Err(LarkError::ApiError {
                code: api_response.code,
                message: api_response.msg,
            });
        }

        api_response.data.ok_or_else(|| {
            LarkError::ParseError("Response data field is empty".to_string())
        })
    }

    /// Send POST request with query parameters
    pub async fn post_with_params<T, B>(
        &self,
        url: &str,
        params: Option<HashMap<String, String>>,
        body: &B,
    ) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        B: serde::Serialize,
    {
        let auth_header = self.auth_manager.get_auth_header().await?;

        let mut request = self.client
            .post(url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json; charset=utf-8")
            .json(body);

        if let Some(p) = params {
            request = request.query(&p);
        }

        tracing::debug!("Sending POST request to: {}", url);
        tracing::debug!("Request body: {:?}", serde_json::to_string(body)?);

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(LarkError::NetworkError(format!(
                "HTTP request failed, status code: {}, response: {}",
                response.status(),
                response.text().await?
            )));
        }

        let text = response.text().await?;
        tracing::debug!("Received response: {}", text);

        let api_response: ApiResponse<T> = serde_json::from_str(&text)?;

        if api_response.code != 0 {
            return Err(LarkError::ApiError {
                code: api_response.code,
                message: api_response.msg,
            });
        }

        api_response.data.ok_or_else(|| {
            LarkError::ParseError("Response data field is empty".to_string())
        })
    }

    /// Send PATCH request with query parameters
    pub async fn patch_with_params<T, B>(
        &self,
        url: &str,
        params: Option<HashMap<String, String>>,
        body: &B,
    ) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        B: serde::Serialize,
    {
        let auth_header = self.auth_manager.get_auth_header().await?;

        let mut request = self.client
            .patch(url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json; charset=utf-8")
            .json(body);

        if let Some(p) = params {
            request = request.query(&p);
        }

        tracing::debug!("Sending PATCH request to: {}", url);
        tracing::debug!("Request body: {:?}", serde_json::to_string(body)?);

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(LarkError::NetworkError(format!(
                "HTTP request failed, status code: {}, response: {}",
                response.status(),
                response.text().await?
            )));
        }

        let text = response.text().await?;
        tracing::debug!("Received response: {}", text);

        let api_response: ApiResponse<T> = serde_json::from_str(&text)?;

        if api_response.code != 0 {
            return Err(LarkError::ApiError {
                code: api_response.code,
                message: api_response.msg,
            });
        }

        api_response.data.ok_or_else(|| {
            LarkError::ParseError("Response data field is empty".to_string())
        })
    }

    /// Send POST request with multipart/form-data
    pub async fn post_form_data<T>(
        &self,
        url: &str,
        body: Vec<u8>,
        headers: HashMap<String, String>,
    ) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let auth_header = self.auth_manager.get_auth_header().await?;

        tracing::debug!("Sending POST multipart request to: {}", url);

        let mut request = self.client
            .post(url)
            .header("Authorization", auth_header);

        // 添加自定义请求头
        for (key, value) in headers {
            request = request.header(key, value);
        }

        let response = request
            .body(body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LarkError::NetworkError(format!(
                "HTTP request failed, status code: {}",
                response.status()
            )));
        }

        let text = response.text().await?;
        tracing::debug!("Received response: {}", text);

        let api_response: ApiResponse<T> = serde_json::from_str(&text)?;

        if api_response.code != 0 {
            return Err(LarkError::ApiError {
                code: api_response.code,
                message: api_response.msg,
            });
        }

        api_response.data.ok_or_else(|| {
            LarkError::ParseError("Response data field is empty".to_string())
        })
    }

    /// Send DELETE request with query parameters and body
    pub async fn delete_with_params<T, B>(
        &self,
        url: &str,
        params: Option<HashMap<String, String>>,
        body: &B,
    ) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        B: serde::Serialize,
    {
        let auth_header = self.auth_manager.get_auth_header().await?;

        let mut request = self.client
            .delete(url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json; charset=utf-8")
            .json(body);

        if let Some(p) = params {
            request = request.query(&p);
        }

        tracing::debug!("Sending DELETE request to: {}", url);
        tracing::debug!("Request body: {:?}", serde_json::to_string(body)?);

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(LarkError::NetworkError(format!(
                "HTTP request failed, status code: {}, response: {}",
                response.status(),
                response.text().await?
            )));
        }

        let text = response.text().await?;
        tracing::debug!("Received response: {}", text);

        let api_response: ApiResponse<T> = serde_json::from_str(&text)?;

        if api_response.code != 0 {
            return Err(LarkError::ApiError {
                code: api_response.code,
                message: api_response.msg,
            });
        }

        api_response.data.ok_or_else(|| {
            LarkError::ParseError("Response data field is empty".to_string())
        })
    }
}