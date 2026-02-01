use crate::api::{ApiClient, FileApi, MediaApi};
use crate::error::Result;
use crate::output::OutputFormat;
use crate::output::format_output;
use base64::{Engine as _, engine::general_purpose};

pub async fn handle_read_file(
    api_client: ApiClient,
    file_path: String,
    output_format: OutputFormat,
) -> Result<()> {
    let file_api = FileApi::new(api_client);
    let result = file_api.read_file(&file_path).await?;
    let output = format_output(&result, output_format)?;
    println!("{}", output);
    Ok(())
}

pub async fn handle_write_file(
    api_client: ApiClient,
    file_path: String,
    content: String,
    overwrite: bool,
    output_format: OutputFormat,
) -> Result<()> {
    let file_api = FileApi::new(api_client);

    // 解码Base64内容
    let decoded_content = general_purpose::STANDARD.decode(&content)
        .map_err(|e| crate::error::LarkError::ParseError(format!("Base64解码失败: {}", e)))?;

    let result = file_api.write_file(&file_path, decoded_content, overwrite).await?;
    let output = format_output(&result, output_format)?;
    println!("{}", output);
    Ok(())
}

pub async fn handle_upload_media(
    api_client: ApiClient,
    file_path: String,
    parent_type: String,
    parent_node: String,
    checksum: Option<String>,
    extra: Option<String>,
    output_format: OutputFormat,
) -> Result<()> {
    let media_api = MediaApi::new(api_client);
    let result = media_api.upload_media_from_file(&file_path, &parent_type, &parent_node, checksum, extra).await?;
    let output = format_output(&result, output_format)?;
    println!("{}", output);
    Ok(())
}