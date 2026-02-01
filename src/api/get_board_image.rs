use super::ApiClient;
use crate::error::Result;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// 获取画板图片的响应
#[derive(Debug, Serialize)]
pub struct GetBoardImageResponse {
    /// 保存的文件路径
    pub file_path: String,
    /// 文件大小（字节）
    pub file_size: u64,
    /// MIME 类型（图片格式）
    pub content_type: String,
    /// 图片格式扩展名
    pub file_extension: String,
}

pub struct GetBoardImageApi {
    client: ApiClient,
}

impl GetBoardImageApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    /// 下载画板为图片
    pub async fn get_board_image(
        &self,
        whiteboard_id: &str,
        output_path: &str,
    ) -> Result<GetBoardImageResponse> {
        // 验证参数
        if whiteboard_id.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "whiteboard_id 参数是必需的".to_string(),
            ));
        }

        // 构建请求 URL
        let url = format!(
            "https://open.larkoffice.com/open-apis/board/v1/whiteboards/{}/download_as_image",
            whiteboard_id
        );

        // 获取认证头
        let auth_header = self.client.auth_manager.get_auth_header().await?;

        // 构建请求
        let response = self.client
            .client
            .get(&url)
            .header("Authorization", auth_header)
            .send()
            .await?;

        let status = response.status();
        tracing::debug!("Response status: {}", status);

        // 检查 HTTP 状态码
        match status.as_u16() {
            200 => {
                // 成功
            }
            400 => {
                let text = response.text().await.unwrap_or_default();
                // 尝试解析错误码
                if let Ok(api_resp) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(code) = api_resp.get("code").and_then(|c| c.as_i64()) {
                        let message = match code {
                            2890001 => "参数格式不正确".to_string(),
                            2890002 => "参数无效".to_string(),
                            2890003 => "找不到记录，whiteboard_id 不存在或图片不存在".to_string(),
                            _ => api_resp.get("msg").and_then(|m| m.as_str()).unwrap_or("请求参数错误").to_string(),
                        };
                        return Err(crate::error::LarkError::ApiError {
                            code: code as i32,
                            message,
                        });
                    }
                }
                return Err(crate::error::LarkError::ApiError {
                    code: 400,
                    message: "请求参数错误".to_string(),
                });
            }
            401 => {
                return Err(crate::error::LarkError::ApiError {
                    code: 401,
                    message: "认证失败，请检查 Authorization 参数".to_string(),
                });
            }
            403 => {
                return Err(crate::error::LarkError::ApiError {
                    code: 403,
                    message: "请求身份没有当前画板的阅读权限".to_string(),
                });
            }
            429 => {
                return Err(crate::error::LarkError::ApiError {
                    code: 429,
                    message: "请求超过接口频率限流值，请稍后再试".to_string(),
                });
            }
            500 => {
                return Err(crate::error::LarkError::ApiError {
                    code: 500,
                    message: "服务运行错误，请重试".to_string(),
                });
            }
            _ => {
                return Err(crate::error::LarkError::NetworkError(format!(
                    "HTTP 请求失败，状态码: {}",
                    status
                )));
            }
        }

        // 获取 Content-Type
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("image/png")
            .to_string();

        // 根据 MIME 类型确定文件扩展名
        let file_extension = match content_type.as_str() {
            "image/png" => "png",
            "image/jpeg" => "jpg",
            "image/gif" => "gif",
            "image/svg+xml" => "svg",
            _ => "png", // 默认使用 png
        };

        // 获取二进制内容
        let bytes = response.bytes().await?;
        let file_size = bytes.len() as u64;

        // 如果 output_path 是目录，则自动添加文件名
        let final_output_path = if output_path.ends_with('/') || output_path.ends_with('\\') {
            let path = Path::new(output_path);
            let filename = format!("{}.{}", whiteboard_id, file_extension);
            path.join(filename).to_str().unwrap().to_string()
        } else {
            output_path.to_string()
        };

        // 确保输出目录存在
        if let Some(parent) = Path::new(&final_output_path).parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        // 写入文件
        let mut file = File::create(&final_output_path)?;
        file.write_all(&bytes)?;

        tracing::debug!(
            "Board image downloaded successfully: {} bytes",
            file_size
        );

        Ok(GetBoardImageResponse {
            file_path: final_output_path,
            file_size,
            content_type,
            file_extension: file_extension.to_string(),
        })
    }

    /// 下载画板图片到指定路径
    /// 如果未指定文件扩展名，将自动根据 Content-Type 添加
    pub async fn get_board_image_auto_name(
        &self,
        whiteboard_id: &str,
        output_dir: &str,
    ) -> Result<GetBoardImageResponse> {
        let path = Path::new(output_dir);
        let filename = format!("{}.{}", whiteboard_id, "png");
        let output_path = path.join(filename).to_str().unwrap().to_string();
        self.get_board_image(whiteboard_id, &output_path).await
    }
}
