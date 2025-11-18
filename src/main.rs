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

mod editor;
mod markdown;
mod preview;

use editor::TextEditor;
use markdown::MarkdownParser;
use preview::MarkdownPreview;

/// 主窗口视图
/// 
/// 包含编辑区和预览区，实现左右分栏布局
pub struct MainWindow {
    /// 文本编辑器
    editor: Entity<TextEditor>,
    /// Markdown 预览器
    preview: Entity<MarkdownPreview>,
    /// 当前 Markdown 内容
    markdown_content: SharedString,
}

impl MainWindow {
    /// 创建新的主窗口
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        // 创建编辑器
        let editor = cx.new(|cx| TextEditor::new(window, cx));

        // 创建预览器
        let preview = cx.new(|_cx| MarkdownPreview::new());

        let mut main_window = Self {
            editor: editor.clone(),
            preview: preview.clone(),
            markdown_content: SharedString::default(),
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
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // 创建左右分栏布局
        div()
            .h_full()
            .w_full()
            .flex()
            .bg(rgb(0xf5f5f5))
            .child(
                // 左侧编辑区
                div()
                    .w_1_2()  // 占据 50% 宽度
                    .h_full()
                    .border_r(px(1.0))
                    .border_color(rgb(0xdddddd))
                    .bg(rgb(0xffffff))
                    .p_2()
                    .child(
                        // 编辑器标题
                        div()
                            .text_sm()
                            .text_color(rgb(0x666666))
                            .mb_2()
                            .child("编辑器")
                    )
                    .child(
                        // 文本编辑器组件
                        div()
                            .flex_1()
                            .overflow_hidden()
                            .child(self.editor.clone())
                    )
            )
            .child(
                // 右侧预览区
                div()
                    .w_1_2()  // 占据 50% 宽度
                    .h_full()
                    .bg(rgb(0xffffff))
                    .p_2()
                    .child(
                        // 预览区标题
                        div()
                            .text_sm()
                            .text_color(rgb(0x666666))
                            .mb_2()
                            .child("预览")
                    )
                    .child(
                        // Markdown 预览组件
                        div()
                            .flex_1()
                            .overflow_hidden()
                            .child(self.preview.clone())
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