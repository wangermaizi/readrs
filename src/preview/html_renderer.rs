//! HTML 到 GPUI 元素的渲染器
//!
//! 将简单的 HTML 转换为 GPUI 的文本元素进行渲染

use gpui::*;

/// 将简单的 HTML 转换为 GPUI 文本元素
pub fn render_html_to_gpui(html: &str) -> Div {
    // 这是一个简化的 HTML 渲染器，只处理基本的 Markdown 生成的 HTML
    let element = div().text_sm();
    
    // 简单的 HTML 标签处理
    if html.contains("<h1>") {
        element.child(
            div()
                .text_xl()
                .font_weight(FontWeight::BOLD)
                .mb_2()
                .child(extract_text_between_tags(html, "h1"))
        )
    } else if html.contains("<h2>") {
        element.child(
            div()
                .text_lg()
                .font_weight(FontWeight::BOLD)
                .mb_2()
                .mt_4()
                .child(extract_text_between_tags(html, "h2"))
        )
    } else if html.contains("<h3>") {
        element.child(
            div()
                .text_base()
                .font_weight(FontWeight::BOLD)
                .mb_2()
                .mt_3()
                .child(extract_text_between_tags(html, "h3"))
        )
    } else if html.contains("<p>") {
        element.child(
            div()
                .mb_3()
                .child(extract_text_between_tags(html, "p"))
        )
    } else if html.contains("<li>") {
        element.child(
            div()
                .ml_4()
                .mb_1()
                .child(format!("• {}", extract_text_between_tags(html, "li")))
        )
    } else if html.contains("<code>") {
        element.child(
            div()
                .bg(rgb(0xf5f5f5))
                .px_1()
                .py_px()
                .rounded_sm()
                .font_weight(FontWeight::BOLD)
                .text_sm()
                .child(extract_text_between_tags(html, "code"))
        )
    } else if html.contains("<pre>") {
        element.child(
            div()
                .bg(rgb(0xf5f5f5))
                .p_2()
                .rounded_sm()
                .flex_auto()
                .font_weight(FontWeight::BOLD)
                .text_sm()
                .mb_3()
                .child(extract_text_between_tags(html, "pre"))
        )
    } else if html.contains("<blockquote>") {
        element.child(
            div()
                .border_l_4()
                .border_color(rgb(0xdddddd))
                .pl_3()
                .ml_2()
                .mb_3()
                .italic()
                .child(extract_text_between_tags(html, "blockquote"))
        )
    } else if html.contains("<strong>") {
        element.child(
            div()
                .font_weight(FontWeight::BOLD)
                .child(extract_text_between_tags(html, "strong"))
        )
    } else if html.contains("<em>") {
        element.child(
            div()
                .italic()
                .child(extract_text_between_tags(html, "em"))
        )
    } else if html.contains("<a") {
        element.child(
            div()
                .text_color(rgb(0x0066cc))
                .child(extract_link_text(html))
        )
    } else if html.contains("<hr>") {
        element.child(
            div()
                .border_t(px(1.0))
                .border_color(rgb(0xdddddd))
                .my_4()
        )
    } else {
        // 如果没有特定标签，直接显示文本
        element.child(div().child(html.to_string()))
    }
}

/// 提取标签之间的文本
fn extract_text_between_tags(html: &str, tag: &str) -> String {
    let start_tag = format!("<{}>", tag);
    let end_tag = format!("</{}>", tag);
    
    if let Some(start) = html.find(&start_tag) {
        if let Some(end) = html.find(&end_tag) {
            let start_pos = start + start_tag.len();
            if start_pos < end {
                return html[start_pos..end].trim().to_string();
            }
        }
    }
    
    html.to_string()
}

/// 提取链接文本
fn extract_link_text(html: &str) -> String {
    // 简单的链接提取，格式: <a href="url">text</a>
    if let Some(start) = html.find('>') {
        if let Some(end) = html.rfind("</a>") {
            if start + 1 < end {
                return html[start + 1..end].to_string();
            }
        }
    }
    html.to_string()
}

/// 渲染 Markdown 预览
pub fn render_markdown_preview(markdown: &str) -> Div {
    // 简单的行处理 - 移除所有whitespace相关设置以避免错误
    div().text_sm().child(markdown.to_string())
}