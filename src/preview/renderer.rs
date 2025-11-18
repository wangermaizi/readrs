//! Markdown 预览渲染器
//! 
//! 使用自定义的 Markdown 渲染器渲染预览内容

use gpui::*;
use crate::markdown::MarkdownParser;

/// Markdown 预览器
/// 
/// 负责渲染解析后的 Markdown 内容
pub struct MarkdownPreview {
    /// 当前显示的 Markdown 内容
    markdown_content: SharedString,
}

impl MarkdownPreview {
    /// 创建新的预览器
    pub fn new() -> Self {
        Self {
            markdown_content: SharedString::default(),
        }
    }

    /// 更新预览内容
    /// 
    /// # 参数
    /// - `markdown`: 要显示的 Markdown 内容
    pub fn update_html(&mut self, markdown: impl Into<SharedString>) {
        self.markdown_content = markdown.into();
    }
}

impl Render for MarkdownPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // 使用自定义的 Markdown 渲染器
        render_markdown_preview(&self.markdown_content.to_string())
    }
}

/// 渲染 Markdown 预览
fn render_markdown_preview(markdown: &str) -> Div {
    if markdown.is_empty() {
        return div()
            .text_sm()
            .p_4()
            .text_color(rgb(0x999999))
            .text_center()
            .child("预览区域");
    }
    
    // 创建渲染元素 - 使用可变绑定
    let mut element = div().text_sm().p_4();
    let mut in_code_block = false;
    let mut code_block_content = String::new();
    
    // 将 markdown 转换为 owned 字符串以避免生命周期问题
    let markdown_owned = markdown.to_string();
    let mut lines = markdown_owned.lines();
    
    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        
        if trimmed.starts_with("```") {
            if in_code_block {
                // 结束代码块
                in_code_block = false;
                let code_content = code_block_content.trim().to_string();
                element = element.child(
                    div()
                        .bg(rgb(0xf5f5f5))
                        .p_2()
                        .rounded_sm()
                        .font_weight(FontWeight::BOLD)
                        .text_sm()
                        .mb_3()
                        .whitespace_nowrap()
                        .overflow_hidden()
                        .child(code_content)
                );
                code_block_content.clear();
            } else {
                // 开始代码块
                in_code_block = true;
            }
            continue;
        }
        
        if in_code_block {
            code_block_content.push_str(line);
            code_block_content.push('\n');
            continue;
        }
        
        // 处理其他 Markdown 语法 - 使用 owned 字符串
        if trimmed.starts_with("# ") {
            // H1
            let content = trimmed[2..].to_string();
            element = element.child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .mb_2()
                    .child(content)
            );
        } else if trimmed.starts_with("## ") {
            // H2
            let content = trimmed[3..].to_string();
            element = element.child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::BOLD)
                    .mb_2()
                    .mt_4()
                    .child(content)
            );
        } else if trimmed.starts_with("### ") {
            // H3
            let content = trimmed[4..].to_string();
            element = element.child(
                div()
                    .text_base()
                    .font_weight(FontWeight::BOLD)
                    .mb_2()
                    .mt_3()
                    .child(content)
            );
        } else if trimmed.starts_with("#### ") {
            // H4
            let content = trimmed[5..].to_string();
            element = element.child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::BOLD)
                    .mb_2()
                    .mt_2()
                    .child(content)
            );
        } else if trimmed.starts_with("##### ") {
            // H5
            let content = trimmed[6..].to_string();
            element = element.child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .mb_2()
                    .mt_2()
                    .child(content)
            );
        } else if trimmed.starts_with("###### ") {
            // H6
            let content = trimmed[7..].to_string();
            element = element.child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .mb_2()
                    .mt_2()
                    .text_color(rgb(0x666666))
                    .child(content)
            );
        } else if trimmed.starts_with("- ") {
            // 无序列表
            let content = format!("• {}", &trimmed[2..]);
            element = element.child(
                div()
                    .ml_4()
                    .mb_1()
                    .child(content)
            );
        } else if trimmed.starts_with("1. ") || trimmed.starts_with("2. ") || trimmed.starts_with("3. ") {
            // 有序列表
            if let Some(dot_pos) = trimmed.find('.') {
                let content = format!("{}. {}", &trimmed[..dot_pos], &trimmed[dot_pos + 2..]);
                element = element.child(
                    div()
                        .ml_4()
                        .mb_1()
                        .child(content)
                );
            }
        } else if trimmed.starts_with("> ") {
            // 引用
            let content = trimmed[2..].to_string();
            element = element.child(
                div()
                    .border_l_4()
                    .border_color(rgb(0xdddddd))
                    .pl_3()
                    .ml_2()
                    .mb_3()
                    .italic()
                    .child(content)
            );
        } else if !trimmed.is_empty() {
            // 普通段落
            element = element.child(
                div()
                    .mb_3()
                    .child(line.to_string())
            );
        } else {
            // 空行
            element = element.child(div().mb_2());
        }
    }
    
    element
}