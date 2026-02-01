use crate::api::DownloadMediaApi;
use crate::error::Result;
use crate::output::OutputFormat;
use crate::output::format_output;

pub async fn handle_download_media(
    api_client: crate::api::ApiClient,
    file_token: String,
    output_path: String,
    extra: Option<String>,
    range: Option<String>,
    output_format: OutputFormat,
) -> Result<()> {
    let download_api = DownloadMediaApi::new(api_client);

    // 如果 output_path 是目录，使用自动命名
    let result = if output_path.ends_with('/') || output_path.ends_with('\\') {
        download_api
            .download_media_auto_name(&file_token, &output_path, extra, range)
            .await?
    } else {
        download_api
            .download_media(&file_token, &output_path, extra, range)
            .await?
    };

    let output = format_output(&result, output_format)?;
    println!("{}", output);
    Ok(())
}
