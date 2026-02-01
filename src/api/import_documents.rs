use super::{ApiClient, ConvertBlocksApi, CreateNestedBlocksApi, DescendantBlock};
use crate::api::convert_blocks::ConvertBlocksResponse;
use crate::api::create_nested_blocks::CreateNestedBlocksResponse;
use crate::error::Result;
use crate::utils::{ImportRequest, ImportResult, BatchImportResult};
use super::block_converter::BlockConverter;

/// 文档导入 API，封装了转换和创建两个步骤
#[derive(Clone)]
pub struct ImportDocumentsApi {
    client: ApiClient,
}

impl ImportDocumentsApi {
    /// 创建新的文档导入 API 实例
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    /// 导入单个文档内容
    #[allow(clippy::too_many_arguments)]
    pub async fn import_document(
        &self,
        document_id: &str,
        content: &str,
        content_type: &str,
        block_id: &str,
        index: i32,
        document_revision_id: Option<i32>,
        client_token: Option<String>,
    ) -> Result<CreateNestedBlocksResponse> {
        // 1. 调用 convert_blocks 转换内容
        let convert_api = ConvertBlocksApi::new(self.client.clone());
        let convert_result = convert_api
            .convert_content_to_blocks(content, content_type)
            .await?;

        // 调试：打印 convert_blocks 的结果
        if std::env::var("DEBUG_CONVERT").is_ok() {
            eprintln!("ConvertBlocks Response: {}", serde_json::to_string_pretty(&convert_result)?);
        }

        // 2. 转换数据格式
        let (children_id, descendants) = Self::convert_response_to_descendants(convert_result)?;

        // 调试：打印转换结果
        if std::env::var("DEBUG_CONVERT").is_ok() {
            eprintln!("转换后的 descendants 数量: {}", descendants.len());
            eprintln!("children_id 数量: {}", children_id.len());
        }

        // 3. 检查块数量，如果超过1000需要分批处理
        const MAX_BLOCKS_PER_REQUEST: usize = 1000;

        if descendants.len() > MAX_BLOCKS_PER_REQUEST {
            // 分批处理
            let mut all_block_relations = Vec::new();
            let mut current_index = index;

            // 调试：打印分批信息
            if std::env::var("DEBUG_CREATE").is_ok() {
                eprintln!("分批处理：总块数 {}，分成 {} 批", descendants.len(),
                    descendants.len().div_ceil(MAX_BLOCKS_PER_REQUEST));
            }

            for (i, chunk) in descendants.chunks(MAX_BLOCKS_PER_REQUEST).enumerate() {
                let chunk_children_id: Vec<String> = chunk
                    .iter()
                    .filter(|block| children_id.contains(&block.block_id))
                    .map(|block| block.block_id.clone())
                    .collect();

                // 调试：打印每批的信息
                if std::env::var("DEBUG_CREATE").is_ok() {
                    eprintln!("第 {} 批：块数 {}，索引 {}", i + 1, chunk.len(), current_index);
                    eprintln!("chunk_children_id: {:?}", chunk_children_id);
                }

                let create_api = CreateNestedBlocksApi::new(self.client.clone());
                let result = create_api
                    .create_nested_blocks(
                        document_id,
                        block_id,
                        chunk_children_id,
                        chunk.to_vec(),
                        Some(current_index),
                        document_revision_id,
                        client_token.clone(),
                    )
                    .await?;

                all_block_relations.extend(result.block_id_relations);
                current_index += chunk.len() as i32;
            }

            // 返回合并的结果
            Ok(CreateNestedBlocksResponse {
                block_id_relations: all_block_relations,
                children: vec![],
                client_token,
                document_revision_id: document_revision_id.unwrap_or(0),
            })
        } else {
            // 3. 调用 create_nested_blocks 创建块
            let create_api = CreateNestedBlocksApi::new(self.client.clone());

            // 调试：打印请求数据
            if std::env::var("DEBUG_CREATE").is_ok() {
                eprintln!("CreateNestedBlocks Request:");
                eprintln!("  document_id: {}", document_id);
                eprintln!("  block_id: {}", block_id);
                eprintln!("  index: {:?}", Some(index));
                eprintln!("  children_id: {:?}", children_id);
                eprintln!("  descendants count: {}", descendants.len());
                eprintln!("  descendants: {}", serde_json::to_string_pretty(&descendants)?);
            }

            let result = create_api
                .create_nested_blocks(
                    document_id,
                    block_id,
                    children_id,
                    descendants,
                    Some(index),
                    document_revision_id,
                    client_token,
                )
                .await?;

            Ok(result)
        }
    }

    /// 批量导入文档
    pub async fn import_batch(
        &self,
        document_id: &str,
        requests: Vec<ImportRequest>,
        _batch_size: usize,
        document_revision_id: Option<i32>,
    ) -> Result<BatchImportResult> {
        let mut results = Vec::new();
        let mut success_count = 0;
        let mut failure_count = 0;

        // 串行处理每个请求
        for request in requests {
            // 处理单个文件
            let result = self.import_document(
                document_id,
                &request.content,
                &request.content_type,
                &request.block_id,
                request.index,
                document_revision_id,
                None,
            ).await;

            match result {
                Ok(_) => {
                    success_count += 1;
                    results.push(ImportResult {
                        file_path: request.file_path,
                        success: true,
                        error: None,
                        block_ids: None,
                    });
                }
                Err(e) => {
                    failure_count += 1;
                    results.push(ImportResult {
                        file_path: request.file_path,
                        success: false,
                        error: Some(format!("导入失败: {}", e)),
                        block_ids: None,
                    });
                }
            }
        }

        Ok(BatchImportResult {
            success_count,
            failure_count,
            skipped_count: 0,
            results,
        })
    }


    /// 将 convert_blocks 的响应转换为 create_nested_blocks 的请求格式
    fn convert_response_to_descendants(
        convert_response: ConvertBlocksResponse,
    ) -> Result<(Vec<String>, Vec<DescendantBlock>)> {
        // 使用 BlockConverter 进行转换
        BlockConverter::convert_response(convert_response)
    }
}