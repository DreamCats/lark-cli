use super::ApiClient;
use crate::error::Result;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// 下载媒体文件的请求参数
#[derive(Debug, Serialize)]
pub struct DownloadMediaRequest {
    /// 文件 token
    pub file_token: String,
    /// 额外扩展信息（用于高级权限的多维表格）
    pub extra: Option<String>,
    /// 分片下载范围（可选），格式 "bytes=start-end"
    pub range: Option<String>,
}

/// 下载媒体文件的响应
#[derive(Debug, Serialize)]
pub struct DownloadMediaResponse {
    /// 保存的文件路径
    pub file_path: String,
    /// 文件大小（字节）
    pub file_size: u64,
    /// MIME 类型
    pub content_type: String,
    /// 文件名（从响应头获取）
    pub file_name: Option<String>,
}

pub struct DownloadMediaApi {
    client: ApiClient,
}

impl DownloadMediaApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    /// 下载媒体文件并保存到本地
    pub async fn download_media(
        &self,
        file_token: &str,
        output_path: &str,
        extra: Option<String>,
        range: Option<String>,
    ) -> Result<DownloadMediaResponse> {
        // 验证参数
        if file_token.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "file_token 参数是必需的".to_string(),
            ));
        }

        // 构建请求 URL
        let url = format!(
            "https://open.larkoffice.com/open-apis/drive/v1/medias/{}/download",
            file_token
        );

        // 构建查询参数
        let mut params = HashMap::new();
        if let Some(ref extra) = extra {
            params.insert("extra".to_string(), extra.clone());
        }

        // 获取认证头
        let auth_header = self.client.auth_manager.get_auth_header().await?;

        // 构建请求
        let mut request = self.client.client.get(&url);

        // 添加认证头
        request = request.header("Authorization", auth_header);

        // 添加 Range 头（分片下载）
        if let Some(ref range_value) = range {
            request = request.header("Range", range_value);
        }

        // 添加查询参数
        if !params.is_empty() {
            request = request.query(&params);
        }

        tracing::debug!("Sending GET request to: {:?}", url);

        // 发送请求
        let response = request.send().await?;

        let status = response.status();
        tracing::debug!("Response status: {}", status);

        // 检查 HTTP 状态码
        match status.as_u16() {
            200 | 206 => {
                // 成功或部分内容
            }
            400 => {
                return Err(crate::error::LarkError::ApiError {
                    code: 400,
                    message: "请求参数错误，对于开启了高级权限的多维表格，需确保已正确添加额外的扩展信息".to_string(),
                });
            }
            403 => {
                return Err(crate::error::LarkError::ApiError {
                    code: 403,
                    message: "没有下载素材的权限，请确保调用身份拥有文档资源权限".to_string(),
                });
            }
            404 => {
                return Err(crate::error::LarkError::ApiError {
                    code: 404,
                    message: "素材 token 不存在或素材被删除".to_string(),
                });
            }
            500 => {
                return Err(crate::error::LarkError::ApiError {
                    code: 500,
                    message: "服务端内部异常，请重试".to_string(),
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
            .unwrap_or("application/octet-stream")
            .to_string();

        // 获取文件名（从 Content-Disposition 头）
        let file_name = response
            .headers()
            .get("content-disposition")
            .and_then(|v| v.to_str().ok())
            .and_then(|disposition| {
                // 解析 filename="..." 或 filename=...
                if let Some(start) = disposition.find("filename=") {
                    let rest = &disposition[start + 9..];
                    let filename = if rest.starts_with('"') {
                        // 带引号的文件名
                        rest.chars()
                            .skip(1)
                            .take_while(|c| *c != '"')
                            .collect::<String>()
                    } else {
                        // 不带引号的文件名，取到空格或分号
                        rest.chars()
                            .take_while(|c| *c != ' ' && *c != ';')
                            .collect::<String>()
                    };
                    Some(filename)
                } else {
                    None
                }
            });

        // 获取二进制内容
        let bytes = response.bytes().await?;
        let file_size = bytes.len() as u64;

        // 确保输出目录存在
        if let Some(parent) = Path::new(output_path).parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        // 写入文件
        let mut file = File::create(output_path)?;
        file.write_all(&bytes)?;

        tracing::debug!(
            "File downloaded successfully: {} bytes",
            file_size
        );

        Ok(DownloadMediaResponse {
            file_path: output_path.to_string(),
            file_size,
            content_type,
            file_name,
        })
    }

    /// 下载媒体文件到指定路径
    /// 如果未指定文件名，将使用 file_token 作为文件名
    pub async fn download_media_auto_name(
        &self,
        file_token: &str,
        output_dir: &str,
        extra: Option<String>,
        range: Option<String>,
    ) -> Result<DownloadMediaResponse> {
        // 先发起请求获取文件名
        let url = format!(
            "https://open.larkoffice.com/open-apis/drive/v1/medias/{}/download",
            file_token
        );

        let mut params = HashMap::new();
        if let Some(ref extra) = extra {
            params.insert("extra".to_string(), extra.clone());
        }

        let auth_header = self.client.auth_manager.get_auth_header().await?;

        let mut request = self.client.client.head(&url);
        request = request.header("Authorization", auth_header);

        if !params.is_empty() {
            request = request.query(&params);
        }

        // 尝试 HEAD 请求获取文件名
        let file_name = match request.send().await {
            Ok(resp) => resp
                .headers()
                .get("content-disposition")
                .and_then(|v| v.to_str().ok())
                .and_then(|disposition| {
                    if let Some(start) = disposition.find("filename=") {
                        let rest = &disposition[start + 9..];
                        let filename = if rest.starts_with('"') {
                            rest.chars()
                                .skip(1)
                                .take_while(|c| *c != '"')
                                .collect::<String>()
                        } else {
                            rest.chars()
                                .take_while(|c| *c != ' ' && *c != ';')
                                .collect::<String>()
                        };
                        Some(filename)
                    } else {
                        None
                    }
                }),
            Err(_) => None,
        };

        // 确定输出文件路径
        let output_path = if let Some(name) = file_name {
            let path = Path::new(output_dir);
            path.join(name).to_str().unwrap().to_string()
        } else {
            let path = Path::new(output_dir);
            path.join(file_token).to_str().unwrap().to_string()
        };

        self.download_media(file_token, &output_path, extra, range)
            .await
    }
}
