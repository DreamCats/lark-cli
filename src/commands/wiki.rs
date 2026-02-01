use crate::api::{ApiClient, WikiApi};
use crate::error::Result;
use crate::output::OutputFormat;
use crate::output::format_output;

pub async fn handle_get_node(
    api_client: ApiClient,
    token: String,
    obj_type: Option<String>,
    output_format: OutputFormat,
) -> Result<()> {
    let wiki_api = WikiApi::new(api_client);
    let node = wiki_api.get_knowledge_space_node(&token, obj_type.as_deref()).await?;
    let output = format_output(&node, output_format)?;
    println!("{}", output);
    Ok(())
}