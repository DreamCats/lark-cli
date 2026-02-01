use crate::api::GetBoardImageApi;
use crate::error::Result;
use crate::output::OutputFormat;
use crate::output::format_output;

pub async fn handle_get_board_image(
    api_client: crate::api::ApiClient,
    whiteboard_id: String,
    output_path: String,
    output_format: OutputFormat,
) -> Result<()> {
    let api = GetBoardImageApi::new(api_client);

    // 如果 output_path 是目录，使用自动命名
    let result = if output_path.ends_with('/') || output_path.ends_with('\\') {
        api.get_board_image_auto_name(&whiteboard_id, &output_path)
            .await?
    } else {
        api.get_board_image(&whiteboard_id, &output_path)
            .await?
    };

    let output = format_output(&result, output_format)?;
    println!("{}", output);
    Ok(())
}
