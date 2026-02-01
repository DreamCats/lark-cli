use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Block的基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    #[serde(rename = "block_id")]
    pub block_id: String,

    #[serde(rename = "block_type")]
    pub block_type: BlockType,

    #[serde(rename = "parent_id")]
    pub parent_id: String,

    #[serde(rename = "children", default)]
    pub children: Vec<String>,

    #[serde(rename = "comment_ids", default)]
    pub comment_ids: Vec<String>,

    // Block内容字段 - 根据block_type只有一个字段会有值
    #[serde(flatten)]
    pub content: BlockContent,
}

/// Block内容枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockContent {
    Page { page: Text },
    Text { text: Text },
    Heading1 { heading1: Text },
    Heading2 { heading2: Text },
    Heading3 { heading3: Text },
    Heading4 { heading4: Text },
    Heading5 { heading5: Text },
    Heading6 { heading6: Text },
    Heading7 { heading7: Text },
    Heading8 { heading8: Text },
    Heading9 { heading9: Text },
    Bullet { bullet: Text },
    Ordered { ordered: Text },
    Code { code: Text },
    Quote { quote: Text },
    Todo { todo: Text },
    Bitable { bitable: Bitable },
    Callout { callout: Callout },
    ChatCard { chat_card: ChatCard },
    Diagram { diagram: Diagram },
    Divider { divider: Divider },
    File { file: File },
    Grid { grid: Grid },
    GridColumn { grid_column: GridColumn },
    Iframe { iframe: Iframe },
    Image { image: Image },
    ISV { isv: ISV },
    Mindnote { mindnote: Mindnote },
    Sheet { sheet: Sheet },
    Table { table: Table },
    TableCell { table_cell: TableCell },
    View { view: View },
    JiraIssue { jira_issue: JiraIssue },
    OKR { okr: OKR },
    OKRObjective { okr_objective: OkrObjective },
    OKRKeyResult { okr_key_result: OkrKeyResult },
    OKRProgress { okr_progress: OkrProgress },
    Board { board: Board },
    Agenda { agenda: Agenda },
    AgendaItem { agenda_item: AgendaItem },
    AgendaItemTitle { agenda_item_title: AgendaItemTitle },
    AgendaItemContent { agenda_item_content: AgendaItemContent },
    LinkPreview { link_preview: LinkPreview },
    SourceSynced { source_synced: SourceSynced },
    ReferenceSynced { reference_synced: ReferenceSynced },
    SubPageList { sub_page_list: SubPageList },
    AITemplate { ai_template: AITemplate },
    Undefined { undefined: Undefined },
    QuoteContainer { quote_container: QuoteContainer },
    AddOns { add_ons: AddOns },
}

/// Block类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum BlockType {
    Page = 1,
    Text = 2,
    Heading1 = 3,
    Heading2 = 4,
    Heading3 = 5,
    Heading4 = 6,
    Heading5 = 7,
    Heading6 = 8,
    Heading7 = 9,
    Heading8 = 10,
    Heading9 = 11,
    Bullet = 12,
    Ordered = 13,
    Code = 14,
    Quote = 15,
    Todo = 17,
    Bitable = 18,
    Callout = 19,
    ChatCard = 20,
    Diagram = 21,
    Divider = 22,
    File = 23,
    Grid = 24,
    GridColumn = 25,
    Iframe = 26,
    Image = 27,
    ISV = 28,
    Mindnote = 29,
    Sheet = 30,
    Table = 31,
    TableCell = 32,
    View = 33,
    QuoteContainer = 34,
    Task = 35,
    OKR = 36,
    OKRObjective = 37,
    OKRKeyResult = 38,
    OKRProgress = 39,
    AddOns = 40,
    JiraIssue = 41,
    WikiCatalog = 42,
    Board = 43,
    Agenda = 44,
    AgendaItem = 45,
    AgendaItemTitle = 46,
    AgendaItemContent = 47,
    LinkPreview = 48,
    SourceSynced = 49,
    ReferenceSynced = 50,
    SubPageList = 51,
    AITemplate = 52,
    Undefined = 999,
}

impl BlockType {
    /// 从数值获取BlockType
    pub fn from_value(value: i32) -> Option<Self> {
        match value {
            1 => Some(BlockType::Page),
            2 => Some(BlockType::Text),
            3 => Some(BlockType::Heading1),
            4 => Some(BlockType::Heading2),
            5 => Some(BlockType::Heading3),
            6 => Some(BlockType::Heading4),
            7 => Some(BlockType::Heading5),
            8 => Some(BlockType::Heading6),
            9 => Some(BlockType::Heading7),
            10 => Some(BlockType::Heading8),
            11 => Some(BlockType::Heading9),
            12 => Some(BlockType::Bullet),
            13 => Some(BlockType::Ordered),
            14 => Some(BlockType::Code),
            15 => Some(BlockType::Quote),
            17 => Some(BlockType::Todo),
            18 => Some(BlockType::Bitable),
            19 => Some(BlockType::Callout),
            20 => Some(BlockType::ChatCard),
            21 => Some(BlockType::Diagram),
            22 => Some(BlockType::Divider),
            23 => Some(BlockType::File),
            24 => Some(BlockType::Grid),
            25 => Some(BlockType::GridColumn),
            26 => Some(BlockType::Iframe),
            27 => Some(BlockType::Image),
            28 => Some(BlockType::ISV),
            29 => Some(BlockType::Mindnote),
            30 => Some(BlockType::Sheet),
            31 => Some(BlockType::Table),
            32 => Some(BlockType::TableCell),
            33 => Some(BlockType::View),
            34 => Some(BlockType::QuoteContainer),
            35 => Some(BlockType::Task),
            36 => Some(BlockType::OKR),
            37 => Some(BlockType::OKRObjective),
            38 => Some(BlockType::OKRKeyResult),
            39 => Some(BlockType::OKRProgress),
            40 => Some(BlockType::AddOns),
            41 => Some(BlockType::JiraIssue),
            42 => Some(BlockType::WikiCatalog),
            43 => Some(BlockType::Board),
            44 => Some(BlockType::Agenda),
            45 => Some(BlockType::AgendaItem),
            46 => Some(BlockType::AgendaItemTitle),
            47 => Some(BlockType::AgendaItemContent),
            48 => Some(BlockType::LinkPreview),
            49 => Some(BlockType::SourceSynced),
            50 => Some(BlockType::ReferenceSynced),
            51 => Some(BlockType::SubPageList),
            52 => Some(BlockType::AITemplate),
            999 => Some(BlockType::Undefined),
            _ => None,
        }
    }

    /// 获取BlockType的数值
    pub fn to_value(&self) -> i32 {
        *self as i32
    }

    /// 获取BlockType对应的JSON字段名
    pub fn to_field_name(&self) -> &'static str {
        match self {
            BlockType::Page => "page",
            BlockType::Text => "text",
            BlockType::Heading1 => "heading1",
            BlockType::Heading2 => "heading2",
            BlockType::Heading3 => "heading3",
            BlockType::Heading4 => "heading4",
            BlockType::Heading5 => "heading5",
            BlockType::Heading6 => "heading6",
            BlockType::Heading7 => "heading7",
            BlockType::Heading8 => "heading8",
            BlockType::Heading9 => "heading9",
            BlockType::Bullet => "bullet",
            BlockType::Ordered => "ordered",
            BlockType::Code => "code",
            BlockType::Quote => "quote",
            BlockType::Todo => "todo",
            BlockType::Bitable => "bitable",
            BlockType::Callout => "callout",
            BlockType::ChatCard => "chat_card",
            BlockType::Diagram => "diagram",
            BlockType::Divider => "divider",
            BlockType::File => "file",
            BlockType::Grid => "grid",
            BlockType::GridColumn => "grid_column",
            BlockType::Iframe => "iframe",
            BlockType::Image => "image",
            BlockType::ISV => "isv",
            BlockType::Mindnote => "mindnote",
            BlockType::Sheet => "sheet",
            BlockType::Table => "table",
            BlockType::TableCell => "table_cell",
            BlockType::View => "view",
            BlockType::QuoteContainer => "quote_container",
            BlockType::Task => "task",
            BlockType::OKR => "okr",
            BlockType::OKRObjective => "okr_objective",
            BlockType::OKRKeyResult => "okr_key_result",
            BlockType::OKRProgress => "okr_progress",
            BlockType::Board => "board",
            BlockType::Agenda => "agenda",
            BlockType::AgendaItem => "agenda_item",
            BlockType::AgendaItemTitle => "agenda_item_title",
            BlockType::AgendaItemContent => "agenda_item_content",
            BlockType::LinkPreview => "link_preview",
            BlockType::SourceSynced => "source_synced",
            BlockType::ReferenceSynced => "reference_synced",
            BlockType::SubPageList => "sub_page_list",
            BlockType::AITemplate => "ai_template",
            BlockType::Undefined => "undefined",
            BlockType::AddOns => "add_ons",
            BlockType::JiraIssue => "jira_issue",
            BlockType::WikiCatalog => "wiki_catalog",
        }
    }
}

// Text内容实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<TextStyle>,

    #[serde(rename = "elements", default)]
    pub elements: Vec<TextElement>,
}

// Text样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
    #[serde(rename = "align", skip_serializing_if = "Option::is_none")]
    pub align: Option<Align>,

    #[serde(rename = "done", skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,

    #[serde(rename = "folded", skip_serializing_if = "Option::is_none")]
    pub folded: Option<bool>,

    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<CodeLanguage>,

    #[serde(rename = "wrap", skip_serializing_if = "Option::is_none")]
    pub wrap: Option<bool>,

    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<TextBackgroundColor>,

    #[serde(rename = "indentation_level", skip_serializing_if = "Option::is_none")]
    pub indentation_level: Option<TextIndentationLevel>,

    #[serde(rename = "sequence", skip_serializing_if = "Option::is_none")]
    pub sequence: Option<String>,
}

// 文本元素
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextElement {
    #[serde(flatten)]
    pub content: TextElementContent,
}

/// 文本元素内容枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextElementContent {
    TextRun { text_run: TextRun },
    MentionUser { mention_user: MentionUser },
    MentionDoc { mention_doc: MentionDoc },
    Reminder { reminder: Reminder },
    File { file: InlineFile },
    InlineBlock { inline_block: InlineBlock },
    Equation { equation: Equation },
    UndefinedElement { undefined_element: UndefinedElement },
}

// TextRun
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRun {
    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "text_element_style", skip_serializing_if = "Option::is_none")]
    pub text_element_style: Option<TextElementStyle>,
}

// 文本元素样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextElementStyle {
    #[serde(rename = "bold", skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,

    #[serde(rename = "italic", skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,

    #[serde(rename = "strikethrough", skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,

    #[serde(rename = "underline", skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,

    #[serde(rename = "inline_code", skip_serializing_if = "Option::is_none")]
    pub inline_code: Option<bool>,

    #[serde(rename = "text_color", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<FontColor>,

    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<FontBackgroundColor>,

    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,

    #[serde(rename = "comment_ids", skip_serializing_if = "Option::is_none")]
    pub comment_ids: Option<Vec<String>>,
}

// 链接
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "url")]
    pub url: String,
}

// MentionUser
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentionUser {
    #[serde(rename = "user_id")]
    pub user_id: String,

    #[serde(rename = "text_element_style", skip_serializing_if = "Option::is_none")]
    pub text_element_style: Option<TextElementStyle>,
}

// MentionDoc
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentionDoc {
    #[serde(rename = "token")]
    pub token: String,

    #[serde(rename = "obj_type")]
    pub obj_type: MentionObjType,

    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "text_element_style", skip_serializing_if = "Option::is_none")]
    pub text_element_style: Option<TextElementStyle>,
}

// Reminder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    #[serde(rename = "create_user_id")]
    pub create_user_id: String,

    #[serde(rename = "is_notify", skip_serializing_if = "Option::is_none")]
    pub is_notify: Option<bool>,

    #[serde(rename = "is_whole_day", skip_serializing_if = "Option::is_none")]
    pub is_whole_day: Option<bool>,

    #[serde(rename = "expire_time")]
    pub expire_time: i64,

    #[serde(rename = "notify_time")]
    pub notify_time: i64,

    #[serde(rename = "text_element_style", skip_serializing_if = "Option::is_none")]
    pub text_element_style: Option<TextElementStyle>,
}

// InlineFile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineFile {
    #[serde(rename = "file_token")]
    pub file_token: String,

    #[serde(rename = "source_block_id")]
    pub source_block_id: String,

    #[serde(rename = "text_element_style", skip_serializing_if = "Option::is_none")]
    pub text_element_style: Option<TextElementStyle>,
}

// InlineBlock
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineBlock {
    #[serde(rename = "block_id")]
    pub block_id: String,

    #[serde(rename = "text_element_style", skip_serializing_if = "Option::is_none")]
    pub text_element_style: Option<TextElementStyle>,
}

// Equation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equation {
    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "text_element_style", skip_serializing_if = "Option::is_none")]
    pub text_element_style: Option<TextElementStyle>,
}

// UndefinedElement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndefinedElement;

// Bitable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bitable {
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    #[serde(rename = "view_type")]
    pub view_type: BitableViewType,
}

// Callout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Callout {
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<CalloutBackgroundColor>,

    #[serde(rename = "border_color", skip_serializing_if = "Option::is_none")]
    pub border_color: Option<CalloutBorderColor>,

    #[serde(rename = "text_color", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<FontColor>,

    #[serde(rename = "emoji_id", skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<String>,
}

// ChatCard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCard {
    #[serde(rename = "chat_id")]
    pub chat_id: String,

    #[serde(rename = "align", skip_serializing_if = "Option::is_none")]
    pub align: Option<Align>,
}

// Diagram
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagram {
    #[serde(rename = "diagram_type", skip_serializing_if = "Option::is_none")]
    pub diagram_type: Option<DiagramType>,
}

// Divider - 空结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Divider;

// File
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "view_type", skip_serializing_if = "Option::is_none")]
    pub view_type: Option<i32>,
}

// Grid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grid {
    #[serde(rename = "column_size")]
    pub column_size: i32,
}

// GridColumn
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridColumn {
    #[serde(rename = "width_ratio", skip_serializing_if = "Option::is_none")]
    pub width_ratio: Option<i32>,
}

// Iframe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iframe {
    #[serde(rename = "component")]
    pub component: IframeComponent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IframeComponent {
    #[serde(rename = "type")]
    pub component_type: IframeComponentType,

    #[serde(rename = "url")]
    pub url: String,
}

// Image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,

    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,

    #[serde(rename = "align", skip_serializing_if = "Option::is_none")]
    pub align: Option<Align>,

    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<Caption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Caption {
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

// ISV
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ISV {
    #[serde(rename = "component_id", skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,

    #[serde(rename = "component_type_id", skip_serializing_if = "Option::is_none")]
    pub component_type_id: Option<String>,
}

// Mindnote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mindnote {
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

// Sheet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sheet {
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    #[serde(rename = "row_size", skip_serializing_if = "Option::is_none")]
    pub row_size: Option<i32>,

    #[serde(rename = "column_size", skip_serializing_if = "Option::is_none")]
    pub column_size: Option<i32>,
}

// Table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    #[serde(rename = "cells", default)]
    pub cells: Vec<String>,

    #[serde(rename = "property")]
    pub property: TableProperty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableProperty {
    #[serde(rename = "row_size")]
    pub row_size: i32,

    #[serde(rename = "column_size")]
    pub column_size: i32,

    #[serde(rename = "column_width", skip_serializing_if = "Option::is_none")]
    pub column_width: Option<Vec<i32>>,

    #[serde(rename = "header_row", skip_serializing_if = "Option::is_none")]
    pub header_row: Option<bool>,

    #[serde(rename = "header_column", skip_serializing_if = "Option::is_none")]
    pub header_column: Option<bool>,

    #[serde(rename = "merge_info", skip_serializing_if = "Option::is_none")]
    pub merge_info: Option<Vec<TableMergeInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableMergeInfo {
    #[serde(rename = "row_span", skip_serializing_if = "Option::is_none")]
    pub row_span: Option<i32>,

    #[serde(rename = "col_span", skip_serializing_if = "Option::is_none")]
    pub col_span: Option<i32>,
}

// TableCell - 空结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCell;

// View
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct View {
    #[serde(rename = "view_type", skip_serializing_if = "Option::is_none")]
    pub view_type: Option<ViewType>,
}

// JiraIssue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraIssue {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

// OKR
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OKR {
    Create {
        #[serde(rename = "okr_id")]
        okr_id: String,
        #[serde(rename = "objectives", skip_serializing_if = "Option::is_none")]
        objectives: Option<Vec<Objective>>,
    },
    Get {
        #[serde(rename = "okr_id", skip_serializing_if = "Option::is_none")]
        okr_id: Option<String>,
        #[serde(rename = "period_display_status", skip_serializing_if = "Option::is_none")]
        period_display_status: Option<OkrPeriodDisplayStatus>,
        #[serde(rename = "period_name_zh", skip_serializing_if = "Option::is_none")]
        period_name_zh: Option<String>,
        #[serde(rename = "period_name_en", skip_serializing_if = "Option::is_none")]
        period_name_en: Option<String>,
        #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
        user_id: Option<String>,
        #[serde(rename = "visible_setting", skip_serializing_if = "Option::is_none")]
        visible_setting: Option<VisibleSetting>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    #[serde(rename = "objective_id")]
    pub objective_id: String,

    #[serde(rename = "kr_ids", skip_serializing_if = "Option::is_none")]
    pub kr_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisibleSetting {
    #[serde(rename = "progress_fill_area_visible", skip_serializing_if = "Option::is_none")]
    pub progress_fill_area_visible: Option<bool>,

    #[serde(rename = "progress_status_visible", skip_serializing_if = "Option::is_none")]
    pub progress_status_visible: Option<bool>,

    #[serde(rename = "score_visible", skip_serializing_if = "Option::is_none")]
    pub score_visible: Option<bool>,
}

// OkrObjective
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OkrObjective {
    #[serde(rename = "objective_id", skip_serializing_if = "Option::is_none")]
    pub objective_id: Option<String>,

    #[serde(rename = "confidential", skip_serializing_if = "Option::is_none")]
    pub confidential: Option<bool>,

    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,

    #[serde(rename = "score", skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,

    #[serde(rename = "visible", skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,

    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<f32>,

    #[serde(rename = "progress_rate", skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<ProgressRate>,

    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<Text>,
}

// OkrKeyResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OkrKeyResult {
    #[serde(rename = "kr_id", skip_serializing_if = "Option::is_none")]
    pub kr_id: Option<String>,

    #[serde(rename = "confidential", skip_serializing_if = "Option::is_none")]
    pub confidential: Option<bool>,

    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,

    #[serde(rename = "score", skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,

    #[serde(rename = "visible", skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,

    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<f32>,

    #[serde(rename = "progress_rate", skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<ProgressRate>,

    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<Text>,
}

// Progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress;

// ProgressRate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressRate {
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<OkrProgressRateMode>,

    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    pub current: Option<f32>,

    #[serde(rename = "percent", skip_serializing_if = "Option::is_none")]
    pub percent: Option<f32>,

    #[serde(rename = "progress_status", skip_serializing_if = "Option::is_none")]
    pub progress_status: Option<OkrProgressStatus>,

    #[serde(rename = "status_type", skip_serializing_if = "Option::is_none")]
    pub status_type: Option<OkrProgressStatusType>,

    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<f32>,

    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<f32>,
}

// Board
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    #[serde(rename = "align", skip_serializing_if = "Option::is_none")]
    pub align: Option<Align>,

    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,

    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
}

// Agenda
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agenda;

// AgendaItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgendaItem;

// AgendaItemTitle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgendaItemTitle {
    #[serde(rename = "align", skip_serializing_if = "Option::is_none")]
    pub align: Option<Align>,

    #[serde(rename = "elements")]
    pub elements: Vec<AgendaTitleElement>,
}

// AgendaTitleElement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgendaTitleElement {
    #[serde(flatten)]
    pub content: AgendaTitleElementContent,
}

/// 议程标题元素内容枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AgendaTitleElementContent {
    TextRun { text_run: TextRun },
    MentionUser { mention_user: MentionUser },
    MentionDoc { mention_doc: MentionDoc },
    Reminder { reminder: Reminder },
    File { file: InlineFile },
    InlineBlock { inline_block: InlineBlock },
    Equation { equation: Equation },
    UndefinedElement { undefined_element: UndefinedElement },
}

// AgendaItemContent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgendaItemContent;

// LinkPreview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkPreview {
    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "url_type")]
    pub url_type: LinkPreviewURLType,
}

// SourceSynced
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceSynced {
    #[serde(rename = "elements", skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<TextElement>>,

    #[serde(rename = "align", skip_serializing_if = "Option::is_none")]
    pub align: Option<Align>,
}

// ReferenceSynced
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceSynced {
    #[serde(rename = "source_block_id", skip_serializing_if = "Option::is_none")]
    pub source_block_id: Option<String>,

    #[serde(rename = "source_document_id", skip_serializing_if = "Option::is_none")]
    pub source_document_id: Option<String>,
}

// SubPageList
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubPageList {
    #[serde(rename = "wiki_token", skip_serializing_if = "Option::is_none")]
    pub wiki_token: Option<String>,
}

// AITemplate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AITemplate;

// Undefined
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Undefined;

// QuoteContainer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteContainer;

// AddOns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddOns {
    #[serde(rename = "component_id", skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,

    #[serde(rename = "component_type_id", skip_serializing_if = "Option::is_none")]
    pub component_type_id: Option<String>,

    #[serde(rename = "record", skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
}

// 枚举类型定义

// Align - 对齐方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum Align {
    Left = 1,
    Center = 2,
    Right = 3,
}

// CodeLanguage - 代码块语言
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum CodeLanguage {
    PlainText = 1,
    ABAP = 2,
    Ada = 3,
    Apache = 4,
    Apex = 5,
    Assembly = 6,
    Bash = 7,
    CSharp = 8,
    Cpp = 9,
    C = 10,
    COBOL = 11,
    CSS = 12,
    CoffeeScript = 13,
    D = 14,
    Dart = 15,
    Delphi = 16,
    Django = 17,
    Dockerfile = 18,
    Erlang = 19,
    Fortran = 20,
    FoxPro = 21,
    Go = 22,
    Groovy = 23,
    HTML = 24,
    HTMLBars = 25,
    HTTP = 26,
    Haskell = 27,
    JSON = 28,
    Java = 29,
    JavaScript = 30,
    Julia = 31,
    Kotlin = 32,
    LateX = 33,
    Lisp = 34,
    Logo = 35,
    Lua = 36,
    MATLAB = 37,
    Makefile = 38,
    Markdown = 39,
    Nginx = 40,
    Objective = 41,
    OpenEdgeABL = 42,
    PHP = 43,
    Perl = 44,
    PostScript = 45,
    PowerShell = 46,
    Prolog = 47,
    ProtoBuf = 48,
    Python = 49,
    R = 50,
    RPG = 51,
    Ruby = 52,
    Rust = 53,
    SAS = 54,
    SCSS = 55,
    SQL = 56,
    Scala = 57,
    Scheme = 58,
    Scratch = 59,
    Shell = 60,
    Swift = 61,
    Thrift = 62,
    TypeScript = 63,
    VBScript = 64,
    Visual = 65,
    XML = 66,
    YAML = 67,
    CMake = 68,
    Diff = 69,
    Gherkin = 70,
    GraphQL = 71,
    OpenGLShadingLanguage = 72,
    Properties = 73,
    Solidity = 74,
    TOML = 75,
}

// TextBackgroundColor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextBackgroundColor {
    LightGrayBackground,
    LightRedBackground,
    LightOrangeBackground,
    LightYellowBackground,
    LightGreenBackground,
    LightBlueBackground,
    LightPurpleBackground,
    PaleGrayBackground,
    DarkGrayBackground,
    DarkRedBackground,
    DarkOrangeBackground,
    DarkYellowBackground,
    DarkGreenBackground,
    DarkBlueBackground,
    DarkPurpleBackground,
}

// TextIndentationLevel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextIndentationLevel {
    NoIndent,
    OneLevelIndent,
}

// FontColor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum FontColor {
    Red = 1,
    Orange = 2,
    Yellow = 3,
    Green = 4,
    Blue = 5,
    Purple = 6,
    Gray = 7,
}

// FontBackgroundColor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum FontBackgroundColor {
    LightRed = 1,
    LightOrange = 2,
    LightYellow = 3,
    LightGreen = 4,
    LightBlue = 5,
    LightPurple = 6,
    MediumGray = 7,
    Red = 8,
    Orange = 9,
    Yellow = 10,
    Green = 11,
    Blue = 12,
    Purple = 13,
    Gray = 14,
    LightGray = 15,
}

// BitableViewType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum BitableViewType {
    Grid = 1,
    Kanban = 2,
}

// CalloutBackgroundColor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum CalloutBackgroundColor {
    LightRed = 1,
    LightOrange = 2,
    LightYellow = 3,
    LightGreen = 4,
    LightBlue = 5,
    LightPurple = 6,
    MediumGray = 7,
    MediumRed = 8,
    MediumOrange = 9,
    MediumYellow = 10,
    MediumGreen = 11,
    MediumBlue = 12,
    MediumPurple = 13,
    Gray = 14,
    LightGray = 15,
}

// CalloutBorderColor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum CalloutBorderColor {
    Red = 1,
    Orange = 2,
    Yellow = 3,
    Green = 4,
    Blue = 5,
    Purple = 6,
    Gray = 7,
}

// DiagramType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum DiagramType {
    Flowchart = 1,
    UML = 2,
}

// IframeComponentType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum IframeComponentType {
    Bilibili = 1,
    XiguaVideo = 2,
    Youku = 3,
    Airtable = 4,
    BaiduMap = 5,
    Amap = 6,
    Undefined7 = 7,
    Figma = 8,
    Modao = 9,
    Canva = 10,
    CodePen = 11,
    FeishuSurvey = 12,
    Jinshuju = 13,
    Undefined14 = 14,
    Undefined15 = 15,
}

// MentionObjType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum MentionObjType {
    Doc = 1,
    Sheet = 3,
    Bitable = 8,
    MindNote = 11,
    File = 12,
    Slide = 15,
    Wiki = 16,
    Docx = 22,
}

// OkrPeriodDisplayStatus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OkrPeriodDisplayStatus {
    Default,
    Normal,
    Invalid,
    Hidden,
}

// OkrProgressRateMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OkrProgressRateMode {
    Simple,
    Advanced,
}

// OkrProgressStatus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OkrProgressStatus {
    Unset,
    Normal,
    Risk,
    Extended,
}

// OkrProgressStatusType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OkrProgressStatusType {
    Default,
    Custom,
}

// LinkPreviewURLType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkPreviewURLType {
    MessageLink,
    Undefined,
}

// ViewType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum ViewType {
    Card = 1,
    Preview = 2,
    Inline = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_serialization() {
        let block = Block {
            block_id: "test_id".to_string(),
            block_type: BlockType::Text,
            parent_id: "parent_id".to_string(),
            children: vec![],
            comment_ids: vec![],
            content: BlockContent::Text {
                text: Text {
                    style: Some(TextStyle {
                        align: Some(Align::Left),
                        done: None,
                        folded: None,
                        language: None,
                        wrap: None,
                        background_color: None,
                        indentation_level: None,
                        sequence: None,
                    }),
                    elements: vec![TextElement {
                        content: TextElementContent::TextRun {
                            text_run: TextRun {
                                content: "Hello World".to_string(),
                                text_element_style: None,
                            },
                        },
                    }],
                },
            },
        };

        let json = serde_json::to_string_pretty(&block).unwrap();
        println!("Block JSON: {}", json);

        // 验证反序列化
        let deserialized: Block = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.block_id, "test_id");
        assert_eq!(deserialized.block_type, BlockType::Text);
    }

    #[test]
    fn test_table_block() {
        let table_block = Block {
            block_id: "table_123".to_string(),
            block_type: BlockType::Table,
            parent_id: "parent_123".to_string(),
            children: vec![],
            comment_ids: vec![],
            content: BlockContent::Table {
                table: Table {
                    cells: vec!["cell1".to_string(), "cell2".to_string()],
                    property: TableProperty {
                        row_size: 2,
                        column_size: 2,
                        column_width: Some(vec![100, 200]),
                        header_row: Some(true),
                        header_column: Some(false),
                        merge_info: Some(vec![TableMergeInfo {
                            row_span: Some(1),
                            col_span: Some(2),
                        }]),
                    },
                },
            },
        };

        let json = serde_json::to_string_pretty(&table_block).unwrap();
        println!("Table Block JSON: {}", json);

        let deserialized: Block = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.block_type, BlockType::Table);
    }
}