use crate::api::{ApiClient, BoardApi};
use crate::error::Result;
use crate::output::{OutputFormat, format_output};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum DiagramSyntax {
    PlantUml,
    Mermaid,
}

impl std::str::FromStr for DiagramSyntax {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "plantuml" | "plant-uml" | "puml" => Ok(DiagramSyntax::PlantUml),
            "mermaid" | "mmd" => Ok(DiagramSyntax::Mermaid),
            _ => Err(format!("无效的图表语法类型: {}。可选值: plantuml, mermaid", s)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DiagramType {
    Auto,
    MindMap,
    Sequence,
    Activity,
    Class,
    ER,
    Flowchart,
    UseCase,
    Component,
}

impl DiagramType {
    fn to_plantuml_value(&self) -> i32 {
        match self {
            DiagramType::Auto => 0,
            DiagramType::MindMap => 1,
            DiagramType::Sequence => 2,
            DiagramType::Activity => 3,
            DiagramType::Class => 4,
            DiagramType::ER => 5,
            DiagramType::Flowchart => 6,
            DiagramType::UseCase => 7,
            DiagramType::Component => 8,
        }
    }
}

impl std::str::FromStr for DiagramType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "auto" | "0" => Ok(DiagramType::Auto),
            "mindmap" | "mind-map" | "1" => Ok(DiagramType::MindMap),
            "sequence" | "时序图" | "2" => Ok(DiagramType::Sequence),
            "activity" | "活动图" | "3" => Ok(DiagramType::Activity),
            "class" | "类图" | "4" => Ok(DiagramType::Class),
            "er" | "实体关系" | "5" => Ok(DiagramType::ER),
            "flowchart" | "流程图" | "6" => Ok(DiagramType::Flowchart),
            "usecase" | "用例图" | "7" => Ok(DiagramType::UseCase),
            "component" | "组件图" | "8" => Ok(DiagramType::Component),
            _ => Err(format!("无效的图表类型: {}。可选值: auto, mindmap, sequence, activity, class, er, flowchart, usecase, component", s)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StyleType {
    BoardStyle,
    ClassicStyle,
}

impl StyleType {
    fn to_value(&self) -> i32 {
        match self {
            StyleType::BoardStyle => 1,
            StyleType::ClassicStyle => 2,
        }
    }
}

impl std::str::FromStr for StyleType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "board" | "1" => Ok(StyleType::BoardStyle),
            "classic" | "2" => Ok(StyleType::ClassicStyle),
            _ => Err(format!("无效的样式类型: {}。可选值: board(1), classic(2)", s)),
        }
    }
}

pub async fn handle_import_diagram(
    api_client: ApiClient,
    whiteboard_id: String,
    source: String,
    source_type: String,
    syntax: DiagramSyntax,
    diagram_type: DiagramType,
    style: StyleType,
    output_format: OutputFormat,
) -> Result<()> {
    let board_api = BoardApi::new(api_client);

    // 获取图表代码
    let diagram_code = match source_type.as_str() {
        "file" => {
            let path = Path::new(&source);
            if !path.exists() {
                return Err(crate::error::LarkError::ConfigError(format!("文件不存在: {}", source)));
            }
            fs::read_to_string(path)
                .map_err(|e| crate::error::LarkError::ConfigError(format!("读取文件失败: {}", e)))?
        }
        "content" => source,
        _ => {
            return Err(crate::error::LarkError::ParseError(format!(
                "无效的源类型: {}。可选值: file, content",
                source_type
            )));
        }
    };

    // 验证代码长度
    if diagram_code.len() > 1_000_000 {
        return Err(crate::error::LarkError::ConfigError(
            "图表代码长度超过最大限制（100万字符）".to_string()
        ));
    }

    if diagram_code.is_empty() {
        return Err(crate::error::LarkError::ConfigError(
            "图表代码不能为空".to_string()
        ));
    }

    // 构建请求
    let request = crate::api::board::ImportDiagramRequest {
        plant_uml_code: diagram_code,
        style_type: Some(style.to_value()),
        syntax_type: match syntax {
            DiagramSyntax::PlantUml => 1,
            DiagramSyntax::Mermaid => 2,
        },
        diagram_type: Some(diagram_type.to_plantuml_value()),
    };

    // 调用API
    let result = board_api.import_diagram(&whiteboard_id, request).await?;

    // 格式化输出
    let output = format_output(&result, output_format)?;
    println!("{}", output);

    Ok(())
}