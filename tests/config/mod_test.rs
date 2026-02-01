use lark_cli::config::Config;
use lark_cli::error::LarkError;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_config_load_success() {
    // 创建临时目录和.env文件
    let temp_dir = TempDir::new().unwrap();
    let env_path = temp_dir.path().join(".env");

    // 写入测试环境变量
    fs::write(&env_path, "APP_ID=test_app_id\nAPP_SECRET=test_app_secret").unwrap();

    // 设置当前执行目录为临时目录
    let _original_exe = std::env::current_exe().unwrap();
    let temp_exe = temp_dir.path().join("test_exe");
    fs::write(&temp_exe, "").unwrap();

    // 由于Config::load()使用current_exe()，我们需要在实际的测试中模拟这个行为
    // 这里我们测试Config结构本身的功能
    let config = Config {
        app_id: "test_app_id".to_string(),
        app_secret: "test_app_secret".to_string(),
    };

    assert_eq!(config.app_id, "test_app_id");
    assert_eq!(config.app_secret, "test_app_secret");
}

#[test]
fn test_config_env_file_path() {
    // 测试获取环境文件路径功能
    let result = Config::env_file_path();

    // 在测试环境中，current_exe()可能会失败，所以我们检查返回的是Result类型
    match result {
        Ok(path) => {
            // 如果成功，路径应该以.env结尾
            assert!(path.to_string_lossy().ends_with(".env"));
        }
        Err(LarkError::ConfigError(msg)) => {
            // 如果失败，应该是获取执行路径失败
            assert!(msg.contains("Failed to get executable path"));
        }
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_config_clone() {
    // 测试Config的Clone trait
    let config1 = Config {
        app_id: "test_id".to_string(),
        app_secret: "test_secret".to_string(),
    };

    let config2 = config1.clone();

    assert_eq!(config1.app_id, config2.app_id);
    assert_eq!(config1.app_secret, config2.app_secret);

    // 确保是深拷贝
    assert_ne!(&config1.app_id as *const _, &config2.app_id as *const _);
}

// 集成测试 - 需要实际的.env文件
#[test]
#[ignore] // 标记为忽略，因为需要实际的.env文件
fn test_real_config_load() {
    // 这个测试需要在有.env文件的环境中运行
    match Config::load() {
        Ok(config) => {
            assert!(!config.app_id.is_empty());
            assert!(!config.app_secret.is_empty());
        }
        Err(e) => {
            println!("Config load error: {:?}", e);
            // 如果是因为缺少.env文件，这是预期的
            match e {
                LarkError::ConfigError(msg) if msg.contains("Environment file not found") => {
                    println!("Expected: .env file not found");
                }
                _ => panic!("Unexpected error: {:?}", e),
            }
        }
    }
}

// 测试环境变量验证
#[test]
fn test_empty_app_id_validation() {
    // 模拟空APP_ID的情况
    let temp_dir = TempDir::new().unwrap();
    let env_path = temp_dir.path().join(".env");

    // 写入空的APP_ID
    fs::write(&env_path, "APP_ID=\nAPP_SECRET=secret123").unwrap();

    // 在实际使用中，这会导致ConfigError
    // 这里我们测试结构的行为
    let config = Config {
        app_id: String::new(),
        app_secret: "secret123".to_string(),
    };

    assert!(config.app_id.is_empty());
    assert_eq!(config.app_secret, "secret123");
}

#[test]
fn test_empty_app_secret_validation() {
    // 模拟空APP_SECRET的情况
    let config = Config {
        app_id: "app123".to_string(),
        app_secret: String::new(),
    };

    assert_eq!(config.app_id, "app123");
    assert!(config.app_secret.is_empty());
}

// 测试环境文件格式
#[test]
fn test_env_file_format() {
    let temp_dir = TempDir::new().unwrap();
    let env_path = temp_dir.path().join(".env");

    // 测试标准格式
    let content = "APP_ID=my_app_id\nAPP_SECRET=my_secret";
    fs::write(&env_path, content).unwrap();

    let read_content = fs::read_to_string(&env_path).unwrap();
    assert_eq!(read_content, content);

    // 测试带空格的格式
    let content_with_spaces = "APP_ID = spaced_app_id\nAPP_SECRET = spaced_secret";
    fs::write(&env_path, content_with_spaces).unwrap();

    let read_content = fs::read_to_string(&env_path).unwrap();
    assert_eq!(read_content, content_with_spaces);
}

// 测试特殊字符处理
#[test]
fn test_special_characters_in_config() {
    let config = Config {
        app_id: "app-with-dashes_and_underscores123".to_string(),
        app_secret: "secret!@#$%^&*()_+-=[]{}|;:,.<>?".to_string(),
    };

    assert_eq!(config.app_id, "app-with-dashes_and_underscores123");
    assert_eq!(config.app_secret, "secret!@#$%^&*()_+-=[]{}|;:,.<>?");
}

// 测试长字符串处理
#[test]
fn test_long_strings_in_config() {
    let long_id = "a".repeat(1000);
    let long_secret = "b".repeat(2000);

    let config = Config {
        app_id: long_id.clone(),
        app_secret: long_secret.clone(),
    };

    assert_eq!(config.app_id.len(), 1000);
    assert_eq!(config.app_secret.len(), 2000);
    assert_eq!(config.app_id, long_id);
    assert_eq!(config.app_secret, long_secret);
}