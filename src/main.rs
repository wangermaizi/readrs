//! ReadRS - 类 Typora 的所见即所得 Markdown 编辑器
//! 
//! 阶段 2：核心功能 - Markdown 实时预览基础版
//! 
//! 本文件实现了：
//! - 编辑区 + 预览区左右分栏布局
//! - Markdown 实时预览功能
//! - 基础文本编辑功能

use gpui::*;
use gpui_component::*;
use rfd::FileDialog;

mod editor;
mod markdown;
mod preview;
mod file_manager;

use editor::TextEditor;
use markdown::MarkdownParser;
use preview::MarkdownPreview;
use file_manager::{FileManager, FileTree, SearchManager, FileItem, FileType};
use gpui_component::button::Button;

/// 主窗口视图
/// 
/// 包含文件树、编辑区和预览区，实现三栏布局
pub struct MainWindow {
    /// 文本编辑器
    editor: Entity<TextEditor>,
    /// Markdown 预览器
    preview: Entity<MarkdownPreview>,
    /// 当前 Markdown 内容
    markdown_content: SharedString,
    /// 文件管理器
    file_manager: Entity<FileManager>,
    /// 文件树
    file_tree: Entity<FileTree>,
    /// 搜索管理器
    search_manager: Entity<SearchManager>,
    /// 搜索查询
    search_query: SharedString,
    /// 搜索结果
    search_results: Vec<String>,
}

impl MainWindow {
    /// 创建新的主窗口
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        // 创建编辑器
        let editor = cx.new(|cx| TextEditor::new(window, cx));

        // 创建预览器
        let preview = cx.new(|_cx| MarkdownPreview::new());

        // 创建文件管理器
        let file_manager = cx.new(|_cx| FileManager::new());

        // 创建文件树（使用当前目录作为根）
        let current_dir = std::env::current_dir().unwrap_or_default();
        let file_tree = cx.new(|_cx| {
            FileTree::new(&current_dir).unwrap_or_else(|_| FileTree::new(".").unwrap_or_else(|_| {
                // 如果都失败了，创建一个空的文件树
                use file_manager::{FileItem, FileType};
                
                // 创建失败的备用方案
                let mut item = FileItem::new(
                    "root".to_string(),
                    std::path::PathBuf::from("."),
                    FileType::Directory,
                );
                item.expanded = true;
                FileTree::new(".").unwrap_or_else(|_| {
                    // 如果还是失败，panic
                    panic!("无法创建文件树")
                })
            }))
        });

        // 创建搜索管理器
        let search_manager = cx.new(|_cx| SearchManager::new());

        let mut main_window = Self {
            editor: editor.clone(),
            preview: preview.clone(),
            markdown_content: SharedString::default(),
            file_manager: file_manager.clone(),
            file_tree: file_tree.clone(),
            search_manager: search_manager.clone(),
            search_query: SharedString::default(),
            search_results: Vec::new(),
        };

        // 订阅编辑器内容变化，实时更新预览
        main_window.setup_realtime_preview(window, cx);

        main_window
    }

    /// 设置实时预览功能
    /// 
    /// 当编辑器内容变化时，自动更新预览
    fn setup_realtime_preview(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let preview = self.preview.clone();
        let input_state = self.editor.read(cx).input_state();
        
        // 订阅输入状态的变化事件
        cx.subscribe_in(&input_state, window, move |_view, state, event, _window, cx| {
            use gpui_component::input::InputEvent as ComponentInputEvent;
            if let ComponentInputEvent::Change = event {
                let content = state.read(cx).value();
                // 直接传递 Markdown 内容到预览器进行渲染
                preview.update(cx, |preview, _cx| {
                    preview.update_html(content.to_string());
                });
                cx.notify();
            }
        })
        .detach();
    }

    /// 更新预览内容
    fn update_preview(&mut self, markdown: &str, cx: &mut Context<Self>) {
        let html = MarkdownParser::parse_with_styles(markdown);
        self.preview.update(cx, |preview, _cx| {
            preview.update_html(html);
        });
    }

    /// 新建文件
    fn new_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.file_manager.update(cx, |manager, _cx| {
            manager.new_file();
        });
        self.editor.update(cx, |editor, cx| {
            editor.set_content("", window, cx);
        });
        self.search_results.clear();
        cx.notify();
    }

    /// 打开文件
    fn open_file(&mut self, path: std::path::PathBuf, window: &mut Window, cx: &mut Context<Self>) {
        // 先尝试打开文件并获取内容
        let open_result = self.file_manager.update(cx, |manager, _cx| {
            manager.open_file(&path).map(|_| manager.content().to_string())
        });
        
        // 如果成功，更新编辑器内容
        if let Ok(content) = open_result {
            self.editor.update(cx, |editor, cx| {
                editor.set_content(content, window, cx);
            });
        }
        cx.notify();
    }

    /// 保存文件
    fn save_file(&mut self, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            let content = editor.content(cx).to_string();
            self.file_manager.update(cx, |manager, _cx| {
                manager.set_content(content);
                if let Err(e) = manager.save_file() {
                    eprintln!("保存文件失败: {}", e);
                }
            });
        });
        cx.notify();
    }

    /// 另存为
    fn save_as(&mut self, path: std::path::PathBuf, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            let content = editor.content(cx).to_string();
            self.file_manager.update(cx, |manager, _cx| {
                manager.set_content(content);
                if let Err(e) = manager.save_as(&path) {
                    eprintln!("另存为失败: {}", e);
                }
            });
        });
        cx.notify();
    }

    /// 执行搜索
    fn perform_search(&mut self, query: String, cx: &mut Context<Self>) {
        if query.is_empty() {
            self.search_results.clear();
            cx.notify();
            return;
        }

        self.editor.update(cx, |editor, cx| {
            let content = editor.content(cx).to_string();
            self.search_manager.update(cx, |manager, _cx| {
                let results = manager.search(&query, &content);
                self.search_results = results.iter()
                    .map(|r| format!("Line {}: {}", r.line_number, r.preview))
                    .collect();
            });
        });
        cx.notify();
    }

    /// 渲染文件项
    fn render_file_item(&self, item: &FileItem, depth: usize, cx: &mut Context<MainWindow>) -> impl IntoElement {
        let is_directory = item.file_type == FileType::Directory;
        let is_markdown = item.is_markdown();
        let path = item.path.clone();
        
        // 缩进
        let indent = px((depth * 16) as f32);
        
        // 图标
        let icon = if is_directory {
            if item.expanded {
                "📂"  // 打开的文件夹
            } else {
                "📁"  // 关闭的文件夹
            }
        } else if is_markdown {
            "📝"  // Markdown 文件
        } else {
            "📄"  // 普通文件
        };
        
        // 文本颜色
        let text_color = if is_markdown {
            rgb(0x00ccff)  // Markdown 文件用蓝色
        } else if is_directory {
            rgb(0xffcc00)  // 文件夹用黄色
        } else {
            rgb(0xcccccc)  // 普通文件用灰色
        };
        
        // 如果是文件，使用 Button 组件支持点击
        if !is_directory {
            Button::new("file")
                .on_click(cx.listener(move |this, _event, window, cx| {
                    this.open_file(path.clone(), window, cx);
                }))
                .child(format!("{} {}", icon, item.name))
                .into_any_element()
        } else {
            // 文件夹使用 Button 组件支持点击展开/折叠
            Button::new("folder")
                .on_click(cx.listener(move |this, _event, _window, cx| {
                    this.file_tree.update(cx, |file_tree, _cx| {
                        file_tree.toggle_expand(&path);
                    });
                    cx.notify();
                }))
                .child(format!("{} {}", icon, item.name))
                .into_any_element()
        }
    }
    
    /// 递归渲染文件项
    fn render_file_item_recursive(
        &self, 
        mut element: Div, 
        item: FileItem, 
        depth: usize, 
        cx: &mut Context<MainWindow>
    ) -> Div {
        // 渲染当前项
        element = element.child(self.render_file_item(&item, depth, cx));
        
        // 如果是目录且已展开，递归渲染子项
        if item.file_type == FileType::Directory && item.expanded && !item.children.is_empty() {
            let children = item.children.clone();
            for child in children {
                element = self.render_file_item_recursive(element, child, depth + 1, cx);
            }
        }
        
        element
    }
    
    /// 渲染文件树
    fn render_file_tree(&self, cx: &mut Context<MainWindow>) -> impl IntoElement {
        let file_tree = self.file_tree.read(cx);
        let root_item = file_tree.root_item();
        
        let children = root_item.children.clone();
        let mut element = div();
        
        for child in children {
            element = self.render_file_item_recursive(element, child, 0, cx);
        }
        
        element
    }
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 获取当前文件名
        let filename = self.file_manager.read(cx).current_filename();

        // 创建三栏布局：左侧文件树 + 中间编辑器 + 右侧预览
        div()
            .h_full()
            .w_full()
            .flex()
            .flex_col()
            .bg(rgb(0xf5f5f5))
            .child(
                // 顶部工具栏
                div()
                    .w_full()
                    .h(px(40.0))
                    .flex()
                    .items_center()
                    .px_2()
                    .bg(rgb(0x2d2d2d))
                    .child(
                        // 文件操作按钮
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                Button::new("new")
                                    .child("新建")
                                    .on_click(cx.listener(|this, _event, window, cx| {
                                        this.new_file(window, cx);
                                    }))
                            )
                            .child(
                                Button::new("open")
                                    .child("打开")
                                    .on_click(cx.listener(|this, _event, window, cx| {
                                        // 打开文件对话框
                                        if let Some(path) = FileDialog::new()
                                            .add_filter("Markdown", &["md", "markdown"])
                                            .add_filter("Text", &["txt"])
                                            .add_filter("All Files", &["*"])
                                            .pick_file()
                                        {
                                            this.open_file(path, window, cx);
                                        }
                                    }))
                            )
                            .child(
                                Button::new("save")
                                    .child("保存")
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.save_file(cx);
                                    }))
                            )
                            .child(
                                Button::new("save_as")
                                    .child("另存为")
                                    .on_click(cx.listener(|this, _event, window, cx| {
                                        // 打开保存对话框
                                        if let Some(path) = FileDialog::new()
                                            .add_filter("Markdown", &["md", "markdown"])
                                            .add_filter("Text", &["txt"])
                                            .add_filter("All Files", &["*"])
                                            .save_file()
                                        {
                                            this.save_as(path, cx);
                                        }
                                    }))
                            )
                    )
                    .child(
                        // 文件名显示
                        div()
                            .flex_1()
                            .px_4()
                            .text_color(rgb(0xffffff))
                            .text_sm()
                            .child(filename)
                    )
                    .child(
                        // 搜索框和结果
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        div()
                                            .w(px(150.0))
                                            .px_2()
                                            .py_1()
                                            .bg(rgb(0x4a4a4a))
                                            .text_color(rgb(0xffffff))
                                            .text_sm()
                                            .rounded(px(2.0))
                                            .child("搜索...")
                                    )
                                    .child(
                                        Button::new("search")
                                            .child("搜索")
                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                this.perform_search("test".to_string(), cx);
                                            }))
                                    )
                            )
                    )
            )
            .child(
                // 主内容区域
                div()
                    .flex_1()
                    .flex()
                    .bg(rgb(0xf5f5f5))
                    .child(
                        // 左侧文件树和搜索结果（宽度 250px）
                        div()
                            .w(px(250.0))
                            .h_full()
                            .bg(rgb(0x2d2d2d))
                            .border_r(px(1.0))
                            .border_color(rgb(0x1a1a1a))
                            .flex()
                            .flex_col()
                            .child(
                                // 文件树区域（占 60%）
                                div()
                                    .flex()
                                    .flex_col()
                                    .h_3_5()  // 60% 高度
                                    .child(
                                        div()
                                            .p_2()
                                            .text_sm()
                                            .text_color(rgb(0xcccccc))
                                            .child("文件树")
                                    )
                                    .child(
                                        div()
                                            .flex_1()

                                            .child(self.render_file_tree(cx))
                                    )
                            )
                            .child(
                                // 搜索结果区域（占 40%）
                                div()
                                    .flex()
                                    .flex_col()
                                    .h_2_5()  // 40% 高度
                                    .border_t(px(1.0))
                                    .border_color(rgb(0x1a1a1a))
                                    .child(
                                        div()
                                            .p_2()
                                            .text_sm()
                                            .text_color(rgb(0xcccccc))
                                            .child("搜索结果")
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .overflow_scroll()
                                            .p_2()
                                            .text_xs()
                                            .text_color(rgb(0x999999))
                                            .map(|mut element| {
                                                // 显示搜索结果
                                                if self.search_results.is_empty() {
                                                    element = element.child("暂无搜索结果");
                                                } else {
                                                    for (i, result) in self.search_results.iter().enumerate() {
                                                        if i > 0 {
                                                            element = element.child(div().h(px(4.0)));
                                                        }
                                                        element = element.child(
                                                            div()
                                                                .child(result.clone())
                                                        );
                                                    }
                                                }
                                                element
                                            })
                                    )
                            )
                    )
                    .child(
                        // 中间编辑区和右侧预览区
                        div()
                            .flex_1()
                            .flex()
                            .child(
                                // 左侧编辑区
                                div()
                                    .w_1_2()
                                    .h_full()
                                    .border_r(px(1.0))
                                    .border_color(rgb(0xdddddd))
                                    .bg(rgb(0xffffff))
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .p_2()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .border_b(px(1.0))
                                            .border_color(rgb(0xeeeeee))
                                            .child("编辑器")
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .overflow_hidden()
                                            .child(self.editor.clone())
                                    )
                            )
                            .child(
                                // 右侧预览区
                                div()
                                    .w_1_2()
                                    .h_full()
                                    .bg(rgb(0xffffff))
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .p_2()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .border_b(px(1.0))
                                            .border_color(rgb(0xeeeeee))
                                            .child("预览")
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .overflow_hidden()
                                            .child(self.preview.clone())
                                    )
                            )
                    )
            )
    }
}

/// 应用程序入口点
fn main() {
    // 创建 GPUI 应用实例
    let app = Application::new();

    // 运行应用
    app.run(move |cx| {
        // 重要：必须在任何 gpui-component 功能使用之前调用初始化
        gpui_component::init(cx);

        // 异步创建窗口
        cx.spawn(async move |cx| {
            // 打开窗口，配置窗口选项
            cx.open_window(
                WindowOptions {
                    // 窗口标题
                    titlebar: Some(TitlebarOptions {
                        title: Some("ReadRS - Markdown 编辑器".into()),
                        ..Default::default()
                    }),
                    // 窗口初始大小和位置
                    window_bounds: Some(WindowBounds::Windowed(
                        Bounds::new(
                            Point::new(px(100.0), px(100.0)),  // 初始位置
                            gpui::Size::new(px(1400.0), px(900.0))  // 初始大小：1400x900（更大的窗口以容纳分栏）
                        )
                    )),
                    // 窗口默认聚焦
                    focus: true,
                    ..Default::default()
                },
                |window, cx| {
                    // 创建主窗口视图
                    let view = cx.new(|cx| MainWindow::new(window, cx));
                    
                    // 重要：窗口的第一层必须是 Root 组件
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}