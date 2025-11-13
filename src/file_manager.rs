// src/file_manager.rs
use gpui::*;
use tokio::fs;
use anyhow::Result;

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

pub struct FileListView {
    recent_files: Vec<String>,
}

impl FileListView {
    pub fn new() -> Self {
        Self {
            recent_files: vec![
                "示例文档.md".to_string(),
                "项目说明.md".to_string(),
                "使用指南.md".to_string(),
            ],
        }
    }

    pub fn add_recent_file(&mut self, file_path: String) {
        self.recent_files.retain(|f| f != &file_path);
        self.recent_files.insert(0, file_path);
        
        if self.recent_files.len() > 20 {
            self.recent_files.truncate(20);
        }
    }

    pub fn render_content(&self) -> Vec<AnyElement> {
        self.recent_files.iter().map(|file| {
            let file_path = file.clone();
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .p(px(8.0))
                .rounded(px(6.0))
                .hover(|style| style.bg(hsla(0.0, 0.0, 0.95, 1.0)))
                .cursor_pointer()
                .child(
                    div()
                        .child(text("📄"))
                        .text_size(px(16.0))
                )
                .child(
                    div()
                        .child(text(file))
                        .text_color(hsla(210.0, 0.8, 0.5, 1.0))
                )
                .into_any_element()
        }).collect()
    }
}

impl Render for FileListView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p(px(12.0))
            .flex()
            .flex_col()
            .child(
                div()
                    .child(text("最近文件"))
                    .text_size(px(16.0))
                    .font_weight(FontWeight::BOLD)
                    .mb(px(12.0))
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .children(self.render_content())
            )
    }
}
