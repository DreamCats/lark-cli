use lark_cli::utils::{FileScanner, FileReader, ImportRequest};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

#[test]
fn test_file_scanner_new_with_pattern() {
    // 测试创建带模式的文件扫描器
    let scanner = FileScanner::new(Some("*.md")).unwrap();
    assert!(scanner.scan_directory(Path::new("nonexistent"), false).is_err());
}

#[test]
fn test_file_scanner_new_without_pattern() {
    // 测试创建不带模式的文件扫描器
    let scanner = FileScanner::new(None).unwrap();
    assert!(scanner.scan_directory(Path::new("nonexistent"), false).is_err());
}

#[test]
fn test_file_scanner_invalid_pattern() {
    // 测试无效的模式
    let result = FileScanner::new(Some("[invalid"));
    assert!(result.is_err());
}

#[test]
fn test_scan_directory_nonexistent() {
    // 测试扫描不存在的目录
    let scanner = FileScanner::new(None).unwrap();
    let result = scanner.scan_directory(Path::new("/nonexistent/path"), false);
    assert!(result.is_err());
}

#[test]
fn test_scan_directory_not_dir() {
    // 测试扫描非目录路径
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let scanner = FileScanner::new(None).unwrap();
    let result = scanner.scan_directory(temp_file.path(), false);
    assert!(result.is_err());
}

#[test]
fn test_scan_directory_empty() {
    // 测试扫描空目录
    let temp_dir = TempDir::new().unwrap();
    let scanner = FileScanner::new(None).unwrap();
    let files = scanner.scan_directory(temp_dir.path(), false).unwrap();
    assert_eq!(files.len(), 0);
}

#[test]
fn test_scan_directory_with_files() {
    // 测试扫描包含文件的目录
    let temp_dir = TempDir::new().unwrap();

    // 创建测试文件
    fs::write(temp_dir.path().join("test1.md"), "# Test 1").unwrap();
    fs::write(temp_dir.path().join("test2.txt"), "Test 2").unwrap();
    fs::write(temp_dir.path().join("test3.md"), "# Test 3").unwrap();

    // 不带模式扫描
    let scanner = FileScanner::new(None).unwrap();
    let files = scanner.scan_directory(temp_dir.path(), false).unwrap();
    assert_eq!(files.len(), 3);

    // 带模式扫描
    let scanner = FileScanner::new(Some("*.md")).unwrap();
    let files = scanner.scan_directory(temp_dir.path(), false).unwrap();
    assert_eq!(files.len(), 2);
}

#[test]
fn test_scan_directory_recursive() {
    // 测试递归扫描
    let temp_dir = TempDir::new().unwrap();
    let sub_dir = temp_dir.path().join("subdir");
    fs::create_dir(&sub_dir).unwrap();

    // 创建文件
    fs::write(temp_dir.path().join("root.md"), "# Root").unwrap();
    fs::write(sub_dir.join("sub.md"), "# Sub").unwrap();

    // 非递归扫描
    let scanner = FileScanner::new(None).unwrap();
    let files = scanner.scan_directory(temp_dir.path(), false).unwrap();
    assert_eq!(files.len(), 1);

    // 递归扫描
    let files = scanner.scan_directory(temp_dir.path(), true).unwrap();
    assert_eq!(files.len(), 2);
}

#[test]
fn test_file_reader_read_to_string() {
    // 测试读取文件内容
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), "Hello, World!").unwrap();

    let content = FileReader::read_to_string(temp_file.path()).unwrap();
    assert_eq!(content, "Hello, World!");
}

#[test]
fn test_file_reader_read_nonexistent() {
    // 测试读取不存在的文件
    let result = FileReader::read_to_string(Path::new("/nonexistent/file.txt"));
    assert!(result.is_err());
}

#[test]
fn test_infer_content_type() {
    // 测试内容类型推断
    assert_eq!(FileReader::infer_content_type(Path::new("test.md")), "markdown");
    assert_eq!(FileReader::infer_content_type(Path::new("test.markdown")), "markdown");
    assert_eq!(FileReader::infer_content_type(Path::new("test.html")), "html");
    assert_eq!(FileReader::infer_content_type(Path::new("test.htm")), "html");
    assert_eq!(FileReader::infer_content_type(Path::new("test.txt")), "markdown");
    assert_eq!(FileReader::infer_content_type(Path::new("test")), "markdown");
}

#[test]
fn test_import_request_creation() {
    // 测试创建导入请求
    let request = ImportRequest {
        file_path: PathBuf::from("test.md"),
        content: "# Test".to_string(),
        content_type: "markdown".to_string(),
        block_id: "block123".to_string(),
        index: 0,
        relative_path: Some("docs/test.md".to_string()),
    };

    assert_eq!(request.file_path, PathBuf::from("test.md"));
    assert_eq!(request.content, "# Test");
    assert_eq!(request.content_type, "markdown");
    assert_eq!(request.block_id, "block123");
    assert_eq!(request.index, 0);
    assert_eq!(request.relative_path, Some("docs/test.md".to_string()));
}

#[test]
fn test_batch_import_result() {
    // 测试批量导入结果
    let results = vec![
        ImportRequest {
            file_path: PathBuf::from("test1.md"),
            content: "# Test 1".to_string(),
            content_type: "markdown".to_string(),
            block_id: "block1".to_string(),
            index: 0,
            relative_path: None,
        },
        ImportRequest {
            file_path: PathBuf::from("test2.md"),
            content: "# Test 2".to_string(),
            content_type: "markdown".to_string(),
            block_id: "block2".to_string(),
            index: 1,
            relative_path: None,
        },
    ];

    // 这里只是测试结构创建，实际使用会在导入逻辑中
    assert_eq!(results.len(), 2);
}

#[test]
fn test_pattern_matching_edge_cases() {
    // 测试模式匹配的边界情况
    let temp_dir = TempDir::new().unwrap();

    // 创建各种文件名
    fs::write(temp_dir.path().join(".hidden.md"), "# Hidden").unwrap();
    fs::write(temp_dir.path().join("file with spaces.md"), "# Spaces").unwrap();
    fs::write(temp_dir.path().join("UPPERCASE.MD"), "# Upper").unwrap();

    // 测试通配符模式
    let scanner = FileScanner::new(Some("*.md")).unwrap();
    let files = scanner.scan_directory(temp_dir.path(), false).unwrap();

    // 注意：模式匹配是区分大小写的
    assert!(files.iter().any(|f| f.file_name().unwrap() == ".hidden.md"));
    assert!(files.iter().any(|f| f.file_name().unwrap() == "file with spaces.md"));

    // 检查大写扩展名是否匹配
    let has_uppercase = files.iter().any(|f| {
        f.file_name().unwrap().to_string_lossy().contains("UPPERCASE.MD")
    });
    // 这取决于 glob 模式的大小写敏感性
    println!("Found uppercase file: {}", has_uppercase);
}