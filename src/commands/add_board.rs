use serde_json::json;

use crate::api::{ApiClient, CreateNestedBlocksApi};
use crate::error::Result;
use crate::output::{OutputFormat, format_output};
use crate::api::create_nested_blocks::DescendantBlock;

#[derive(Debug, Clone, Copy)]
pub enum BoardAlign {
    Left,
    Center,
    Right,
}

impl BoardAlign {
}

impl std::str::FromStr for BoardAlign {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "left" | "左对齐" | "1" => Ok(BoardAlign::Left),
            "center" | "居中" | "2" => Ok(BoardAlign::Center),
            "right" | "右对齐" | "3" => Ok(BoardAlign::Right),
            _ => Err(format!("无效的对齐方式: {}。可选值: left, center, right", s)),
        }
    }
}

pub async fn handle_add_board(
    api_client: ApiClient,
    document_id: String,
    parent_id: Option<String>,
    index: Option<i32>,
    output_format: OutputFormat,
) -> Result<()> {
    let create_blocks_api = CreateNestedBlocksApi::new(api_client);

    // 生成临时块ID
    let temp_board_id = format!("temp-board-{}", uuid::Uuid::new_v4());

    // 确定父块ID - 如果用户没有提供，则使用文档ID作为顶级容器
    let effective_parent_id = parent_id.as_deref()
        .filter(|id| !id.is_empty())
        .unwrap_or(&document_id);

    let board_block = DescendantBlock {
        block_id: temp_board_id.clone(),
        block_type: 43, // BlockType::Board 的值
        parent_id: Some(effective_parent_id.to_string()),
        children: Some(vec![]), // 画板块不支持子块
        // todo: 补充画板属性，可以为空
        board: Some(json!({
            "align": 2,// 默认居中对齐
        })),
        ..Default::default()
    };

    // 构建请求数据
    let children = vec![temp_board_id.clone()];
    let descendants = vec![board_block];

    // 调用API创建块
    let result = create_blocks_api.create_nested_blocks(
        &document_id,
        effective_parent_id,
        children,
        descendants,
        index, // 使用传入的index
        None, // document_revision_id
        None, // client_token
    ).await?;

    // 格式化输出
    let output = format_output(&result, output_format)?;
    println!("{}", output);

    Ok(())
}