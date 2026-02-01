use crate::api::{ApiClient, DocumentApi};
use crate::error::Result;
use crate::output::OutputFormat;
use crate::output::format_output;

pub async fn handle_create_document(
    api_client: ApiClient,
    folder_token: Option<String>,
    title: Option<String>,
    output_format: OutputFormat,
) -> Result<()> {
    let document_api = DocumentApi::new(api_client);
    let document = document_api.create_document(folder_token, title).await?;
    let output = format_output(&document, output_format)?;
    println!("{}", output);
    Ok(())
}