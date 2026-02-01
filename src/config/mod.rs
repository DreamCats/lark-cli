use std::path::PathBuf;
use crate::error::{LarkError, Result};

/// 用户级配置目录名称
const USER_CONFIG_DIR: &str = ".lark-cli";
/// 环境变量文件名
const ENV_FILE_NAME: &str = ".env";
/// 应用 ID 环境变量名
const ENV_APP_ID: &str = "APP_ID";
/// 应用密钥环境变量名
const ENV_APP_SECRET: &str = "APP_SECRET";

#[derive(Clone)]
pub struct Config {
    pub app_id: String,
    pub app_secret: String,
}

impl Config {
    /// Load configuration from .env file in the same directory as the executable
    /// or from user-level directory as fallback
    pub fn load() -> Result<Self> {
        // Get executable path
        let exe_path = std::env::current_exe()
            .map_err(|e| LarkError::ConfigError(format!("Failed to get executable path: {}", e)))?;

        // Get executable directory
        let exe_dir = exe_path.parent()
            .ok_or_else(|| LarkError::ConfigError("Cannot get executable directory".to_string()))?;

        // Build .env file paths
        let exe_env_path = exe_dir.join(ENV_FILE_NAME);
        let user_env_path = dirs::home_dir()
            .map(|home| home.join(USER_CONFIG_DIR).join(ENV_FILE_NAME))
            .ok_or_else(|| LarkError::ConfigError("Cannot get user home directory".to_string()))?;

        let mut env_loaded = false;
        let mut loaded_path = exe_env_path.clone();

        // Try to load from executable directory first
        if exe_env_path.exists() {
            match dotenvy::from_path(&exe_env_path) {
                Ok(_) => {
                    tracing::debug!("Loaded .env file from executable directory: {}", exe_env_path.display());
                    env_loaded = true;
                }
                Err(e) => {
                    tracing::warn!("Failed to load .env from executable directory: {}", e);
                }
            }
        }

        // If not loaded from executable directory, try user-level directory
        if !env_loaded && user_env_path.exists() {
            match dotenvy::from_path(&user_env_path) {
                Ok(_) => {
                    tracing::debug!("Loaded .env file from user directory: {}", user_env_path.display());
                    loaded_path = user_env_path.clone();
                    env_loaded = true;
                }
                Err(e) => {
                    tracing::warn!("Failed to load .env from user directory: {}", e);
                }
            }
        }

        // If no .env file found in either location
        if !env_loaded {
            return Err(LarkError::ConfigError(format!(
                "Environment file not found in either location:\n\
                1. {} (executable directory)\n\
                2. {} (user directory: ~/{}/{})\n\
                \n\
                Please create a .env file in one of these locations with {} and {}",
                exe_env_path.display(),
                user_env_path.display(),
                USER_CONFIG_DIR,
                ENV_FILE_NAME,
                ENV_APP_ID,
                ENV_APP_SECRET
            )));
        }

        // Read environment variables
        let app_id = std::env::var(ENV_APP_ID)
            .map_err(|_| LarkError::ConfigError(format!("Environment variable {} not set", ENV_APP_ID)))?;
        let app_secret = std::env::var(ENV_APP_SECRET)
            .map_err(|_| LarkError::ConfigError(format!("Environment variable {} not set", ENV_APP_SECRET)))?;

        // Validate environment variables are not empty
        if app_id.is_empty() {
            return Err(LarkError::ConfigError(format!("{} cannot be empty", ENV_APP_ID)));
        }
        if app_secret.is_empty() {
            return Err(LarkError::ConfigError(format!("{} cannot be empty", ENV_APP_SECRET)));
        }

        tracing::debug!("Configuration loaded successfully, env file: {}", loaded_path.display());

        Ok(Config { app_id, app_secret })
    }

    /// Get the path to the environment file
    #[allow(dead_code)]
    pub fn env_file_path() -> Result<PathBuf> {
        let exe_path = std::env::current_exe()
            .map_err(|e| LarkError::ConfigError(format!("Failed to get executable path: {}", e)))?;
        let exe_dir = exe_path.parent()
            .ok_or_else(|| LarkError::ConfigError("Cannot get executable directory".to_string()))?;
        Ok(exe_dir.join(ENV_FILE_NAME))
    }
}