use crate::api::{ApiClient, PermissionApi};
use crate::error::Result;
use crate::output::OutputFormat;
use crate::output::format_output;

#[allow(clippy::too_many_arguments)]
pub async fn handle_add_permission(
    api_client: ApiClient,
    token: String,
    doc_type: String,
    member_type: String,
    member_id: String,
    perm: String,
    perm_type: Option<String>,
    collaborator_type: String,
    notification: bool,
    output_format: OutputFormat,
) -> Result<()> {
    let permission_api = PermissionApi::new(api_client);

    // 根据文档类型决定是否使用perm_type
    let effective_perm_type = if doc_type == "wiki" {
        perm_type  // 对于wiki文档，使用提供的perm_type
    } else {
        None  // 对于非wiki文档，不使用perm_type
    };

    let request = crate::api::permission::AddPermissionRequest {
        member_type,
        member_id,
        perm,
        perm_type: effective_perm_type,
        collaborator_type: Some(collaborator_type),
    };

    let result = permission_api.add_permission_member(
        &token,
        &doc_type,
        request,
        Some(notification)
    ).await?;

    let output = format_output(&result, output_format)?;
    println!("{}", output);
    Ok(())
}