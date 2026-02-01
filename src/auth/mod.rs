use std::time::Duration;
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::error::{LarkError, Result};

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    tenant_access_token: String,
    expire: i64,
}

#[derive(Clone)]
pub struct AuthManager {
    config: Config,
    client: reqwest::Client,
}

impl AuthManager {
    pub fn new(config: Config) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { config, client }
    }

    /// Get a new tenant_access_token
    pub async fn get_token(&self) -> Result<String> {
        tracing::info!("Getting tenant_access_token");

        let url = "https://open.larkoffice.com/open-apis/auth/v3/tenant_access_token/internal";

        let request_body = serde_json::json!({
            "app_id": self.config.app_id,
            "app_secret": self.config.app_secret
        });

        tracing::debug!("Sending auth request to: {}", url);

        let response = self.client
            .post(url)
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LarkError::AuthError(format!(
                "Auth request failed, status code: {}",
                response.status()
            )));
        }

        let token_response: TokenResponse = response.json().await
            .map_err(|e| LarkError::ParseError(format!("Failed to parse auth response: {}, may be due to invalid app_id or app_secret", e)))?;

        if token_response.tenant_access_token.is_empty() {
            return Err(LarkError::AuthError("tenant_access_token in response is empty".to_string()));
        }

        tracing::info!("tenant_access_token retrieved successfully");

        Ok(token_response.tenant_access_token)
    }

    /// Get authorization header
    pub async fn get_auth_header(&self) -> Result<String> {
        let token = self.get_token().await?;
        Ok(format!("Bearer {}", token))
    }
}