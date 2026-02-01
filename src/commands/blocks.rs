use crate::api::{
    ApiClient, ConvertBlocksApi, GetBlocksApi,
    BatchUpdateBlocksApi, DeleteBlocksApi
};
use crate::error::{Result, LarkError};
use crate::output::OutputFormat;
use crate::output::format_output;
use serde_json;

#[allow(dead_code)]
pub async fn handle_convert_blocks(
    api_client: ApiClient,
    content: String,
    content_type: String,
    output_format: OutputFormat,
) -> Result<()> {
    let convert_api = ConvertBlocksApi::new(api_client);
    let result = convert_api.convert_content_to_blocks(&content, &content_type).await?;
    let output = format_output(&result, output_format)?;
    println!("{}", output);
    Ok(())
}


#[allow(clippy::too_many_arguments)]
pub async fn handle_get_blocks(
    api_client: ApiClient,
    document_id: String,
    page_size: i32,
    page_token: Option<String>,
    document_revision_id: Option<i32>,
    user_id_type: String,
    all: bool,
    output_format: OutputFormat,
) -> Result<()> {
    let blocks_api = GetBlocksApi::new(api_client);

    let result = if all {
        // 获取所有块（自动处理分页）
        let blocks = blocks_api.get_all_document_blocks(
            &document_id,
            document_revision_id,
            Some(user_id_type),
        ).await?;

        // 构建响应结构
        serde_json::json!({
            "items": blocks,
            "total_count": blocks.len(),
            "has_more": false,
            "page_token": null
        })
    } else {
        // 获取单页块
        let response = blocks_api.get_document_blocks(
            &document_id,
            Some(page_size),
            page_token,
            document_revision_id,
            Some(user_id_type),
        ).await?;

        serde_json::json!({
            "items": response.items,
            "total_count": response.items.len(),
            "has_more": response.has_more,
            "page_token": response.page_token
        })
    };

    let output = format_output(&result, output_format)?;
    println!("{}", output);
    Ok(())
}

pub async fn handle_batch_update_blocks(
    api_client: ApiClient,
    document_id: String,
    requests: String,
    document_revision_id: Option<i32>,
    client_token: Option<String>,
    user_id_type: String,
    output_format: OutputFormat,
) -> Result<()> {
    let batch_update_api = BatchUpdateBlocksApi::new(api_client);

    // 解析 JSON 参数
    let requests_list: Vec<crate::api::batch_update_blocks::UpdateBlockRequest> = serde_json::from_str(&requests)
        .map_err(|e| LarkError::ParseError(format!("requests JSON 解析失败: {}", e)))?;

    let result = batch_update_api.batch_update_blocks(
        &document_id,
        requests_list,
        document_revision_id,
        client_token,
        Some(user_id_type)
    ).await?;

    let output = format_output(&result, output_format)?;
    println!("{}", output);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn handle_delete_blocks(
    api_client: ApiClient,
    document_id: String,
    block_id: String,
    start_index: i32,
    end_index: i32,
    document_revision_id: Option<i32>,
    client_token: Option<String>,
    output_format: OutputFormat,
) -> Result<()> {
    let delete_blocks_api = DeleteBlocksApi::new(api_client);

    let result = delete_blocks_api.delete_blocks(
        &document_id,
        &block_id,
        start_index,
        end_index,
        document_revision_id,
        client_token
    ).await?;

    let output = format_output(&result, output_format)?;
    println!("{}", output);
    Ok(())
}