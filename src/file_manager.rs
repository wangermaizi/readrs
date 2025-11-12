// src/file_manager.rs
use gpui::*;
use std::path::Path;
use tokio::fs;
use anyhow::Result;
use std::collections::VecDeque;

// 最近文件管理器
#[derive(Debug, Clone)]
pub struct RecentFileManager {
    recent_files: VecDeque<String>,
    max_files: usize,
}

impl RecentFileManager {
    pub fn new(max_files: usize) -> Self {
        Self {
            recent_files: VecDeque::new(),
            max_files,
        }
    }

    pub fn add_file(&mut self, file_path: String) {
        // 如果文件已经存在，先移除它
        self.recent_files.retain(|f| f != &file_path);
        // 将文件添加到列表开头
        self.recent_files.push_front(file_path);
        
        // 限制最近文件数量
        while self.recent_files.len() > self.max_files {
            self.recent_files.pop_back();
        }
    }

    pub fn get_recent_files(&self) -> Vec<String> {
        self.recent_files.iter().cloned().collect()
    }

    pub fn remove_file(&mut self, file_path: &str) {
        self.recent_files.retain(|f| f != file_path);
    }
}

pub struct FileManager;

impl FileManager {
    pub async fn new_file() -> Result<String> {
        Ok(String::new())
    }

    pub async fn open_file(file_path: &str) -> Result<String> {
        let content = fs::read_to_string(file_path).await?;
        Ok(content)
    }

    pub async fn save_file(file_path: &str, content: &str) -> Result<()> {
        fs::write(file_path, content).await?;
        Ok(())
    }

    pub async fn save_file_as(file_path: &str, content: &str) -> Result<()> {
        Self::save_file(file_path, content).await
    }
}

// 实现一个简单的文件对话框模拟（实际应用中会需要原生对话框支持）
pub struct FileDialog;

impl FileDialog {
    pub fn show_open_dialog() -> Option<String> {
        // 这里应该显示原生文件打开对话框
        // 暂时返回一个模拟路径
        Some("example.md".to_string())
    }

    pub fn show_save_dialog() -> Option<String> {
        // 这里应该显示原生文件保存对话框
        // 暂时返回一个模拟路径
        Some("example.md".to_string())
    }
}

// 文件树视图组件
pub struct FileTreeView {
    root_path: String,
    files: Vec<String>,
}

impl FileTreeView {
    pub fn new(root_path: String) -> Self {
        Self {
            files: vec![],
            root_path,
        }
    }

    pub fn load_directory(&mut self) {
        // 加载目录中的文件
        // 实际实现中需要使用 tokio::fs::read_dir
    }
}

impl Render for FileTreeView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w_64()
            .h_full()
            .bg(gpui::gray_100())
            .p_3()
            .child(
                div()
                    .text_lg()
                    .font_bold()
                    .mb_3()
                    .child(Label::new("Files"))
            )
            .child(
                div()
                    .w_full()
                    .child(Label::new("No files loaded"))
            )
    }
}