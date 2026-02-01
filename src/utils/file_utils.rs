use crate::error::Result;
use glob::Pattern;
use serde::Serialize;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// 文件扫描器，用于查找匹配条件的文件
#[derive(Debug)]
pub struct FileScanner {
    pattern: Option<Pattern>,
}

impl FileScanner {
    /// 创建新的文件扫描器
    pub fn new(pattern: Option<&str>) -> Result<Self> {
        let pattern = match pattern {
            Some(p) => Some(Pattern::new(p).map_err(|e| {
                crate::error::LarkError::ValidationError(
                    format!("无效的模式 '{}': {}", p, e)
                )
            })?),
            None => None,
        };

        Ok(Self { pattern })
    }

    /// 扫描目录，返回匹配的文件列表
    pub fn scan_directory(&self, path: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
        if !path.exists() {
            return Err(crate::error::LarkError::ValidationError(
                format!("路径不存在: {}", path.display())
            ));
        }

        if !path.is_dir() {
            return Err(crate::error::LarkError::ValidationError(
                format!("路径不是目录: {}", path.display())
            ));
        }

        let mut files = Vec::new();

        if recursive {
            // 递归扫描
            for entry in WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                if self.matches_pattern(entry.path()) {
                    files.push(entry.path().to_path_buf());
                }
            }
        } else {
            // 非递归扫描
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() && self.matches_pattern(&path) {
                    files.push(path);
                }
            }
        }

        files.sort();
        Ok(files)
    }

    /// 检查文件路径是否匹配模式
    fn matches_pattern(&self, path: &Path) -> bool {
        match &self.pattern {
            Some(pattern) => {
                // 检查文件名
                if let Some(file_name) = path.file_name() {
                    if let Some(name_str) = file_name.to_str() {
                        if pattern.matches(name_str) {
                            return true;
                        }
                    }
                }
                // 检查完整路径
                pattern.matches_path(path)
            }
            None => true,
        }
    }
}

/// 文件读取器，用于读取文件内容
pub struct FileReader;

impl FileReader {
    /// 读取文件内容为字符串
    pub fn read_to_string(path: &Path) -> Result<String> {
        std::fs::read_to_string(path).map_err(|e| {
            crate::error::LarkError::ValidationError(
                format!("读取文件失败 {}: {}", path.display(), e)
            )
        })
    }

    /// 根据文件扩展名推断内容类型
    pub fn infer_content_type(path: &Path) -> &'static str {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("md") | Some("markdown") => "markdown",
            Some("html") | Some("htm") => "html",
            _ => "markdown", // 默认为 markdown
        }
    }
}

/// 导入请求，表示一个文件的导入任务
#[derive(Debug)]
pub struct ImportRequest {
    /// 文件路径
    pub file_path: PathBuf,
    /// 文件内容
    pub content: String,
    /// 内容类型
    pub content_type: String,
    /// 目标块 ID
    pub block_id: String,
    /// 插入位置
    pub index: i32,
    /// 相对路径（用于保持目录结构）
    #[allow(dead_code)]
    pub relative_path: Option<String>,
}

/// 批量导入结果
#[derive(Debug, Serialize)]
pub struct BatchImportResult {
    /// 成功导入的文件数
    pub success_count: usize,
    /// 失败的文件数
    pub failure_count: usize,
    /// 跳过的文件数
    pub skipped_count: usize,
    /// 详细结果
    pub results: Vec<ImportResult>,
}

/// 单个文件导入结果
#[derive(Debug, Serialize)]
pub struct ImportResult {
    /// 文件路径
    pub file_path: PathBuf,
    /// 是否成功
    pub success: bool,
    /// 错误信息（如果有）
    pub error: Option<String>,
    /// 创建的块 ID（如果成功）
    pub block_ids: Option<Vec<String>>,
}