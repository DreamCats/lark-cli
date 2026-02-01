use crate::api::{ApiClient, DocxApi};
use crate::error::Result;
use crate::output::{format_string_output, OutputFormat};

pub async fn handle_get_content(
    api_client: ApiClient,
    document_id: String,
    output_format: OutputFormat,
) -> Result<()> {
    let docx_api = DocxApi::new(api_client);
    let content = docx_api.get_document_raw_content(&document_id).await?;
    let output = format_string_output(&content, output_format)?;
    println!("{}", output);
    Ok(())
}