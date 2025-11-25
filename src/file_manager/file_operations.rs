//! 文件操作模块
//!
//! 提供文件操作功能：
//! - 新建文件
//! - 打开文件
//! - 保存文件
//! - 另存为

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};

/// 文件操作管理器
pub struct FileManager {
    /// 当前打开的文件路径
    current_file: Option<PathBuf>,
    /// 文件内容
    content: String,
    /// 是否已修改
    is_modified: bool,
}

impl FileManager {
    /// 创建新的文件管理器
    pub fn new() -> Self {
        Self {
            current_file: None,
            content: String::new(),
            is_modified: false,
        }
    }

    /// 获取当前文件路径
    pub fn current_file(&self) -> Option<&Path> {
        self.current_file.as_deref()
    }

    /// 获取文件名（用于显示）
    pub fn current_filename(&self) -> String {
        self.current_file
            .as_ref()
            .and_then(|path| path.file_name())
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| "未命名".to_string())
    }

    /// 获取文件内容
    pub fn content(&self) -> &str {
        &self.content
    }

    /// 判断文件是否已修改
    pub fn is_modified(&self) -> bool {
        self.is_modified
    }

    /// 设置内容并标记为已修改
    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.is_modified = true;
    }

    /// 新建文件
    pub fn new_file(&mut self) {
        self.current_file = None;
        self.content = String::new();
        self.is_modified = false;
    }

    /// 打开文件
    pub fn open_file(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        // 读取文件内容
        let content = fs::read_to_string(path)
            .with_context(|| format!("无法读取文件: {}", path.display()))?;

        self.current_file = Some(path.to_path_buf());
        self.content = content;
        self.is_modified = false;

        Ok(())
    }

    /// 保存文件（如果已有文件路径）
    pub fn save_file(&mut self) -> Result<()> {
        if let Some(path) = &self.current_file {
            self.save_to_file(path)?;
            self.is_modified = false;
            Ok(())
        } else {
            Err(anyhow::anyhow!("没有文件路径，请使用 save_as"))
        }
    }

    /// 另存为
    pub fn save_as(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        self.save_to_file(path)?;
        self.current_file = Some(path.to_path_buf());
        self.is_modified = false;
        Ok(())
    }

    /// 保存到指定文件
    fn save_to_file(&self, path: &Path) -> Result<()> {
        // 确保父目录存在
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("无法创建目录: {}", parent.display()))?;
        }

        // 写入文件
        let mut file = fs::File::create(path)
            .with_context(|| format!("无法创建文件: {}", path.display()))?;

        file.write_all(self.content.as_bytes())
            .with_context(|| format!("无法写入文件: {}", path.display()))?;

        Ok(())
    }

    /// 检查是否需要保存（文件已修改且有路径）
    pub fn needs_save(&self) -> bool {
        self.is_modified && self.current_file.is_some()
    }
}

impl Default for FileManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_new_file() {
        let mut manager = FileManager::new();
        manager.new_file();

        assert!(manager.current_file().is_none());
        assert_eq!(manager.content(), "");
        assert!(!manager.is_modified());
        assert_eq!(manager.current_filename(), "未命名");
    }

    #[test]
    fn test_open_file() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "Hello, World!")?;

        let mut manager = FileManager::new();
        manager.open_file(temp_file.path())?;

        assert!(manager.current_file().is_some());
        assert_eq!(manager.content(), "Hello, World!\n");
        assert!(!manager.is_modified());

        Ok(())
    }

    #[test]
    fn test_save_file() -> Result<()> {
        let temp_file = NamedTempFile::new()?;

        let mut manager = FileManager::new();
        manager.open_file(temp_file.path())?;
        manager.set_content("New content".to_string());
        manager.save_file()?;

        let saved_content = fs::read_to_string(temp_file.path())?;
        assert_eq!(saved_content, "New content");
        assert!(!manager.is_modified());

        Ok(())
    }

    #[test]
    fn test_save_as() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let new_file_path = temp_dir.path().join("new_file.md");

        let mut manager = FileManager::new();
        manager.set_content("New file content".to_string());
        manager.save_as(&new_file_path)?;

        assert!(new_file_path.exists());
        let saved_content = fs::read_to_string(&new_file_path)?;
        assert_eq!(saved_content, "New file content");
        assert!(!manager.is_modified());
        assert_eq!(manager.current_filename(), "new_file.md");

        Ok(())
    }

    #[test]
    fn test_is_markdown_file() {
        let markdown_file = FileItem::new(
            "test.md".to_string(),
            PathBuf::from("/path/to/test.md"),
            crate::file_manager::FileType::File,
        );

        let not_markdown_file = FileItem::new(
            "test.txt".to_string(),
            PathBuf::from("/path/to/test.txt"),
            crate::file_manager::FileType::File,
        );

        let directory = FileItem::new(
            "folder".to_string(),
            PathBuf::from("/path/to/folder"),
            crate::file_manager::FileType::Directory,
        );

        assert!(markdown_file.is_markdown());
        assert!(!not_markdown_file.is_markdown());
        assert!(!directory.is_markdown());
    }
}
