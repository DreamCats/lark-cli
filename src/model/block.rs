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
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<Text>,

    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<Text>,

    #[serde(rename = "heading1", skip_serializing_if = "Option::is_none")]
    pub heading1: Option<Text>,

    #[serde(rename = "heading2", skip_serializing_if = "Option::is_none")]
    pub heading2: Option<Text>,

    #[serde(rename = "heading3", skip_serializing_if = "Option::is_none")]
    pub heading3: Option<Text>,

    #[serde(rename = "heading4", skip_serializing_if = "Option::is_none")]
    pub heading4: Option<Text>,

    #[serde(rename = "heading5", skip_serializing_if = "Option::is_none")]
    pub heading5: Option<Text>,

    #[serde(rename = "heading6", skip_serializing_if = "Option::is_none")]
    pub heading6: Option<Text>,

    #[serde(rename = "heading7", skip_serializing_if = "Option::is_none")]
    pub heading7: Option<Text>,

    #[serde(rename = "heading8", skip_serializing_if = "Option::is_none")]
    pub heading8: Option<Text>,

    #[serde(rename = "heading9", skip_serializing_if = "Option::is_none")]
    pub heading9: Option<Text>,

    #[serde(rename = "bullet", skip_serializing_if = "Option::is_none")]
    pub bullet: Option<Text>,

    #[serde(rename = "ordered", skip_serializing_if = "Option::is_none")]
    pub ordered: Option<Text>,

    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<Text>,

    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<Text>,

    #[serde(rename = "todo", skip_serializing_if = "Option::is_none")]
    pub todo: Option<Text>,

    #[serde(rename = "bitable", skip_serializing_if = "Option::is_none")]
    pub bitable: Option<Bitable>,

    #[serde(rename = "callout", skip_serializing_if = "Option::is_none")]
    pub callout: Option<Callout>,

    #[serde(rename = "chat_card", skip_serializing_if = "Option::is_none")]
    pub chat_card: Option<ChatCard>,

    #[serde(rename = "diagram", skip_serializing_if = "Option::is_none")]
    pub diagram: Option<Diagram>,

    #[serde(rename = "divider", skip_serializing_if = "Option::is_none")]
    pub divider: Option<Divider>,

    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<File>,

    #[serde(rename = "grid", skip_serializing_if = "Option::is_none")]
    pub grid: Option<Grid>,

    #[serde(rename = "grid_column", skip_serializing_if = "Option::is_none")]
    pub grid_column: Option<GridColumn>,

    #[serde(rename = "iframe", skip_serializing_if = "Option::is_none")]
    pub iframe: Option<Iframe>,

    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,

    #[serde(rename = "isv", skip_serializing_if = "Option::is_none")]
    pub isv: Option<ISV>,

    #[serde(rename = "mindnote", skip_serializing_if = "Option::is_none")]
    pub mindnote: Option<Mindnote>,

    #[serde(rename = "sheet", skip_serializing_if = "Option::is_none")]
    pub sheet: Option<Sheet>,

    #[serde(rename = "table", skip_serializing_if = "Option::is_none")]
    pub table: Option<Table>,

    #[serde(rename = "table_cell", skip_serializing_if = "Option::is_none")]
    pub table_cell: Option<TableCell>,

    #[serde(rename = "view", skip_serializing_if = "Option::is_none")]
    pub view: Option<View>,

    #[serde(rename = "jira_issue", skip_serializing_if = "Option::is_none")]
    pub jira_issue: Option<JiraIssue>,

    #[serde(rename = "okr", skip_serializing_if = "Option::is_none")]
    pub okr: Option<OKR>,

    #[serde(rename = "okr_objective", skip_serializing_if = "Option::is_none")]
    pub okr_objective: Option<OkrObjective>,

    #[serde(rename = "okr_key_result", skip_serializing_if = "Option::is_none")]
    pub okr_key_result: Option<OkrKeyResult>,

    #[serde(rename = "okr_progress", skip_serializing_if = "Option::is_none")]
    pub okr_progress: Option<OkrProgress>,

    #[serde(rename = "board", skip_serializing_if = "Option::is_none")]
    pub board: Option<Board>,

    #[serde(rename = "agenda", skip_serializing_if = "Option::is_none")]
    pub agenda: Option<Agenda>,

    #[serde(rename = "agenda_item", skip_serializing_if = "Option::is_none")]
    pub agenda_item: Option<AgendaItem>,

    #[serde(rename = "agenda_item_title", skip_serializing_if = "Option::is_none")]
    pub agenda_item_title: Option<AgendaItemTitle>,

    #[serde(rename = "agenda_item_content", skip_serializing_if = "Option::is_none")]
    pub agenda_item_content: Option<AgendaItemContent>,

    #[serde(rename = "link_preview", skip_serializing_if = "Option::is_none")]
    pub link_preview: Option<LinkPreview>,

    #[serde(rename = "source_synced", skip_serializing_if = "Option::is_none")]
    pub source_synced: Option<SourceSynced>,

    #[serde(rename = "reference_synced", skip_serializing_if = "Option::is_none")]
    pub reference_synced: Option<ReferenceSynced>,

    #[serde(rename = "sub_page_list", skip_serializing_if = "Option::is_none")]
    pub sub_page_list: Option<SubPageList>,

    #[serde(rename = "ai_template", skip_serializing_if = "Option::is_none")]
    pub ai_template: Option<AITemplate>,

    #[serde(rename = "undefined", skip_serializing_if = "Option::is_none")]
    pub undefined: Option<Undefined>,

    #[serde(rename = "quote_container", skip_serializing_if = "Option::is_none")]
    pub quote_container: Option<QuoteContainer>,

    #[serde(rename = "add_ons", skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<AddOns>,
}

impl Block {
    /// 获取block的内容字段名称
    pub fn get_content_field(&self) -> Option<&str> {
        match self.block_type {
            BlockType::Page => self.page.as_ref().map(|_| "page"),
            BlockType::Text => self.text.as_ref().map(|_| "text"),
            BlockType::Heading1 => self.heading1.as_ref().map(|_| "heading1"),
            BlockType::Heading2 => self.heading2.as_ref().map(|_| "heading2"),
            BlockType::Heading3 => self.heading3.as_ref().map(|_| "heading3"),
            BlockType::Heading4 => self.heading4.as_ref().map(|_| "heading4"),
            BlockType::Heading5 => self.heading5.as_ref().map(|_| "heading5"),
            BlockType::Heading6 => self.heading6.as_ref().map(|_| "heading6"),
            BlockType::Heading7 => self.heading7.as_ref().map(|_| "heading7"),
            BlockType::Heading8 => self.heading8.as_ref().map(|_| "heading8"),
            BlockType::Heading9 => self.heading9.as_ref().map(|_| "heading9"),
            BlockType::Bullet => self.bullet.as_ref().map(|_| "bullet"),
            BlockType::Ordered => self.ordered.as_ref().map(|_| "ordered"),
            BlockType::Code => self.code.as_ref().map(|_| "code"),
            BlockType::Quote => self.quote.as_ref().map(|_| "quote"),
            BlockType::Todo => self.todo.as_ref().map(|_| "todo"),
            BlockType::Bitable => self.bitable.as_ref().map(|_| "bitable"),
            BlockType::Callout => self.callout.as_ref().map(|_| "callout"),
            BlockType::ChatCard => self.chat_card.as_ref().map(|_| "chat_card"),
            BlockType::Diagram => self.diagram.as_ref().map(|_| "diagram"),
            BlockType::Divider => self.divider.as_ref().map(|_| "divider"),
            BlockType::File => self.file.as_ref().map(|_| "file"),
            BlockType::Grid => self.grid.as_ref().map(|_| "grid"),
            BlockType::GridColumn => self.grid_column.as_ref().map(|_| "grid_column"),
            BlockType::Iframe => self.iframe.as_ref().map(|_| "iframe"),
            BlockType::Image => self.image.as_ref().map(|_| "image"),
            BlockType::ISV => self.isv.as_ref().map(|_| "isv"),
            BlockType::Mindnote => self.mindnote.as_ref().map(|_| "mindnote"),
            BlockType::Sheet => self.sheet.as_ref().map(|_| "sheet"),
            BlockType::Table => self.table.as_ref().map(|_| "table"),
            BlockType::TableCell => self.table_cell.as_ref().map(|_| "table_cell"),
            BlockType::View => self.view.as_ref().map(|_| "view"),
            BlockType::JiraIssue => self.jira_issue.as_ref().map(|_| "jira_issue"),
            BlockType::OKR => self.okr.as_ref().map(|_| "okr"),
            BlockType::OKRObjective => self.okr_objective.as_ref().map(|_| "okr_objective"),
            BlockType::OKRKeyResult => self.okr_key_result.as_ref().map(|_| "okr_key_result"),
            BlockType::OKRProgress => self.okr_progress.as_ref().map(|_| "okr_progress"),
            BlockType::Board => self.board.as_ref().map(|_| "board"),
            BlockType::Agenda => self.agenda.as_ref().map(|_| "agenda"),
            BlockType::AgendaItem => self.agenda_item.as_ref().map(|_| "agenda_item"),
            BlockType::AgendaItemTitle => self.agenda_item_title.as_ref().map(|_| "agenda_item_title"),
            BlockType::AgendaItemContent => self.agenda_item_content.as_ref().map(|_| "agenda_item_content"),
            BlockType::LinkPreview => self.link_preview.as_ref().map(|_| "link_preview"),
            BlockType::SourceSynced => self.source_synced.as_ref().map(|_| "source_synced"),
            BlockType::ReferenceSynced => self.reference_synced.as_ref().map(|_| "reference_synced"),
            BlockType::SubPageList => self.sub_page_list.as_ref().map(|_| "sub_page_list"),
            BlockType::AITemplate => self.ai_template.as_ref().map(|_| "ai_template"),
            BlockType::Undefined => self.undefined.as_ref().map(|_| "undefined"),
            BlockType::QuoteContainer => self.quote_container.as_ref().map(|_| "quote_container"),
            BlockType::AddOns => self.add_ons.as_ref().map(|_| "add_ons"),
        }
    }

    /// 获取block的可变内容字段
    pub fn get_content_field_mut(&mut self) -> Option<&mut serde_json::Value> {
        match self.block_type {
            BlockType::Page => self.page.as_mut().map(|p| {
                // 这里需要特殊处理，因为page字段是Text类型，我们需要将其转换为serde_json::Value
                // 在实际实现中，可能需要使用序列化/反序列化
                serde_json::to_value(p).ok()
            }).flatten(),
            BlockType::Text => self.text.as_mut().map(|t| serde_json::to_value(t).ok()).flatten(),
            BlockType::Heading1 => self.heading1.as_mut().map(|h| serde_json::to_value(h).ok()).flatten(),
            BlockType::Heading2 => self.heading2.as_mut().map(|h| serde_json::to_value(h).ok()).flatten(),
            BlockType::Heading3 => self.heading3.as_mut().map(|h| serde_json::to_value(h).ok()).flatten(),
            BlockType::Heading4 => self.heading4.as_mut().map(|h| serde_json::to_value(h).ok()).flatten(),
            BlockType::Heading5 => self.heading5.as_mut().map(|h| serde_json::to_value(h).ok()).flatten(),
            BlockType::Heading6 => self.heading6.as_mut().map(|h| serde_json::to_value(h).ok()).flatten(),
            BlockType::Heading7 => self.heading7.as_mut().map(|h| serde_json::to_value(h).ok()).flatten(),
            BlockType::Heading8 => self.heading8.as_mut().map(|h| serde_json::to_value(h).ok()).flatten(),
            BlockType::Heading9 => self.heading9.as_mut().map(|h| serde_json::to_value(h).ok()).flatten(),
            BlockType::Bullet => self.bullet.as_mut().map(|b| serde_json::to_value(b).ok()).flatten(),
            BlockType::Ordered => self.ordered.as_mut().map(|o| serde_json::to_value(o).ok()).flatten(),
            BlockType::Code => self.code.as_mut().map(|c| serde_json::to_value(c).ok()).flatten(),
            BlockType::Quote => self.quote.as_mut().map(|q| serde_json::to_value(q).ok()).flatten(),
            BlockType::Todo => self.todo.as_mut().map(|t| serde_json::to_value(t).ok()).flatten(),
            BlockType::Bitable => self.bitable.as_mut().map(|b| serde_json::to_value(b).ok()).flatten(),
            BlockType::Callout => self.callout.as_mut().map(|c| serde_json::to_value(c).ok()).flatten(),
            BlockType::ChatCard => self.chat_card.as_mut().map(|c| serde_json::to_value(c).ok()).flatten(),
            BlockType::Diagram => self.diagram.as_mut().map(|d| serde_json::to_value(d).ok()).flatten(),
            BlockType::Divider => self.divider.as_mut().map(|d| serde_json::to_value(d).ok()).flatten(),
            BlockType::File => self.file.as_mut().map(|f| serde_json::to_value(f).ok()).flatten(),
            BlockType::Grid => self.grid.as_mut().map(|g| serde_json::to_value(g).ok()).flatten(),
            BlockType::GridColumn => self.grid_column.as_mut().map(|g| serde_json::to_value(g).ok()).flatten(),
            BlockType::Iframe => self.iframe.as_mut().map(|i| serde_json::to_value(i).ok()).flatten(),
            BlockType::Image => self.image.as_mut().map(|i| serde_json::to_value(i).ok()).flatten(),
            BlockType::ISV => self.isv.as_mut().map(|i| serde_json::to_value(i).ok()).flatten(),
            BlockType::Mindnote => self.mindnote.as_mut().map(|m| serde_json::to_value(m).ok()).flatten(),
            BlockType::Sheet => self.sheet.as_mut().map(|s| serde_json::to_value(s).ok()).flatten(),
            BlockType::Table => self.table.as_mut().map(|t| serde_json::to_value(t).ok()).flatten(),
            BlockType::TableCell => self.table_cell.as_mut().map(|t| serde_json::to_value(t).ok()).flatten(),
            BlockType::View => self.view.as_mut().map(|v| serde_json::to_value(v).ok()).flatten(),
            BlockType::JiraIssue => self.jira_issue.as_mut().map(|j| serde_json::to_value(j).ok()).flatten(),
            BlockType::OKR => self.okr.as_mut().map(|o| serde_json::to_value(o).ok()).flatten(),
            BlockType::OKRObjective => self.okr_objective.as_mut().map(|o| serde_json::to_value(o).ok()).flatten(),
            BlockType::OKRKeyResult => self.okr_key_result.as_mut().map(|k| serde_json::to_value(k).ok()).flatten(),
            BlockType::OKRProgress => self.okr_progress.as_mut().map(|p| serde_json::to_value(p).ok()).flatten(),
            BlockType::Board => self.board.as_mut().map(|b| serde_json::to_value(b).ok()).flatten(),
            BlockType::Agenda => self.agenda.as_mut().map(|a| serde_json::to_value(a).ok()).flatten(),
            BlockType::AgendaItem => self.agenda_item.as_mut().map(|a| serde_json::to_value(a).ok()).flatten(),
            BlockType::AgendaItemTitle => self.agenda_item_title.as_mut().map(|a| serde_json::to_value(a).ok()).flatten(),
            BlockType::AgendaItemContent => self.agenda_item_content.as_mut().map(|a| serde_json::to_value(a).ok()).flatten(),
            BlockType::LinkPreview => self.link_preview.as_mut().map(|l| serde_json::to_value(l).ok()).flatten(),
            BlockType::SourceSynced => self.source_synced.as_mut().map(|s| serde_json::to_value(s).ok()).flatten(),
            BlockType::ReferenceSynced => self.reference_synced.as_mut().map(|r| serde_json::to_value(r).ok()).flatten(),
            BlockType::SubPageList => self.sub_page_list.as_mut().map(|s| serde_json::to_value(s).ok()).flatten(),
            BlockType::AITemplate => self.ai_template.as_mut().map(|a| serde_json::to_value(a).ok()).flatten(),
            BlockType::Undefined => self.undefined.as_mut().map(|u| serde_json::to_value(u).ok()).flatten(),
            BlockType::QuoteContainer => self.quote_container.as_mut().map(|q| serde_json::to_value(q).ok()).flatten(),
            BlockType::AddOns => self.add_ons.as_mut().map(|a| serde_json::to_value(a).ok()).flatten(),
        }
    }
}

/// Block类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockType {
    #[serde(rename = "page")]
    Page = 1,
    #[serde(rename = "text")]
    Text = 2,
    #[serde(rename = "heading1")]
    Heading1 = 3,
    #[serde(rename = "heading2")]
    Heading2 = 4,
    #[serde(rename = "heading3")]
    Heading3 = 5,
    #[serde(rename = "heading4")]
    Heading4 = 6,
    #[serde(rename = "heading5")]
    Heading5 = 7,
    #[serde(rename = "heading6")]
    Heading6 = 8,
    #[serde(rename = "heading7")]
    Heading7 = 9,
    #[serde(rename = "heading8")]
    Heading8 = 10,
    #[serde(rename = "heading9")]
    Heading9 = 11,
    #[serde(rename = "bullet")]
    Bullet = 12,
    #[serde(rename = "ordered")]
    Ordered = 13,
    #[serde(rename = "code")]
    Code = 14,
    #[serde(rename = "quote")]
    Quote = 15,
    #[serde(rename = "todo")]
    Todo = 17,
    #[serde(rename = "bitable")]
    Bitable = 18,
    #[serde(rename = "callout")]
    Callout = 19,
    #[serde(rename = "chat_card")]
    ChatCard = 20,
    #[serde(rename = "diagram")]
    Diagram = 21,
    #[serde(rename = "divider")]
    Divider = 22,
    #[serde(rename = "file")]
    File = 23,
    #[serde(rename = "grid")]
    Grid = 24,
    #[serde(rename = "grid_column")]
    GridColumn = 25,
    #[serde(rename = "iframe")]
    Iframe = 26,
    #[serde(rename = "image")]
    Image = 27,
    #[serde(rename = "isv")]
    ISV = 28,
    #[serde(rename = "mindnote")]
    Mindnote = 29,
    #[serde(rename = "sheet")]
    Sheet = 30,
    #[serde(rename = "table")]
    Table = 31,
    #[serde(rename = "table_cell")]
    TableCell = 32,
    #[serde(rename = "view")]
    View = 33,
    #[serde(rename = "quote_container")]
    QuoteContainer = 34,
    #[serde(rename = "task")]
    Task = 35,
    #[serde(rename = "okr")]
    OKR = 36,
    #[serde(rename = "okr_objective")]
    OKRObjective = 37,
    #[serde(rename = "okr_key_result")]
    OKRKeyResult = 38,
    #[serde(rename = "okr_progress")]
    OKRProgress = 39,
    #[serde(rename = "add_ons")]
    AddOns = 40,
    #[serde(rename = "jira_issue")]
    JiraIssue = 41,
    #[serde(rename = "wiki_catalog")]
    WikiCatalog = 42,
    #[serde(rename = "board")]
    Board = 43,
    #[serde(rename = "agenda")]
    Agenda = 44,
    #[serde(rename = "agenda_item")]
    AgendaItem = 45,
    #[serde(rename = "agenda_item_title")]
    AgendaItemTitle = 46,
    #[serde(rename = "agenda_item_content")]
    AgendaItemContent = 47,
    #[serde(rename = "link_preview")]
    LinkPreview = 48,
    #[serde(rename = "source_synced")]
    SourceSynced = 49,
    #[serde(rename = "reference_synced")]
    ReferenceSynced = 50,
    #[serde(rename = "sub_page_list")]
    SubPageList = 51,
    #[serde(rename = "ai_template")]
    AITemplate = 52,
    #[serde(rename = "undefined")]
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
    #[serde(rename = "text_run", skip_serializing_if = "Option::is_none")]
    pub text_run: Option<TextRun>,

    #[serde(rename = "mention_user", skip_serializing_if = "Option::is_none")]
    pub mention_user: Option<MentionUser>,

    #[serde(rename = "mention_doc", skip_serializing_if = "Option::is_none")]
    pub mention_doc: Option<MentionDoc>,

    #[serde(rename = "reminder", skip_serializing_if = "Option::is_none")]
    pub reminder: Option<Reminder>,

    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<InlineFile>,

    #[serde(rename = "inline_block", skip_serializing_if = "Option::is_none")]
    pub inline_block: Option<InlineBlock>,

    #[serde(rename = "equation", skip_serializing_if = "Option::is_none")]
    pub equation: Option<Equation>,

    #[serde(rename = "undefined_element", skip_serializing_if = "Option::is_none")]
    pub undefined_element: Option<UndefinedElement>,
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
    #[serde(rename = "text_run", skip_serializing_if = "Option::is_none")]
    pub text_run: Option<TextRun>,

    #[serde(rename = "mention_user", skip_serializing_if = "Option::is_none")]
    pub mention_user: Option<MentionUser>,

    #[serde(rename = "mention_doc", skip_serializing_if = "Option::is_none")]
    pub mention_doc: Option<MentionDoc>,

    #[serde(rename = "reminder", skip_serializing_if = "Option::is_none")]
    pub reminder: Option<Reminder>,

    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<InlineFile>,

    #[serde(rename = "inline_block", skip_serializing_if = "Option::is_none")]
    pub inline_block: Option<InlineBlock>,

    #[serde(rename = "equation", skip_serializing_if = "Option::is_none")]
    pub equation: Option<Equation>,

    #[serde(rename = "undefined_element", skip_serializing_if = "Option::is_none")]
    pub undefined_element: Option<UndefinedElement>,
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
#[serde(rename_all = "snake_case")]
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
            page: None,
            text: Some(Text {
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
                    text_run: Some(TextRun {
                        content: "Hello World".to_string(),
                        text_element_style: None,
                    }),
                    mention_user: None,
                    mention_doc: None,
                    reminder: None,
                    file: None,
                    inline_block: None,
                    equation: None,
                    undefined_element: None,
                }],
            }),
            heading1: None,
            heading2: None,
            heading3: None,
            heading4: None,
            heading5: None,
            heading6: None,
            heading7: None,
            heading8: None,
            heading9: None,
            bullet: None,
            ordered: None,
            code: None,
            quote: None,
            todo: None,
            bitable: None,
            callout: None,
            chat_card: None,
            diagram: None,
            divider: None,
            file: None,
            grid: None,
            grid_column: None,
            iframe: None,
            image: None,
            isv: None,
            mindnote: None,
            sheet: None,
            table: None,
            table_cell: None,
            view: None,
            jira_issue: None,
            okr: None,
            okr_objective: None,
            okr_key_result: None,
            okr_progress: None,
            board: None,
            agenda: None,
            agenda_item: None,
            agenda_item_title: None,
            agenda_item_content: None,
            link_preview: None,
            source_synced: None,
            reference_synced: None,
            sub_page_list: None,
            ai_template: None,
            undefined: None,
            quote_container: None,
            add_ons: None,
        };

        let json = serde_json::to_string_pretty(&block).unwrap();
        println!("Block JSON: {}", json);

        // 验证反序列化
        let deserialized: Block = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.block_id, "test_id");
        assert_eq!(deserialized.block_type, BlockType::Text);
    }
}