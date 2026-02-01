use lark_cli::output::{format_output, format_string_output, OutputFormat};
use serde::Serialize;
use std::collections::HashMap;

// 测试用的简单结构体
#[derive(Debug, Serialize, PartialEq)]
struct TestData {
    name: String,
    value: i32,
}

// 测试用的复杂结构体
#[derive(Debug, Serialize)]
struct ComplexData {
    id: u64,
    items: Vec<String>,
    metadata: HashMap<String, String>,
}

#[test]
fn test_format_output_text_simple_struct() {
    let data = TestData {
        name: "test".to_string(),
        value: 42,
    };

    let result = format_output(&data, OutputFormat::Text).unwrap();

    // 文本格式应该包含Debug格式的输出
    assert!(result.contains("TestData"));
    assert!(result.contains("name: \"test\""));
    assert!(result.contains("value: 42"));
}

#[test]
fn test_format_output_json_simple_struct() {
    let data = TestData {
        name: "test".to_string(),
        value: 42,
    };

    let result = format_output(&data, OutputFormat::Json).unwrap();

    // JSON格式应该是有效的JSON
    assert!(result.contains("\"name\": \"test\""));
    assert!(result.contains("\"value\": 42"));

    // 验证是格式化的JSON
    assert!(result.contains("{\n"));
    assert!(result.contains("\n}"));
}

#[test]
fn test_format_output_text_complex_struct() {
    let mut metadata = HashMap::new();
    metadata.insert("key1".to_string(), "value1".to_string());
    metadata.insert("key2".to_string(), "value2".to_string());

    let data = ComplexData {
        id: 12345,
        items: vec!["item1".to_string(), "item2".to_string()],
        metadata,
    };

    let result = format_output(&data, OutputFormat::Text).unwrap();

    // 验证文本格式包含所有字段
    assert!(result.contains("ComplexData"));
    assert!(result.contains("id: 12345"));
    assert!(result.contains("items:"));
    assert!(result.contains("\"item1\""));
    assert!(result.contains("\"item2\""));
    assert!(result.contains("metadata:"));
    assert!(result.contains("\"key1\": \"value1\""));
}

#[test]
fn test_format_output_json_complex_struct() {
    let mut metadata = HashMap::new();
    metadata.insert("key1".to_string(), "value1".to_string());
    metadata.insert("key2".to_string(), "value2".to_string());

    let data = ComplexData {
        id: 12345,
        items: vec!["item1".to_string(), "item2".to_string()],
        metadata,
    };

    let result = format_output(&data, OutputFormat::Json).unwrap();

    // 验证JSON格式
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["id"], 12345);
    assert_eq!(parsed["items"].as_array().unwrap().len(), 2);
    assert_eq!(parsed["metadata"]["key1"], "value1");
}

#[test]
fn test_format_string_output_text() {
    let data = "Hello, World!";

    let result = format_string_output(data, OutputFormat::Text).unwrap();

    // 文本格式应该直接返回字符串
    assert_eq!(result, "Hello, World!");
}

#[test]
fn test_format_string_output_json() {
    let data = "Hello, World!";

    let result = format_string_output(data, OutputFormat::Json).unwrap();

    // JSON格式应该将字符串序列化为JSON
    assert_eq!(result, "\"Hello, World!\"");
}

#[test]
fn test_format_string_output_special_chars() {
    let data = "Line 1\nLine 2\tTabbed\"Quoted\"";

    let result = format_string_output(data, OutputFormat::Json).unwrap();

    // 特殊字符应该被正确转义
    assert!(result.contains("\\n"));
    assert!(result.contains("\\t"));
    assert!(result.contains("\\\""));
}

#[test]
fn test_format_output_empty_struct() {
    #[derive(Debug, Serialize)]
    struct EmptyStruct;

    let data = EmptyStruct;

    let text_result = format_output(&data, OutputFormat::Text).unwrap();
    let json_result = format_output(&data, OutputFormat::Json).unwrap();

    assert!(text_result.contains("EmptyStruct"));
    assert_eq!(json_result, "null");
}

#[test]
fn test_format_output_vec_data() {
    let data = vec![
        TestData {
            name: "first".to_string(),
            value: 1,
        },
        TestData {
            name: "second".to_string(),
            value: 2,
        },
    ];

    let text_result = format_output(&data, OutputFormat::Text).unwrap();
    let json_result = format_output(&data, OutputFormat::Json).unwrap();

    // 验证文本格式
    assert!(text_result.contains("TestData"));
    assert!(text_result.contains("first"));
    assert!(text_result.contains("second"));

    // 验证JSON格式
    let parsed: serde_json::Value = serde_json::from_str(&json_result).unwrap();
    assert_eq!(parsed.as_array().unwrap().len(), 2);
}

#[test]
fn test_format_output_hashmap_data() {
    let mut data = HashMap::new();
    data.insert("key1", TestData {
        name: "test1".to_string(),
        value: 100,
    });
    data.insert("key2", TestData {
        name: "test2".to_string(),
        value: 200,
    });

    let text_result = format_output(&data, OutputFormat::Text).unwrap();
    let json_result = format_output(&data, OutputFormat::Json).unwrap();

    // 验证两种格式都包含数据
    assert!(text_result.contains("key1"));
    assert!(text_result.contains("key2"));
    assert!(text_result.contains("test1"));
    assert!(text_result.contains("test2"));

    let parsed: serde_json::Value = serde_json::from_str(&json_result).unwrap();
    assert_eq!(parsed["key1"]["name"], "test1");
    assert_eq!(parsed["key2"]["value"], 200);
}

#[test]
fn test_format_output_unicode_data() {
    #[derive(Debug, Serialize)]
    struct UnicodeData {
        chinese: String,
        emoji: String,
        special: String,
    }

    let data = UnicodeData {
        chinese: "你好世界".to_string(),
        emoji: "🦀🚀✨".to_string(),
        special: " café ".to_string(),
    };

    let text_result = format_output(&data, OutputFormat::Text).unwrap();
    let json_result = format_output(&data, OutputFormat::Json).unwrap();

    // 验证Unicode字符正确处理
    assert!(text_result.contains("你好世界"));
    assert!(text_result.contains("🦀🚀✨"));
    assert!(text_result.contains("café"));

    let parsed: serde_json::Value = serde_json::from_str(&json_result).unwrap();
    assert_eq!(parsed["chinese"], "你好世界");
    assert_eq!(parsed["emoji"], "🦀🚀✨");
}

#[test]
fn test_format_string_output_empty() {
    let data = "";

    let text_result = format_string_output(data, OutputFormat::Text).unwrap();
    let json_result = format_string_output(data, OutputFormat::Json).unwrap();

    assert_eq!(text_result, "");
    assert_eq!(json_result, "\"\"");
}

#[test]
fn test_format_string_output_json_parse_error() {
    // 这个测试验证JSON解析错误的情况
    // 由于format_string_output对任何字符串都应该成功，
    // 我们测试一个理论上可能导致问题的场景

    let data = "Valid string";
    let result = format_string_output(data, OutputFormat::Json);

    // 应该总是成功
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\"Valid string\"");
}

// 性能测试 - 大数据量
#[test]
fn test_format_output_large_data() {
    let large_vec: Vec<TestData> = (0..1000).map(|i| TestData {
        name: format!("item_{}", i),
        value: i,
    }).collect();

    let text_result = format_output(&large_vec, OutputFormat::Text).unwrap();
    let json_result = format_output(&large_vec, OutputFormat::Json).unwrap();

    // 验证大数据量处理
    assert!(text_result.contains("item_0"));
    assert!(text_result.contains("item_999"));

    let parsed: serde_json::Value = serde_json::from_str(&json_result).unwrap();
    assert_eq!(parsed.as_array().unwrap().len(), 1000);
}