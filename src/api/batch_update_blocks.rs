use super::ApiClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct BatchUpdateBlocksRequest {
    pub requests: Vec<UpdateBlockRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBlockRequest {
    #[serde(rename = "block_id")]
    pub block_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_text_elements: Option<UpdateTextElementsRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_text_style: Option<UpdateTextStyleRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_table_property: Option<UpdateTablePropertyRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_table_row: Option<InsertTableRowRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_table_column: Option<InsertTableColumnRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_table_rows: Option<DeleteTableRowsRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_table_columns: Option<DeleteTableColumnsRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_table_cells: Option<MergeTableCellsRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmerge_table_cells: Option<UnmergeTableCellsRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_grid_column: Option<InsertGridColumnRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_grid_column: Option<DeleteGridColumnRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_grid_column_width_ratio: Option<UpdateGridColumnWidthRatioRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_image: Option<ReplaceImageRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_file: Option<ReplaceFileRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_text: Option<UpdateTextRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_task: Option<UpdateTaskRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTextElementsRequest {
    pub elements: Vec<TextElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTextStyleRequest {
    pub style: TextStyle,
    pub fields: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTablePropertyRequest {
    pub column_width: i32,
    pub column_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_row: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_column: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertTableRowRequest {
    pub row_index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertTableColumnRequest {
    pub column_index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteTableRowsRequest {
    pub row_start_index: i32,
    pub row_end_index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteTableColumnsRequest {
    pub column_start_index: i32,
    pub column_end_index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeTableCellsRequest {
    pub row_start_index: i32,
    pub row_end_index: i32,
    pub column_start_index: i32,
    pub column_end_index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnmergeTableCellsRequest {
    pub row_index: i32,
    pub column_index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertGridColumnRequest {
    pub column_index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteGridColumnRequest {
    pub column_index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGridColumnWidthRatioRequest {
    pub width_ratios: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplaceImageRequest {
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Caption>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplaceFileRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTextRequest {
    pub elements: Vec<TextElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folded: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_run: Option<TextRun>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_user: Option<MentionUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_doc: Option<MentionDoc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder: Option<Reminder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<InlineFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equation: Option<Equation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undefined: Option<UndefinedElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_block: Option<InlineBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextRun {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_element_style: Option<TextElementStyle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MentionUser {
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_element_style: Option<TextElementStyle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MentionDoc {
    pub token: String,
    pub obj_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_element_style: Option<TextElementStyle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reminder {
    pub create_user_id: String,
    pub expire_time: String,
    pub notify_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_notify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_whole_day: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_element_style: Option<TextElementStyle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineFile {
    pub file_token: String,
    pub source_block_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_element_style: Option<TextElementStyle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Equation {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_element_style: Option<TextElementStyle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UndefinedElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_element_style: Option<TextElementStyle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineBlock {
    pub block_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_element_style: Option<TextElementStyle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextElementStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_code: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_ids: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indentation_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Caption {
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BatchUpdateBlocksResponse {
    #[serde(rename = "blocks")]
    pub blocks: Vec<serde_json::Value>,
    #[serde(rename = "client_token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "document_revision_id")]
    pub document_revision_id: i32,
}

#[allow(dead_code)]
impl UpdateBlockRequest {
    pub fn new(block_id: String) -> Self {
        Self {
            block_id,
            update_text_elements: None,
            update_text_style: None,
            update_table_property: None,
            insert_table_row: None,
            insert_table_column: None,
            delete_table_rows: None,
            delete_table_columns: None,
            merge_table_cells: None,
            unmerge_table_cells: None,
            insert_grid_column: None,
            delete_grid_column: None,
            update_grid_column_width_ratio: None,
            replace_image: None,
            replace_file: None,
            update_text: None,
            update_task: None,
        }
    }

    // 便捷方法：更新文本元素
    pub fn with_text_elements(mut self, elements: Vec<TextElement>) -> Self {
        self.update_text_elements = Some(UpdateTextElementsRequest { elements });
        self
    }

    // 便捷方法：更新文本样式
    pub fn with_text_style(mut self, style: TextStyle, fields: Vec<i32>) -> Self {
        self.update_text_style = Some(UpdateTextStyleRequest { style, fields });
        self
    }

    // 便捷方法：替换图片
    pub fn with_replace_image(mut self, token: String, width: Option<i32>, height: Option<i32>, align: Option<i32>) -> Self {
        self.replace_image = Some(ReplaceImageRequest {
            token,
            width,
            height,
            align,
            caption: None,
        });
        self
    }

    // 便捷方法：更新文本（简化版）
    pub fn with_update_text(mut self, elements: Vec<TextElement>) -> Self {
        self.update_text = Some(UpdateTextRequest { elements });
        self
    }
}

#[allow(dead_code)]
impl TextElement {
    pub fn text_run(content: String, style: Option<TextElementStyle>) -> Self {
        Self {
            text_run: Some(TextRun { content, text_element_style: style }),
            mention_user: None,
            mention_doc: None,
            reminder: None,
            file: None,
            equation: None,
            undefined: None,
            inline_block: None,
        }
    }

    pub fn mention_user(user_id: String, style: Option<TextElementStyle>) -> Self {
        Self {
            text_run: None,
            mention_user: Some(MentionUser { user_id, text_element_style: style }),
            mention_doc: None,
            reminder: None,
            file: None,
            equation: None,
            undefined: None,
            inline_block: None,
        }
    }

    pub fn mention_doc(token: String, obj_type: i32, url: Option<String>, title: Option<String>) -> Self {
        Self {
            text_run: None,
            mention_user: None,
            mention_doc: Some(MentionDoc { token, obj_type, url, title, text_element_style: None }),
            reminder: None,
            file: None,
            equation: None,
            undefined: None,
            inline_block: None,
        }
    }
}

#[allow(dead_code)]
impl TextElementStyle {
    pub fn simple() -> Self {
        Self {
            bold: None,
            italic: None,
            strikethrough: None,
            underline: None,
            inline_code: None,
            background_color: None,
            text_color: None,
            link: None,
            comment_ids: None,
        }
    }

    pub fn with_bold(mut self) -> Self {
        self.bold = Some(true);
        self
    }

    pub fn with_italic(mut self) -> Self {
        self.italic = Some(true);
        self
    }

    pub fn with_color(mut self, color: i32) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn with_background_color(mut self, color: i32) -> Self {
        self.background_color = Some(color);
        self
    }
}

#[allow(dead_code)]
impl TextStyle {
    pub fn simple() -> Self {
        Self {
            align: None,
            done: None,
            folded: None,
            language: None,
            wrap: None,
            background_color: None,
            indentation_level: None,
            sequence: None,
        }
    }

    pub fn with_align(mut self, align: i32) -> Self {
        self.align = Some(align);
        self
    }

    pub fn with_done(mut self, done: bool) -> Self {
        self.done = Some(done);
        self
    }
}

pub struct BatchUpdateBlocksApi {
    client: ApiClient,
}

impl BatchUpdateBlocksApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    pub async fn batch_update_blocks(
        &self,
        document_id: &str,
        requests: Vec<UpdateBlockRequest>,
        document_revision_id: Option<i32>,
        client_token: Option<String>,
        user_id_type: Option<String>,
    ) -> Result<BatchUpdateBlocksResponse> {
        // 验证参数
        if document_id.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "document_id 参数是必需的".to_string()
            ));
        }

        if requests.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "requests 参数是必需的，且不能为空列表".to_string()
            ));
        }

        let user_id_type = user_id_type.unwrap_or_else(|| "open_id".to_string());
        let valid_user_id_types = ["open_id", "union_id", "user_id"];
        if !valid_user_id_types.contains(&user_id_type.as_str()) {
            return Err(crate::error::LarkError::ValidationError(
                format!("user_id_type 必须是以下值之一：{}", valid_user_id_types.join(", "))
            ));
        }

        // 构建请求体
        let request = BatchUpdateBlocksRequest { requests };

        // 构建查询参数
        let mut params = HashMap::new();
        params.insert("document_revision_id".to_string(), document_revision_id.unwrap_or(-1).to_string());
        params.insert("user_id_type".to_string(), user_id_type.clone());
        if let Some(token) = &client_token {
            params.insert("client_token".to_string(), token.clone());
        }

        // 构建URL
        let url = format!(
            "https://open.larkoffice.com/open-apis/docx/v1/documents/{}/blocks/batch_update",
            document_id
        );

        // 发送请求
        let data: BatchUpdateBlocksResponse = self.client
            .patch_with_params(&url, Some(params), &request)
            .await?;

        Ok(data)
    }
}