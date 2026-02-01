use crate::api::{convert_blocks::ConvertBlocksResponse, create_nested_blocks::DescendantBlock};
use crate::error::Result;
use serde_json;
use std::collections::HashSet;

/// 块转换器，负责将ConvertBlock转换为DescendantBlock
pub struct BlockConverter;

impl BlockConverter {
    /// 过滤表格单元格块（类型32）
    pub fn filter_table_cell_blocks(response: &ConvertBlocksResponse) -> HashSet<String> {
        response
            .blocks
            .iter()
            .filter(|block| block.block_type == 32)
            .map(|block| block.block_id.clone())
            .collect()
    }

    /// 过滤一级块ID，移除表格单元格块
    pub fn filter_first_level_block_ids(
        first_level_block_ids: Vec<String>,
        table_cell_block_ids: &HashSet<String>,
    ) -> Vec<String> {
        first_level_block_ids
            .into_iter()
            .filter(|id| !table_cell_block_ids.contains(id))
            .collect()
    }

    /// 处理表格块内容，移除merge_info字段
    pub fn process_table_block_content(content: &mut serde_json::Value) {
        if let Some(table_content) = content.get_mut("table") {
            if let Some(property) = table_content.get_mut("property") {
                if let Some(prop_obj) = property.as_object_mut() {
                    prop_obj.remove("merge_info");
                }
            }
        }
    }

    /// 将单个ConvertBlock转换为DescendantBlock
    pub fn convert_block(mut block: crate::api::convert_blocks::Block) -> DescendantBlock {
        // 处理表格块
        if block.block_type == 31 {
            Self::process_table_block_content(&mut block.content);
        }

        let mut descendant = DescendantBlock {
            block_id: block.block_id.clone(),
            block_type: block.block_type,
            parent_id: if block.parent_id.is_empty() { None } else { Some(block.parent_id.clone()) },
            children: block.children.clone(),
            ..Default::default()
        };

        // 根据block_type设置对应的内容字段
        let content_found = Self::set_content_by_type(&mut descendant, &block);

        // 如果没有找到内容，创建默认内容
        if !content_found {
            Self::set_default_content(&mut descendant);
        }

        descendant
    }

    /// 根据块类型设置内容字段
    fn set_content_by_type(
        descendant: &mut DescendantBlock,
        block: &crate::api::convert_blocks::Block,
    ) -> bool {
        match block.block_type {
            2 => Self::set_text_content(descendant, block),
            3 => Self::set_heading_content(descendant, block, "heading1"),
            4 => Self::set_heading_content(descendant, block, "heading2"),
            5 => Self::set_heading_content(descendant, block, "heading3"),
            6 => Self::set_heading_content(descendant, block, "heading4"),
            7 => Self::set_heading_content(descendant, block, "heading5"),
            8 => Self::set_heading_content(descendant, block, "heading6"),
            9 => Self::set_heading_content(descendant, block, "heading7"),
            10 => Self::set_heading_content(descendant, block, "heading8"),
            11 => Self::set_heading_content(descendant, block, "heading9"),
            12 => Self::set_list_content(descendant, block, "bullet"),
            13 => Self::set_list_content(descendant, block, "ordered"),
            14 => Self::set_code_content(descendant, block),
            15 => Self::set_quote_content(descendant, block),
            17 => Self::set_todo_content(descendant, block),
            19 => Self::set_callout_content(descendant, block),
            22 => Self::set_divider_content(descendant),
            24 => Self::set_grid_content(descendant, block),
            25 => Self::set_grid_column_content(descendant, block),
            27 => Self::set_image_content(descendant, block),
            30 => Self::set_sheet_content(descendant, block),
            31 => Self::set_table_content(descendant, block),
            32 => Self::set_table_cell_content(descendant, block),
            35 => Self::set_task_content(descendant, block),
            41 => Self::set_jira_content(descendant, block),
            _ => Self::set_generic_content(descendant, block),
        }
    }

    // 以下是各种内容设置方法
    fn set_text_content(
        descendant: &mut DescendantBlock,
        block: &crate::api::convert_blocks::Block,
    ) -> bool {
        if let Some(text_content) = block.content.get("text") {
            descendant.text = Some(text_content.clone());
            true
        } else {
            false
        }
    }

    fn set_heading_content(
        descendant: &mut DescendantBlock,
        block: &crate::api::convert_blocks::Block,
        field_name: &str,
    ) -> bool {
        if let Some(heading_content) = block.content.get(field_name) {
            match field_name {
                "heading1" => descendant.heading1 = Some(heading_content.clone()),
                "heading2" => descendant.heading2 = Some(heading_content.clone()),
                "heading3" => descendant.heading3 = Some(heading_content.clone()),
                "heading4" => descendant.heading4 = Some(heading_content.clone()),
                "heading5" => descendant.heading5 = Some(heading_content.clone()),
                "heading6" => descendant.heading6 = Some(heading_content.clone()),
                "heading7" => descendant.heading7 = Some(heading_content.clone()),
                "heading8" => descendant.heading8 = Some(heading_content.clone()),
                "heading9" => descendant.heading9 = Some(heading_content.clone()),
                _ => {}
            }
            true
        } else {
            false
        }
    }

    fn set_list_content(
        descendant: &mut DescendantBlock,
        block: &crate::api::convert_blocks::Block,
        field_name: &str,
    ) -> bool {
        if let Some(list_content) = block.content.get(field_name) {
            match field_name {
                "bullet" => descendant.bullet = Some(list_content.clone()),
                "ordered" => descendant.ordered = Some(list_content.clone()),
                _ => {}
            }
            true
        } else {
            false
        }
    }

    fn set_code_content(
        descendant: &mut DescendantBlock,
        block: &crate::api::convert_blocks::Block,
    ) -> bool {
        if let Some(code_content) = block.content.get("code") {
            descendant.code = Some(code_content.clone());
            true
        } else {
            false
        }
    }

    fn set_quote_content(
        descendant: &mut DescendantBlock,
        block: &crate::api::convert_blocks::Block,
    ) -> bool {
        if let Some(quote_content) = block.content.get("quote") {
            descendant.quote = Some(quote_content.clone());
            true
        } else {
            false
        }
    }

    fn set_todo_content(
        descendant: &mut DescendantBlock,
        block: &crate::api::convert_blocks::Block,
    ) -> bool {
        if let Some(todo_content) = block.content.get("todo") {
            descendant.todo = Some(todo_content.clone());
            true
        } else {
            false
        }
    }

    fn set_callout_content(
        descendant: &mut DescendantBlock,
        block: &crate::api::convert_blocks::Block,
    ) -> bool {
        if let Some(callout_content) = block.content.get("callout") {
            descendant.callout = Some(callout_content.clone());
            true
        } else {
            false
        }
    }

    fn set_divider_content(descendant: &mut DescendantBlock) -> bool {
        descendant.divider = Some(serde_json::json!({}));
        true
    }

    fn set_grid_content(
        descendant: &mut DescendantBlock,
        block: &crate::api::convert_blocks::Block,
    ) -> bool {
        if let Some(grid_content) = block.content.get("grid") {
            descendant.grid = Some(grid_content.clone());
            true
        } else {
            false
        }
    }

    fn set_grid_column_content(
        descendant: &mut DescendantBlock,
        block: &crate::api::convert_blocks::Block,
    ) -> bool {
        if let Some(grid_column_content) = block.content.get("grid_column") {
            descendant.grid_column = Some(grid_column_content.clone());
            true
        } else {
            false
        }
    }

    fn set_image_content(
        descendant: &mut DescendantBlock,
        block: &crate::api::convert_blocks::Block,
    ) -> bool {
        if let Some(image_content) = block.content.get("image") {
            descendant.image = Some(image_content.clone());
            true
        } else {
            false
        }
    }

    fn set_sheet_content(
        descendant: &mut DescendantBlock,
        block: &crate::api::convert_blocks::Block,
    ) -> bool {
        if let Some(sheet_content) = block.content.get("sheet") {
            descendant.sheet = Some(sheet_content.clone());
            true
        } else {
            false
        }
    }

    fn set_table_content(
        descendant: &mut DescendantBlock,
        block: &crate::api::convert_blocks::Block,
    ) -> bool {
        if let Some(table_content) = block.content.get("table") {
            descendant.table = Some(table_content.clone());
            true
        } else {
            false
        }
    }

    fn set_table_cell_content(
        descendant: &mut DescendantBlock,
        block: &crate::api::convert_blocks::Block,
    ) -> bool {
        if let Some(table_cell_content) = block.content.get("table_cell") {
            descendant.table_cell = Some(table_cell_content.clone());
            true
        } else {
            false
        }
    }

    fn set_task_content(
        descendant: &mut DescendantBlock,
        block: &crate::api::convert_blocks::Block,
    ) -> bool {
        if let Some(task_content) = block.content.get("task") {
            descendant.task = Some(task_content.clone());
            true
        } else {
            false
        }
    }

    fn set_jira_content(
        descendant: &mut DescendantBlock,
        block: &crate::api::convert_blocks::Block,
    ) -> bool {
        if let Some(jira_issue_content) = block.content.get("jira_issue") {
            descendant.jira_issue = Some(jira_issue_content.clone());
            true
        } else {
            false
        }
    }

    fn set_generic_content(
        descendant: &mut DescendantBlock,
        block: &crate::api::convert_blocks::Block,
    ) -> bool {
        // 尝试通用的内容字段
        let content_fields = [
            "text", "heading1", "heading2", "heading3", "heading4", "heading5",
            "heading6", "heading7", "heading8", "heading9", "bullet", "ordered",
            "code", "quote", "todo", "callout", "grid", "grid_column", "image",
            "sheet", "table", "table_cell", "task", "jira_issue"
        ];

        for field in &content_fields {
            if let Some(content) = block.content.get(*field) {
                match *field {
                    "text" => descendant.text = Some(content.clone()),
                    "heading1" => descendant.heading1 = Some(content.clone()),
                    "heading2" => descendant.heading2 = Some(content.clone()),
                    "heading3" => descendant.heading3 = Some(content.clone()),
                    "heading4" => descendant.heading4 = Some(content.clone()),
                    "heading5" => descendant.heading5 = Some(content.clone()),
                    "heading6" => descendant.heading6 = Some(content.clone()),
                    "heading7" => descendant.heading7 = Some(content.clone()),
                    "heading8" => descendant.heading8 = Some(content.clone()),
                    "heading9" => descendant.heading9 = Some(content.clone()),
                    "bullet" => descendant.bullet = Some(content.clone()),
                    "ordered" => descendant.ordered = Some(content.clone()),
                    "code" => descendant.code = Some(content.clone()),
                    "quote" => descendant.quote = Some(content.clone()),
                    "todo" => descendant.todo = Some(content.clone()),
                    "callout" => descendant.callout = Some(content.clone()),
                    "grid" => descendant.grid = Some(content.clone()),
                    "grid_column" => descendant.grid_column = Some(content.clone()),
                    "image" => descendant.image = Some(content.clone()),
                    "sheet" => descendant.sheet = Some(content.clone()),
                    "table" => descendant.table = Some(content.clone()),
                    "table_cell" => descendant.table_cell = Some(content.clone()),
                    "task" => descendant.task = Some(content.clone()),
                    "jira_issue" => descendant.jira_issue = Some(content.clone()),
                    _ => {}
                }
                return true;
            }
        }
        false
    }

    /// 设置默认内容
    fn set_default_content(descendant: &mut DescendantBlock) {
        match descendant.block_type {
            2 => {
                descendant.text = Some(serde_json::json!({
                    "elements": [{"text_run": {"content": "", "text_element_style": {}}}],
                    "style": {"align": 1}
                }));
            }
            3 => {
                descendant.heading1 = Some(serde_json::json!({
                    "elements": [{"text_run": {"content": "", "text_element_style": {}}}],
                    "style": {"align": 1, "folded": false}
                }));
            }
            4 => {
                descendant.heading2 = Some(serde_json::json!({
                    "elements": [{"text_run": {"content": "", "text_element_style": {}}}],
                    "style": {"align": 1, "folded": false}
                }));
            }
            5 => {
                descendant.heading3 = Some(serde_json::json!({
                    "elements": [{"text_run": {"content": "", "text_element_style": {}}}],
                    "style": {"align": 1, "folded": false}
                }));
            }
            12 => {
                descendant.bullet = Some(serde_json::json!({
                    "elements": [{"text_run": {"content": "", "text_element_style": {}}}],
                    "style": {"align": 1}
                }));
            }
            13 => {
                descendant.ordered = Some(serde_json::json!({
                    "elements": [{"text_run": {"content": "", "text_element_style": {}}}],
                    "style": {"align": 1}
                }));
            }
            14 => {
                descendant.code = Some(serde_json::json!({
                    "elements": [{"text_run": {"content": "", "text_element_style": {}}}],
                    "style": {"align": 1, "language": 1}
                }));
            }
            15 => {
                descendant.quote = Some(serde_json::json!({
                    "elements": [{"text_run": {"content": "", "text_element_style": {}}}],
                    "style": {"align": 1}
                }));
            }
            22 => {
                descendant.divider = Some(serde_json::json!({}));
            }
            31 => {
                descendant.table = Some(serde_json::json!({
                    "cells": [],
                    "property": {
                        "row_size": 0,
                        "column_size": 0
                    }
                }));
            }
            32 => {
                descendant.table_cell = Some(serde_json::json!({}));
            }
            _ => {
                descendant.text = Some(serde_json::json!({
                    "elements": [{"text_run": {"content": "", "text_element_style": {}}}],
                    "style": {"align": 1}
                }));
            }
        }
    }

    /// 转换整个响应
    pub fn convert_response(
        convert_response: ConvertBlocksResponse,
    ) -> Result<(Vec<String>, Vec<DescendantBlock>)> {
        // 收集所有类型32的块ID（表格列/单元格）
        let table_cell_block_ids = Self::filter_table_cell_blocks(&convert_response);

        // 过滤掉类型32的块，从first_level_block_ids中移除
        let children_id = Self::filter_first_level_block_ids(
            convert_response.first_level_block_ids,
            &table_cell_block_ids,
        );

        // 转换所有块
        let descendants: Vec<DescendantBlock> = convert_response
            .blocks
            .into_iter()
            .map(Self::convert_block)
            .collect();

        Ok((children_id, descendants))
    }
}