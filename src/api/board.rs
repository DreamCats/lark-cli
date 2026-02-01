use crate::api::ApiClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ImportDiagramRequest {
    pub plant_uml_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_type: Option<i32>,
    pub syntax_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagram_type: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImportDiagramResponse {
    pub node_id: String,
}

pub struct BoardApi {
    client: ApiClient,
}

impl BoardApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    pub async fn import_diagram(
        &self,
        whiteboard_id: &str,
        request: ImportDiagramRequest,
    ) -> Result<ImportDiagramResponse> {
        let url = format!(
            "https://open.larkoffice.com/open-apis/board/v1/whiteboards/{}/nodes/plantuml",
            whiteboard_id
        );

        let response: ImportDiagramResponse = self.client.post(&url, &request).await?;
        Ok(response)
    }
}