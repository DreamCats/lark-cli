use crate::api::{ApiClient, ImportDocumentsApi};
use crate::error::{Result, LarkError};
use crate::output::OutputFormat;
use crate::output::format_output;
use crate::utils::{FileScanner, FileReader};
use std::path::Path;

#[allow(clippy::too_many_arguments)]
pub async fn handle_add_content(
    api_client: ApiClient,
    document_id: String,
    source: String,
    source_type: String,
    content_type: String,
    block_id: String,
    index: i32,
    recursive: bool,
    pattern: Option<String>,
    batch_size: usize,
    _skip_existing: bool,
    verbose: bool,
    output_format: OutputFormat,
) -> Result<()> {
    let import_api = ImportDocumentsApi::new(api_client);

    match source_type.as_str() {
        "content" => {
            // 直接导入内容
            let result = import_api.import_document(
                &document_id,
                &source,
                &content_type,
                &block_id,
                index,
                None,
                None,
            ).await?;
            let output = format_output(&result, output_format)?;
            println!("{}", output);
        }
        "file" => {
            // 导入单个文件
            let file_path = Path::new(&source);
            let content = FileReader::read_to_string(file_path)?;
            let inferred_type = FileReader::infer_content_type(file_path);

            let result = import_api.import_document(
                &document_id,
                &content,
                inferred_type,
                &block_id,
                index,
                None,
                None,
            ).await?;
            let output = format_output(&result, output_format)?;
            println!("{}", output);
        }
        "dir" => {
            // 批量导入目录
            let dir_path = Path::new(&source);
            let scanner = FileScanner::new(pattern.as_deref())?;

            // 扫描文件
            let files = scanner.scan_directory(dir_path, recursive)?;
            if files.is_empty() {
                println!("未找到匹配的文件");
                return Ok(());
            }

            println!("找到 {} 个文件，开始导入...", files.len());

            // 准备导入请求
            let mut import_requests = Vec::new();
            let mut failed_count = 0;
            for file_path in files {
                let content = match FileReader::read_to_string(&file_path) {
                    Ok(c) => c,
                    Err(e) => {
                        failed_count += 1;
                        eprintln!("读取文件失败 {}: {}", file_path.display(), e);
                        continue;
                    }
                };

                let file_type = FileReader::infer_content_type(&file_path);

                import_requests.push(crate::utils::ImportRequest {
                    file_path: file_path.clone(),
                    content,
                    content_type: file_type.to_string(),
                    block_id: block_id.clone(),
                    index,
                    relative_path: None,
                });
            }

            if import_requests.is_empty() {
                println!("没有成功读取的文件可以导入");
                return Ok(());
            }

            // 执行批量导入
            let result = import_api.import_batch(
                &document_id,
                import_requests,
                batch_size,
                None,
            ).await?;

            // 显示详细结果（如果启用了详细模式）
            if verbose {
                // 显示详细结果
                for import_result in &result.results {
                    let file_name = import_result.file_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("?");

                    if import_result.success {
                        println!("  ✓ {}", file_name);
                    } else {
                        println!("  ✗ {} - {}",
                            file_name,
                            import_result.error.as_deref().unwrap_or("未知错误"));
                    }
                }
            }

            // 显示统计摘要
            let total = result.results.len() + failed_count;
            println!("\n=== 导入统计 ===");
            println!("  总数:     {}", total);
            println!("  成功:     {} ({:.1}%)",
                result.success_count,
                (result.success_count as f64 / total as f64) * 100.0);
            if result.failure_count > 0 {
                println!("  导入失败: {} ({:.1}%)",
                    result.failure_count,
                    (result.failure_count as f64 / total as f64) * 100.0);
            }
            if failed_count > 0 {
                println!("  读取失败: {} ({:.1}%)",
                    failed_count,
                    (failed_count as f64 / total as f64) * 100.0);
            }

            // 输出结果
            let output = format_output(&result, output_format)?;
            println!("{}", output);
        }
        _ => {
            return Err(LarkError::ValidationError(
                "不支持的 source_type，支持的类型：file, dir, content".to_string()
            ));
        }
    }
    Ok(())
}