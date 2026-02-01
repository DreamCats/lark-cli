use crate::api::{ApiClient, MessageApi, SearchChatsApi, GetMessageHistoryApi};
use crate::error::Result;
use crate::output::OutputFormat;
use crate::output::format_output;

pub async fn handle_send_message(
    api_client: ApiClient,
    receive_id: String,
    receive_id_type: String,
    msg_type: String,
    content: String,
    uuid: Option<String>,
    output_format: OutputFormat,
) -> Result<()> {
    let message_api = MessageApi::new(api_client);
    let result = message_api.send_message(&receive_id, &receive_id_type, &msg_type, &content, uuid).await?;
    let output = format_output(&result, output_format)?;
    println!("{}", output);
    Ok(())
}

pub async fn handle_search_chats(
    api_client: ApiClient,
    user_id_type: String,
    query: Option<String>,
    page_token: Option<String>,
    page_size: i32,
    output_format: OutputFormat,
) -> Result<()> {
    let search_chats_api = SearchChatsApi::new(api_client);
    let result = search_chats_api.search_chats(
        Some(user_id_type),
        query,
        page_token,
        Some(page_size),
    ).await?;
    let output = format_output(&result, output_format)?;
    println!("{}", output);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn handle_get_message_history(
    api_client: ApiClient,
    container_id_type: String,
    container_id: String,
    start_time: Option<String>,
    end_time: Option<String>,
    sort_type: String,
    page_size: i32,
    page_token: Option<String>,
    output_format: OutputFormat,
) -> Result<()> {
    let get_message_history_api = GetMessageHistoryApi::new(api_client);
    let result = get_message_history_api.get_message_history(
        container_id_type,
        container_id,
        start_time,
        end_time,
        Some(sort_type),
        Some(page_size),
        page_token,
    ).await?;
    let output = format_output(&result, output_format)?;
    println!("{}", output);
    Ok(())
}