use crate::api::{ApiClient, CreateNestedBlocksApi};
use crate::error::Result;
use crate::output::{OutputFormat, format_output};
use crate::api::create_nested_blocks::DescendantBlock;
use serde_json::json;

#[derive(Debug, Clone, Copy)]
pub enum CalloutType {
    Info,
    Warning,
    Error,
    Success,
}

impl CalloutType {
    fn to_colors(&self) -> (i32, i32, &'static str) {
        match self {
            CalloutType::Info => (
                2,  // Light橙色
                2,  // 橙色
                "sparkles"
            ),
            CalloutType::Warning => (
                3,  // LightYellow
                3,  // Yellow
                "o"
            ),
            CalloutType::Error => (
                1,  // LightRed
                1,  // Red
                "x"
            ),
            CalloutType::Success => (
                4,  // LightGreen
                4,  // Green
                "white_check_mark"
            ),
        }
    }
}

impl std::str::FromStr for CalloutType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "info" => Ok(CalloutType::Info),
            "warning" | "warn" => Ok(CalloutType::Warning),
            "error" | "danger" => Ok(CalloutType::Error),
            "success" | "good" => Ok(CalloutType::Success),
            _ => Err(format!("无效的高亮块类型: {}。可选值: info, warning, error, success", s)),
        }
    }
}

pub async fn handle_add_callout(
    api_client: ApiClient,
    document_id: String,
    content: String,
    parent_id: Option<String>,
    index: Option<i32>,
    callout_type: CalloutType,
    icon: Option<String>,
    output_format: OutputFormat,
) -> Result<()> {
    let create_blocks_api = CreateNestedBlocksApi::new(api_client);

    // 生成临时块ID
    let temp_callout_id = format!("temp-callout-{}", uuid::Uuid::new_v4());
    let temp_text_id = format!("temp-text-{}", uuid::Uuid::new_v4());

    // 根据类型获取颜色配置
    let (bg_color, border_color, default_icon) = callout_type.to_colors();

    // 确定父块ID - 如果用户没有提供，则使用文档ID作为顶级容器
    let effective_parent_id = parent_id.as_deref()
        .filter(|id| !id.is_empty())
        .unwrap_or(&document_id);

    // 构建高亮块（父块）
    let callout_block = DescendantBlock {
        block_id: temp_callout_id.clone(),
        block_type: 19, // BlockType::Callout 的值
        parent_id: Some(effective_parent_id.to_string()),
        children: Some(vec![temp_text_id.clone()]), // 包含文本子块
        callout: Some(json!({
            "background_color": bg_color as i32,
            "border_color": border_color as i32,
            "emoji_id": icon.unwrap_or_else(|| default_icon.to_string())
        })),
        ..Default::default()
    };

    // 构建文本子块
    let text_block = DescendantBlock {
        block_id: temp_text_id.clone(),
        block_type: 2, // BlockType::Text 的值
        parent_id: Some(temp_callout_id.clone()), // 父块是高亮块
        children: Some(vec![]),
        text: Some(json!({
            "elements": [{
                "text_run": {
                    "content": content
                }
            }]
        })),
        ..Default::default()
    };

    // 构建请求数据
    let children = vec![temp_callout_id.clone()];
    let descendants = vec![callout_block, text_block];

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