// src/main.rs
use gpui::*;
use std::sync::Arc;
use tokio::sync::Mutex;

mod editor;
mod preview;
mod file_manager;
mod export;
mod theme;
mod features;
mod renderer;

use editor::MarkdownEditor;
use preview::MarkdownPreview;
use file_manager::FileListView;
use features::OutlineView;
use theme::ThemeManager;

// 应用状态
#[derive(Clone)]
struct AppState {
    text: SharedString,
    current_file_path: Option<SharedString>,
    theme_manager: Arc<Mutex<ThemeManager>>,
    view_mode: ViewMode,
    active_sidebar_tab: SidebarTab,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            text: "# 欢迎使用 ReadRS\n\n这是一个现代化的 Markdown 编辑器。\n\n## 功能特性\n\n- **实时预览** - 编辑时即时查看渲染效果\n- **多格式导出** - 支持导出为 HTML、PDF、DOCX 等格式\n- **主题切换** - 提供多种界面主题\n- **大纲视图** - 快速导航文档结构\n\n## 示例内容\n\n这是一些示例内容：\n\n- 项目1\n- 项目2\n- 项目3\n\n> 这是一个引用块\n\n`行内代码` 和代码块：\n\n```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```".into(),
            current_file_path: None,
            theme_manager: Arc::new(Mutex::new(ThemeManager::new())),
            view_mode: ViewMode::Dual,
            active_sidebar_tab: SidebarTab::Files,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    Preview,
    Edit,
    Dual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SidebarTab {
    Files,
    Outline,
}

// 主窗口视图
struct MainWindow {
    state: AppState,
    editor: MarkdownEditor,
    preview: MarkdownPreview,
    outline: OutlineView,
    file_list: FileListView,
}

impl MainWindow {
    fn new(_cx: &mut Window) -> Self {
        let mut editor = MarkdownEditor::new();
        let mut preview = MarkdownPreview::new();
        let mut outline = OutlineView::new();
        let file_list = FileListView::new();
        
        // 初始化编辑器文本
        let initial_text = AppState::default().text.clone();
        editor.set_text(initial_text.to_string());
        preview.update_text(initial_text.clone());
        outline.update_from_markdown(&initial_text);
        
        Self {
            state: AppState::default(),
            editor,
            preview,
            outline,
            file_list,
        }
    }
}

impl Render for MainWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme_manager.blocking_lock().current_theme();
        let view_mode = self.state.view_mode;
        let active_tab = self.state.active_sidebar_tab;
        
        div()
            .size_full()
            .bg(theme.background)
            .text_color(theme.text)
            .flex()
            .flex_col()
            .child(
                // 工具栏
                div()
                    .w_full()
                    .px(px(16.0))
                    .py(px(8.0))
                    .bg(theme.primary)
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .gap(px(8.0))
                            .child(
                                self.create_button("新建", move |this: &mut Self, _, _| {
                                    this.handle_new_file();
                                })
                            )
                            .child(
                                self.create_button("打开", move |this: &mut Self, _, _| {
                                    this.handle_open_file();
                                })
                            )
                            .child(
                                self.create_button("保存", move |this: &mut Self, _, _| {
                                    this.handle_save_file();
                                })
                            )
                            .child(
                                self.create_button("导出", move |this: &mut Self, _, _| {
                                    this.handle_export_file();
                                })
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(8.0))
                            .child(
                                self.create_view_mode_button("阅读模式", ViewMode::Preview, view_mode, move |this: &mut Self, _, _| {
                                    this.switch_view_mode(ViewMode::Preview);
                                })
                            )
                            .child(
                                self.create_view_mode_button("编辑模式", ViewMode::Edit, view_mode, move |this: &mut Self, _, _| {
                                    this.switch_view_mode(ViewMode::Edit);
                                })
                            )
                            .child(
                                self.create_view_mode_button("所见即所得", ViewMode::Dual, view_mode, move |this: &mut Self, _, _| {
                                    this.switch_view_mode(ViewMode::Dual);
                                })
                            )
                            .child(
                                self.create_button("主题", move |this: &mut Self, _, _| {
                                    this.handle_toggle_theme();
                                })
                            )
                            .child(
                                self.create_button("搜索", move |_: &mut Self, _, _| {
                                    // TODO: 实现搜索功能
                                })
                            )
                    )
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .child(
                        // 侧边栏
                        div()
                            .w(px(256.0))
                            .h_full()
                            .border_r(px(1.0))
                            .border_color(theme.border)
                            .bg(theme.sidebar_bg)
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .w_full()
                                    .border_b(px(1.0))
                                    .border_color(theme.border)
                                    .flex()
                                    .child(
                                        self.create_tab_button("文件", SidebarTab::Files, active_tab, theme.clone(), move |this: &mut Self, _, _| {
                                            this.switch_sidebar_tab(SidebarTab::Files);
                                        })
                                    )
                                    .child(
                                        self.create_tab_button("大纲", SidebarTab::Outline, active_tab, theme.clone(), move |this: &mut Self, _, _| {
                                            this.switch_sidebar_tab(SidebarTab::Outline);
                                        })
                                    )
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .overflow_y_scroll()
                                    .when(active_tab == SidebarTab::Files, |this| {
                                        this.child(self.file_list.render(window, &mut Context::new()))
                                    })
                                    .when(active_tab == SidebarTab::Outline, |this| {
                                        this.child(self.outline.render(window, &mut Context::new()))
                                    })
                            )
                    )
                    .child(
                        // 主编辑区
                        div()
                            .flex_1()
                            .h_full()
                            .flex()
                            .when(view_mode == ViewMode::Edit, |this| {
                                this.child(
                                    div()
                                        .w_full()
                                        .h_full()
                                        .child(self.editor.render(window, &mut Context::new()))
                                )
                            })
                            .when(view_mode == ViewMode::Preview, |this| {
                                this.child(
                                    div()
                                        .w_full()
                                        .h_full()
                                        .child(self.preview.render(window, &mut Context::new()))
                                )
                            })
                            .when(view_mode == ViewMode::Dual, |this| {
                                this.child(
                                    div()
                                        .w_1_2()
                                        .h_full()
                                        .border_r(px(1.0))
                                        .border_color(theme.border)
                                        .child(self.editor.render(window, &mut Context::new()))
                                )
                                .child(
                                    div()
                                        .w_1_2()
                                        .h_full()
                                        .child(self.preview.render(window, &mut Context::new()))
                                )
                            })
                    )
            )
    }
}

impl MainWindow {
    fn create_button<F>(&self, label: &str, on_click: F) -> impl IntoElement 
    where
        F: Fn(&mut Self, &mut Window, &mut Context<Self>) + 'static,
    {
        div()
            .px(px(12.0))
            .py(px(6.0))
            .bg(hsla(0.0, 0.0, 1.0, 0.2))
            .rounded(px(6.0))
            .cursor_pointer()
            .child(text(label))
            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
    }

    fn create_view_mode_button<F>(&self, label: &str, mode: ViewMode, current_mode: ViewMode, on_click: F) -> impl IntoElement 
    where
        F: Fn(&mut Self, &mut Window, &mut Context<Self>) + 'static,
    {
        let is_selected = mode == current_mode;
        div()
            .px(px(8.0))
            .py(px(4.0))
            .bg(if is_selected { hsla(0.0, 0.0, 1.0, 0.3) } else { hsla(0.0, 0.0, 1.0, 0.2) })
            .rounded(px(4.0))
            .cursor_pointer()
            .child(text(label))
            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
            .font_weight(if is_selected { FontWeight::BOLD } else { FontWeight::NORMAL })
    }

    fn create_tab_button<F>(&self, label: &str, tab: SidebarTab, active_tab: SidebarTab, theme: crate::theme::Theme, on_click: F) -> impl IntoElement 
    where
        F: Fn(&mut Self, &mut Window, &mut Context<Self>) + 'static,
    {
        let is_selected = tab == active_tab;
        div()
            .flex_1()
            .px(px(12.0))
            .py(px(12.0))
            .bg(if is_selected { theme.sidebar_bg } else { theme.tab_inactive_bg })
            .text_center()
            .cursor_pointer()
            .child(text(label))
            .text_color(theme.text)
            .font_weight(if is_selected { FontWeight::BOLD } else { FontWeight::NORMAL })
    }

    fn handle_new_file(&mut self) {
        self.state.text = String::new().into();
        self.state.current_file_path = None;
        self.editor.set_text(String::new());
        self.preview.update_text(String::new().into());
        self.outline.update_from_markdown("");
    }

    fn handle_open_file(&mut self) {
        // TODO: 实现文件打开对话框
        let content = "# 新打开的文档\n\n这是从文件加载的内容。".to_string();
        self.state.text = content.clone().into();
        self.state.current_file_path = Some("example.md".into());
        self.editor.set_text(content.clone());
        self.preview.update_text(content.clone().into());
        self.outline.update_from_markdown(&content);
    }

    fn handle_save_file(&mut self) {
        let text = self.state.text.to_string();
        let path = self.state.current_file_path.clone();
        
        // 异步保存文件
        if let Some(path) = path {
            let path_str = path.to_string();
            tokio::spawn(async move {
                if let Err(e) = file_manager::FileManager::save_file(&path_str, &text).await {
                    eprintln!("保存文件失败: {}", e);
                }
            });
        } else {
            eprintln!("请先选择保存位置");
        }
    }

    fn handle_export_file(&mut self) {
        let text = self.state.text.to_string();
        
        tokio::spawn(async move {
            if let Err(e) = export::Exporter::export_to_html(&text, "output.html").await {
                eprintln!("导出失败: {}", e);
            }
        });
    }

    fn handle_toggle_theme(&mut self) {
        self.state.theme_manager.blocking_lock().next_theme();
    }

    fn switch_view_mode(&mut self, mode: ViewMode) {
        self.state.view_mode = mode;
    }

    fn switch_sidebar_tab(&mut self, tab: SidebarTab) {
        self.state.active_sidebar_tab = tab;
    }
}

fn main() {
    App::new(|_app, _cx| {
        // App初始化
    })
    .run(|app: &mut App, cx: &mut AppContext| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("ReadRS - Markdown 编辑器".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(
                    Bounds::new(Point::new(px(100.0), px(100.0)), Size::new(px(1200.0), px(800.0)))
                )),
                ..Default::default()
            },
            |window, _cx| {
                MainWindow::new(window)
            },
        );
    });
}
