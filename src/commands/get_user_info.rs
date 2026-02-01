use crate::api::GetUserInfoApi;
use crate::output::OutputFormat;
use crate::Result;

pub async fn handle_get_user_info(
    api_client: crate::api::ApiClient,
    user_id: String,
    user_id_type: Option<String>,
    department_id_type: Option<String>,
    output_format: OutputFormat,
) -> Result<()> {
    // 创建 API 实例
    let api = GetUserInfoApi::new(api_client);

    // 调用 API
    let user_info = api
        .get_user_info(
            &user_id,
            user_id_type.as_deref(),
            department_id_type.as_deref(),
        )
        .await?;

    // 输出结果
    match output_format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&user_info)?);
        }
        OutputFormat::Text => {
            println!("{:#?}", user_info);
        }
    }

    Ok(())
}
