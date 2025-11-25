//! 文件夹树视图模块
//!
//! 提供文件夹树视图功能：
//! - 递归扫描文件夹
//! - 构建文件树结构
//! - 支持展开/折叠

use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use walkdir::WalkDir;

use super::{FileItem, FileType};

/// 文件夹树管理器
pub struct FileTree {
    /// 根路径
    root_path: PathBuf,
    /// 文件树结构
    root_item: FileItem,
    /// 是否显示隐藏文件
    show_hidden: bool,
}

impl FileTree {
    /// 创建新的文件夹树
    pub fn new(root_path: impl AsRef<Path>) -> Result<Self> {
        let root_path = root_path.as_ref();

        // 检查路径是否存在
        if !root_path.exists() {
            return Err(anyhow::anyhow!("路径不存在: {}", root_path.display()));
        }

        // 构建文件树
        let root_item = Self::build_tree(root_path, true)?;

        Ok(Self {
            root_path: root_path.to_path_buf(),
            root_item,
            show_hidden: false,
        })
    }

    /// 从路径创建文件树（递归）
    fn build_tree(path: &Path, is_root: bool) -> Result<FileItem> {
        let metadata = fs::metadata(path)
            .with_context(|| format!("无法读取路径元数据: {}", path.display()))?;

        let file_type = if metadata.is_dir() {
            FileType::Directory
        } else {
            FileType::File
        };

        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());

        let mut item = FileItem::new(name, path.to_path_buf(), file_type);

        // 如果是目录且不是根目录，递归构建子项
        if metadata.is_dir() && is_root {
            item.children = Self::scan_directory(path)?;
            item.expanded = true; // 根目录默认展开
        }

        Ok(item)
    }

    /// 扫描目录内容
    fn scan_directory(path: &Path) -> Result<Vec<FileItem>> {
        let mut children = Vec::new();

        // 读取目录内容
        let entries = fs::read_dir(path)
            .with_context(|| format!("无法读取目录: {}", path.display()))?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            // 跳过隐藏文件（以.开头）
            if let Some(name) = path.file_name() {
                if name.to_string_lossy().starts_with('.') {
                    continue;
                }
            }

            let metadata = entry.metadata()?;
            let file_type = if metadata.is_dir() {
                FileType::Directory
            } else {
                FileType::File
            };

            let name = path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.to_string_lossy().to_string());

            let mut item = FileItem::new(name, path.clone(), file_type);

            // 如果是目录，递归扫描
            if metadata.is_dir() {
                match Self::scan_directory(&path) {
                    Ok(sub_children) => {
                        item.children = sub_children;
                    }
                    Err(e) => {
                        eprintln!("警告: 无法扫描目录 {}: {}", path.display(), e);
                        item.children = Vec::new();
                    }
                }
            }

            children.push(item);
        }

        // 排序：目录在前，文件在后；按名称排序
        children.sort_by(|a, b| {
            match (a.file_type, b.file_type) {
                (FileType::Directory, FileType::File) => std::cmp::Ordering::Less,
                (FileType::File, FileType::Directory) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            }
        });

        Ok(children)
    }

    /// 刷新文件树
    pub fn refresh(&mut self) -> Result<()> {
        self.root_item = Self::build_tree(&self.root_path, true)?;
        Ok(())
    }

    /// 切换展开/折叠状态
    pub fn toggle_expand(&mut self, path: &Path) -> bool {
        if let Some(item) = Self::find_item_mut(&mut self.root_item, path) {
            item.expanded = !item.expanded;
            return item.expanded;
        }
        false
    }

    /// 查找文件项（可变引用）
    pub fn find_item_mut<'a>(item: &'a mut FileItem, path: &Path) -> Option<&'a mut FileItem> {
        if item.path == path {
            return Some(item);
        }

        for child in &mut item.children {
            if let Some(found) = Self::find_item_mut(child, path) {
                return Some(found);
            }
        }

        None
    }

    /// 查找文件项（不可变引用）
    pub fn find_item<'a>(item: &'a FileItem, path: &Path) -> Option<&'a FileItem> {
        if item.path == path {
            return Some(item);
        }

        for child in &item.children {
            if let Some(found) = Self::find_item(child, path) {
                return Some(found);
            }
        }

        None
    }

    /// 获取根项
    pub fn root_item(&self) -> &FileItem {
        &self.root_item
    }

    /// 获取根路径
    pub fn root_path(&self) -> &Path {
        &self.root_path
    }

    /// 设置是否显示隐藏文件
    pub fn set_show_hidden(&mut self, show_hidden: bool) -> Result<()> {
        self.show_hidden = show_hidden;
        self.refresh()
    }

    /// 获取所有 Markdown 文件
    pub fn get_markdown_files(&self) -> Vec<&FileItem> {
        let mut files = Vec::new();
        self.collect_markdown_files(&self.root_item, &mut files);
        files
    }

    /// 递归收集所有 Markdown 文件
    fn collect_markdown_files<'a>(&'a self, item: &'a FileItem, files: &mut Vec<&'a FileItem>) {
        if item.is_markdown() {
            files.push(item);
        }

        for child in &item.children {
            self.collect_markdown_files(child, files);
        }
    }

    /// 获取指定路径的子项
    pub fn get_children(&self, path: &Path) -> Option<&[FileItem]> {
        Self::find_item(&self.root_item, path)
            .map(|item| item.children.as_slice())
    }

    /// 判断路径是否已展开
    pub fn is_expanded(&self, path: &Path) -> bool {
        Self::find_item(&self.root_item, path)
            .map(|item| item.expanded)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File;

    #[test]
    fn test_create_file_tree() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let root_path = temp_dir.path();

        // 创建测试文件结构
        fs::create_dir(root_path.join("subdir"))?;
        File::create(root_path.join("file1.md"))?;
        File::create(root_path.join("subdir").join("file2.md"))?;

        let file_tree = FileTree::new(root_path)?;

        assert_eq!(file_tree.root_path(), root_path);
        assert_eq!(file_tree.root_item().file_type, FileType::Directory);

        Ok(())
    }

    #[test]
    fn test_scan_directory() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let root_path = temp_dir.path();

        // 创建测试文件结构
        fs::create_dir(root_path.join("subdir1"))?;
        fs::create_dir(root_path.join("subdir2"))?;
        File::create(root_path.join("file1.md"))?;
        File::create(root_path.join("file2.txt"))?;
        File::create(root_path.join("subdir1").join("file3.md"))?;

        let children = FileTree::scan_directory(root_path)?;

        // 应该包含 2 个目录和 2 个文件
        assert_eq!(children.len(), 4);

        // 验证排序：目录在前
        assert_eq!(children[0].file_type, FileType::Directory);
        assert_eq!(children[1].file_type, FileType::Directory);
        assert_eq!(children[2].file_type, FileType::File);
        assert_eq!(children[3].file_type, FileType::File);

        Ok(())
    }

    #[test]
    fn test_toggle_expand() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let root_path = temp_dir.path();

        fs::create_dir(root_path.join("subdir"))?;

        let mut file_tree = FileTree::new(root_path)?;
        let subdir_path = root_path.join("subdir");

        // 初始状态为展开
        assert!(file_tree.is_expanded(&subdir_path));

        // 切换为折叠
        file_tree.toggle_expand(&subdir_path);
        assert!(!file_tree.is_expanded(&subdir_path));

        // 切换回展开
        file_tree.toggle_expand(&subdir_path);
        assert!(file_tree.is_expanded(&subdir_path));

        Ok(())
    }

    #[test]
    fn test_get_markdown_files() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let root_path = temp_dir.path();

        // 创建测试文件结构
        fs::create_dir(root_path.join("subdir"))?;
        File::create(root_path.join("file1.md"))?;
        File::create(root_path.join("file2.txt"))?;
        File::create(root_path.join("subdir").join("file3.md"))?;
        File::create(root_path.join("subdir").join("file4.txt"))?;

        let file_tree = FileTree::new(root_path)?;
        let markdown_files = file_tree.get_markdown_files();

        // 应该只包含 .md 文件
        assert_eq!(markdown_files.len(), 2);
        assert!(markdown_files.iter().all(|f| f.is_markdown()));

        Ok(())
    }

    #[test]
    fn test_find_item() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let root_path = temp_dir.path();

        fs::create_dir(root_path.join("subdir"))?;
        File::create(root_path.join("file.md"))?;

        let file_tree = FileTree::new(root_path)?;

        let root_item = file_tree.root_item();
        let found = FileTree::find_item(root_item, &root_path.join("subdir"));
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "subdir");

        Ok(())
    }
}
