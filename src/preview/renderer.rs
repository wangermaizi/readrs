//! Markdown 预览渲染器
//! 
//! 使用 WebView 组件渲染 HTML 预览

use gpui::*;
use gpui_component::*;

/// Markdown 预览器
/// 
/// 负责渲染解析后的 Markdown HTML 内容
pub struct MarkdownPreview {
    /// 当前显示的 HTML 内容
    html_content: SharedString,
}

impl MarkdownPreview {
    /// 创建新的预览器
    pub fn new() -> Self {
        Self {
            html_content: Self::default_html().into(),
        }
    }

    /// 更新预览内容
    /// 
    /// # 参数
    /// - `html`: 要显示的 HTML 内容
    pub fn update_html(&mut self, html: impl Into<SharedString>) {
        self.html_content = html.into();
    }

    /// 获取默认的 HTML 内容（空文档）
    fn default_html() -> String {
        r#"
        <div style="
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Helvetica Neue', Arial, sans-serif;
            line-height: 1.6;
            color: #666;
            padding: 20px;
            text-align: center;
        ">
            <p>预览区域</p>
            <p style="font-size: 0.9em; color: #999;">在这里输入 Markdown 内容，预览将实时更新</p>
        </div>
        "#
        .to_string()
    }
}

impl Render for MarkdownPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // 使用 div 渲染 HTML 内容
        // 注意：GPUI 0.2 版本可能不支持直接渲染 HTML
        // 这里我们先使用文本显示，后续可以使用 WebView 组件
        div()
            .h_full()
            .w_full()
            .overflow_y_scroll()
            .bg(rgb(0xffffff))
            .p_4()
            .child(
                // 暂时使用文本显示，后续替换为 WebView
                div()
                    .text_sm()
                    .text_color(rgb(0x333333))
                    .whitespace_pre_wrap()
                    .child(
                        // 显示 HTML 内容（简化版，后续会使用 WebView）
                        // 这里我们只显示文本内容，不解析 HTML
                        // 实际应用中应该使用 WebView 组件来渲染 HTML
                        self.html_content.as_ref()
                    )
            )
    }
}

