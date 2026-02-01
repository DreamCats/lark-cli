use crate::api::CreateBoardNotesApi;
use crate::api::create_board_notes::CreateBoardNotesRequest;
use crate::error::Result;
use crate::output::OutputFormat;
use crate::output::format_output;
use std::fs;
use std::path::Path;

pub async fn handle_create_board_notes(
    api_client: crate::api::ApiClient,
    whiteboard_id: String,
    nodes_json: String,
    client_token: Option<String>,
    user_id_type: Option<String>,
    output_format: OutputFormat,
) -> Result<()> {
    let api = CreateBoardNotesApi::new(api_client);

    // 解析节点 JSON（支持直接 JSON 或文件路径）
    let raw_json = if Path::new(&nodes_json).is_file() {
        fs::read_to_string(&nodes_json).map_err(|e| {
            crate::error::LarkError::ParseError(format!("无法读取节点文件: {}", e))
        })?
    } else {
        nodes_json
    };
    let nodes_value: serde_json::Value = serde_json::from_str(&raw_json).map_err(|e| {
        crate::error::LarkError::ParseError(format!("无法解析节点 JSON: {}", e))
    })?;
    let nodes = match nodes_value {
        serde_json::Value::Array(nodes) => nodes,
        _ => {
            return Err(crate::error::LarkError::ParseError(
                "节点 JSON 必须是数组".to_string(),
            ));
        }
    };

    let request = CreateBoardNotesRequest { nodes };

    let result = api
        .create_board_notes(&whiteboard_id, request, client_token, user_id_type)
        .await?;

    let output = format_output(&result, output_format)?;
    println!("{}", output);
    Ok(())
}
