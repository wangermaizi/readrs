//! 文件管理模块
//! 
//! 提供文件操作功能，包括：
//! - 文件新建、打开、保存
//! - 文件夹树视图
//! - 文档内搜索

mod file_operations;
mod file_tree;
mod search;

pub use file_operations::*;
pub use file_tree::*;
pub use search::*;

use std::path::PathBuf;

/// 文件类型枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileType {
    File,
    Directory,
}

/// 文件项结构
#[derive(Debug, Clone)]
pub struct FileItem {
    pub name: String,
    pub path: PathBuf,
    pub file_type: FileType,
    pub children: Vec<FileItem>,
    pub expanded: bool,
}

impl FileItem {
    /// 创建新的文件项
    pub fn new(name: String, path: PathBuf, file_type: FileType) -> Self {
        Self {
            name,
            path,
            file_type,
            children: Vec::new(),
            expanded: false,
        }
    }

    /// 判断是否为 Markdown 文件
    pub fn is_markdown(&self) -> bool {
        self.file_type == FileType::File
            && self.path.extension()
                .map(|ext| ext.to_string_lossy().to_lowercase())
                .map(|ext| ext == "md" || ext == "markdown")
                .unwrap_or(false)
    }
}
