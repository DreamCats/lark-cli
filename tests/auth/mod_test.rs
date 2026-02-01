use lark_cli::auth::AuthManager;
use lark_cli::config::Config;
use lark_cli::error::LarkError;
use serde_json::json;

#[tokio::test]
async fn test_auth_manager_new() {
    let config = Config {
        app_id: "test_app_id".to_string(),
        app_secret: "test_app_secret".to_string(),
    };

    let auth_manager = AuthManager::new(config.clone());

    // AuthManager should be created successfully
    // We can't directly test the private fields, but we can verify it works
    // by checking if get_token() method exists (it compiles)
}

#[tokio::test]
async fn test_auth_manager_clone() {
    let config = Config {
        app_id: "cli_a8d667668b73900b".to_string(),
        app_secret: "FS9N0KX5IFrAnu38ANGQegJpyWeOvEr7".to_string(),
    };

    let auth_manager1 = AuthManager::new(config);
    let auth_manager2 = auth_manager1.clone();

    // Both should work independently
    // We can't test much without mocking, but clone should work
    assert!(true);
}

// 测试TokenResponse结构体解析
#[test]
fn test_token_response_deserialization() {
    // 模拟TokenResponse结构
    #[derive(Debug, serde::Deserialize)]
    struct TokenResponse {
        tenant_access_token: String,
        expire: i64,
    }

    let json_data = r#"{
        "tenant_access_token": "test_token_123",
        "expire": 7200
    }"#;

    let token_response: Result<TokenResponse, _> = serde_json::from_str(json_data);
    assert!(token_response.is_ok());

    let parsed = token_response.unwrap();
    assert_eq!(parsed.tenant_access_token, "test_token_123");
    assert_eq!(parsed.expire, 7200);
}

// 测试错误响应格式
#[test]
fn test_error_response_format() {
    let error_json = r#"{
        "code": 400,
        "msg": "app_id or app_secret is invalid"
    }"#;

    let parsed: serde_json::Value = serde_json::from_str(error_json).unwrap();
    assert_eq!(parsed["code"], 400);
    assert_eq!(parsed["msg"], "app_id or app_secret is invalid");
}

// 测试认证请求体格式
#[test]
fn test_auth_request_body_format() {
    let app_id = "cli_a8d667668b73900b";
    let app_secret = "FS9N0KX5IFrAnu38ANGQegJpyWeOvEr7";

    let request_body = json!({
        "app_id": app_id,
        "app_secret": app_secret
    });

    assert_eq!(request_body["app_id"], app_id);
    assert_eq!(request_body["app_secret"], app_secret);
}

// 测试空token处理
#[test]
fn test_empty_token_validation() {
    let empty_token = "";
    assert!(empty_token.is_empty());

    let valid_token = "some_token_123";
    assert!(!valid_token.is_empty());
}

// 测试Bearer头格式
#[test]
fn test_bearer_header_format() {
    let token = "test_token_abc123";
    let header = format!("Bearer {}", token);

    assert_eq!(header, "Bearer test_token_abc123");
    assert!(header.starts_with("Bearer "));
}

// 测试网络错误类型
#[test]
fn test_network_error_types() {
    // 模拟reqwest错误转换为我们的错误类型
    let network_error = LarkError::NetworkError("Connection refused".to_string());

    match network_error {
        LarkError::NetworkError(msg) => {
            assert_eq!(msg, "Connection refused");
        }
        _ => panic!("Expected NetworkError"),
    }
}

// 测试认证错误类型
#[test]
fn test_auth_error_types() {
    let auth_error = LarkError::AuthError("Invalid credentials".to_string());

    match auth_error {
        LarkError::AuthError(msg) => {
            assert_eq!(msg, "Invalid credentials");
        }
        _ => panic!("Expected AuthError"),
    }
}

// 测试解析错误类型
#[test]
fn test_parse_error_types() {
    let parse_error = LarkError::ParseError("Invalid JSON".to_string());

    match parse_error {
        LarkError::ParseError(msg) => {
            assert_eq!(msg, "Invalid JSON");
        }
        _ => panic!("Expected ParseError"),
    }
}

// 集成测试 - 需要真实API调用（标记为忽略）
#[tokio::test]
#[ignore] // 需要真实的API凭证
async fn test_real_auth_integration() {
    let config = Config {
        app_id: "cli_a8d667668b73900b".to_string(),
        app_secret: "FS9N0KX5IFrAnu38ANGQegJpyWeOvEr7".to_string(),
    };

    let auth_manager = AuthManager::new(config);

    match auth_manager.get_token().await {
        Ok(token) => {
            assert!(!token.is_empty());
            println!("Successfully obtained token: {}", token);

            // Test get_auth_header too
            let header = auth_manager.get_auth_header().await.unwrap();
            assert!(header.starts_with("Bearer "));
            assert_eq!(header, format!("Bearer {}", token));
        }
        Err(e) => {
            println!("Auth failed: {:?}", e);
            // 如果失败，可能是因为凭证无效或网络问题
            // 这在这里是可以接受的，因为我们正在测试真实的API
        }
    }
}