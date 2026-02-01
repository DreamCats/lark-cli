#![allow(dead_code)]

use super::ApiClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 创建画板节点的请求
#[derive(Debug, Serialize)]
pub struct CreateBoardNotesRequest {
    pub nodes: Vec<Value>,
}

/// 画板节点
#[derive(Debug, Serialize, Deserialize)]
pub struct WhiteboardNode {
    /// 节点 id，用于唯一标识此节点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 节点图形类型
    #[serde(rename = "type")]
    pub node_type: String,
    /// 父节点 id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 图形相对画布的 x 轴位置信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /// 图形相对画布的 y 轴位置信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    /// 图形旋转角度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    /// 图形高度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// 图形宽度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    /// 图形内文字
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Text>,
    /// 图形样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
    /// 基础图形属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_shape: Option<CompositeShape>,
    /// 图片属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    /// 连线属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector: Option<Connector>,
    /// 表格属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Table>,
    /// 分区属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<Section>,
    /// 生命对象属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifeline: Option<Lifeline>,
    /// 画笔属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paint: Option<Paint>,
    /// svg 图形属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svg: Option<Svg>,
    /// 便签图形属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_note: Option<StickyNote>,
    /// 思维导图节点属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mind_map_node: Option<MindMapNode>,
    /// 思维导图根节点属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mind_map_root: Option<MindMapRoot>,
    /// 图形是否锁定
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// 图形在兄弟节点中的层级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z_index: Option<i32>,
}

/// 文字属性
#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    /// 文字内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 文字字重
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<String>,
    /// 文字大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<i32>,
    /// 水平对齐
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_align: Option<String>,
    /// 垂直对齐
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_align: Option<String>,
    /// 文字颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    /// 文字背景色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_background_color: Option<String>,
    /// 是否存在删除线
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_through: Option<bool>,
    /// 是否存在下划线
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    /// 是否斜体
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    /// 文字旋转角度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<i32>,
    /// 文字颜色主题配色编码值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_text_color_code: Option<i32>,
    /// 文字背景颜色主题配色编码值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_text_background_color_code: Option<i32>,
    /// 文字颜色类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color_type: Option<i32>,
    /// 文字背景颜色类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_background_color_type: Option<i32>,
    /// 富文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_text: Option<RichText>,
}

/// 图形样式
#[derive(Debug, Serialize, Deserialize)]
pub struct Style {
    /// 填充颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_color: Option<String>,
    /// 填充透明度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_opacity: Option<f64>,
    /// 边框样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_style: Option<String>,
    /// 边框宽度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_width: Option<String>,
    /// 边框透明度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_opacity: Option<f64>,
    /// 水平翻折
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_flip: Option<bool>,
    /// 垂直翻折
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_flip: Option<bool>,
    /// 边框颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,
    /// 填充颜色主题配色编码值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_fill_color_code: Option<i32>,
    /// 边框颜色主题配色编码值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_border_color_code: Option<i32>,
    /// 填充颜色类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_color_type: Option<i32>,
    /// 边框颜色类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color_type: Option<i32>,
}

/// 基础图形属性
#[derive(Debug, Serialize, Deserialize)]
pub struct CompositeShape {
    /// 基础图形的具体类型
    #[serde(rename = "type")]
    pub shape_type: String,
    /// 饼图属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie: Option<Pie>,
    /// 圆环属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circular_ring: Option<Pie>,
}

/// 饼图属性
#[derive(Debug, Serialize, Deserialize)]
pub struct Pie {
    /// 开始径向边角度
    pub start_radial_line_angle: f64,
    /// 圆心角角度
    pub central_angle: f64,
    /// 半径长度
    pub radius: f64,
    /// 扇区占比
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sector_ratio: Option<f64>,
}

/// 图片属性
#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    /// 图片 token
    pub token: String,
}

/// 连线属性
#[derive(Debug, Serialize, Deserialize)]
pub struct Connector {
    /// 起始端点信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<ConnectorInfo>,
    /// 结束端点信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<ConnectorInfo>,
    /// 连线文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captions: Option<ConnectorCaption>,
    /// 连线类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    /// 连线转向点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turning_points: Option<Vec<Point>>,
    /// 连线上的文本方向是否自动跟随连线方向
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_auto_direction: Option<bool>,
    /// 文本在连线上的相对位置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_position: Option<f64>,
}

/// 连线端点信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectorInfo {
    /// 连接图形信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_object: Option<ConnectorAttachedObject>,
    /// 连线端点在画布内的坐标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Point>,
    /// 连线端点箭头样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrow_style: Option<String>,
}

/// 连接图形信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectorAttachedObject {
    /// 连接图形的 id
    pub id: String,
    /// 连接图形的方向
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_to: Option<String>,
    /// 连接图形的相对坐标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Point>,
}

/// 点坐标
#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    /// x 坐标
    pub x: f64,
    /// y 坐标
    pub y: f64,
}

/// 连线文本
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectorCaption {
    /// 文本列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Text>>,
}

/// 表格属性
#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    /// 元信息
    pub meta: TableMeta,
    /// 整个表格的样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
    /// 整个表格的文字样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Text>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 单元格列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cells: Option<Vec<TableCell>>,
}

/// 表格元信息
#[derive(Debug, Serialize, Deserialize)]
pub struct TableMeta {
    /// 行高
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_sizes: Option<Vec<f64>>,
    /// 列宽
    #[serde(skip_serializing_if = "Option::is_none")]
    pub col_sizes: Option<Vec<f64>>,
}

/// 表格单元格
#[derive(Debug, Serialize, Deserialize)]
pub struct TableCell {
    /// 行下标
    pub row_index: i32,
    /// 列下标
    pub col_index: i32,
    /// 单元格合并信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_info: Option<CellMergeInfo>,
    /// 单元格包含的子节点 id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<String>>,
    /// 单元格内文字
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Text>,
    /// 单元格样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
}

/// 单元格合并信息
#[derive(Debug, Serialize, Deserialize)]
pub struct CellMergeInfo {
    /// 被合并的连续行数
    pub row_span: i32,
    /// 被合并的连续列数
    pub col_span: i32,
}

/// 分区属性
#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    /// 分区标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// 生命对象属性
#[derive(Debug, Serialize, Deserialize)]
pub struct Lifeline {
    /// 生命线长度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    /// 生命线类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifeline_type: Option<String>,
}

/// 画笔属性
#[derive(Debug, Serialize, Deserialize)]
pub struct Paint {
    /// 画笔类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paint_type: Option<String>,
    /// 画板线段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<Point>>,
    /// 画笔粗细
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// 画笔颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// svg 图形属性
#[derive(Debug, Serialize, Deserialize)]
pub struct Svg {
    /// svg 代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svg_code: Option<String>,
}

/// 便签图形属性
#[derive(Debug, Serialize, Deserialize)]
pub struct StickyNote {
    /// 用户 id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 是否展示用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_author_info: Option<bool>,
}

/// 思维导图节点属性
#[derive(Debug, Serialize, Deserialize)]
pub struct MindMapNode {
    /// 思维导图节点的父节点
    pub parent_id: String,
    /// 思维导图节点图形类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 思维导图节点在兄弟节点中的位置index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z_index: Option<i32>,
    /// 子节点相对根节点的方向
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_position: Option<String>,
    /// 是否收起子节点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapsed: Option<bool>,
}

/// 思维导图根节点属性
#[derive(Debug, Serialize, Deserialize)]
pub struct MindMapRoot {
    /// 思维导图布局方式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
    /// 思维导图根节点图形类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 思维导图图形连接线样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_style: Option<String>,
}

/// 富文本
#[derive(Debug, Serialize, Deserialize)]
pub struct RichText {
    /// 段落列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paragraphs: Option<Vec<RichTextParagraph>>,
}

/// 富文本段落
#[derive(Debug, Serialize, Deserialize)]
pub struct RichTextParagraph {
    /// 段落类别
    pub paragraph_type: i32,
    /// 段落的元素列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<RichTextElement>>,
}

/// 富文本元素
#[derive(Debug, Serialize, Deserialize)]
pub struct RichTextElement {
    /// 元素类别
    pub element_type: i32,
    /// 文本元素信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_element: Option<RichTextElementText>,
}

/// 文本元素
#[derive(Debug, Serialize, Deserialize)]
pub struct RichTextElementText {
    /// 文字
    pub text: String,
}

/// 创建画板节点的响应
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBoardNotesResponse {
    /// 所创建的节点 id 列表
    pub ids: Vec<String>,
    /// 操作的唯一标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

pub struct CreateBoardNotesApi {
    client: ApiClient,
}

impl CreateBoardNotesApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    /// 创建画板节点
    /// 支持批量创建、创建含父子关系的节点等
    pub async fn create_board_notes(
        &self,
        whiteboard_id: &str,
        request: CreateBoardNotesRequest,
        client_token: Option<String>,
        user_id_type: Option<String>,
    ) -> Result<CreateBoardNotesResponse> {
        // 验证参数
        if whiteboard_id.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "whiteboard_id 参数是必需的".to_string(),
            ));
        }

        if request.nodes.is_empty() {
            return Err(crate::error::LarkError::ValidationError(
                "nodes 参数不能为空".to_string(),
            ));
        }

        // 构建查询参数
        let mut params = std::collections::HashMap::new();
        if let Some(token) = client_token {
            params.insert("client_token".to_string(), token);
        }
        if let Some(id_type) = user_id_type {
            params.insert("user_id_type".to_string(), id_type);
        }

        // 构建请求 URL
        let url = format!(
            "https://open.larkoffice.com/open-apis/board/v1/whiteboards/{}/nodes",
            whiteboard_id
        );

        let response: CreateBoardNotesResponse =
            self.client.post_with_params(&url, Some(params), &request).await?;

        tracing::debug!(
            "Created {} board nodes successfully",
            response.ids.len()
        );

        Ok(response)
    }
}
