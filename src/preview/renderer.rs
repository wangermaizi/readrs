//! Markdown 预览渲染器
//! 
//! 使用自定义的 Markdown 渲染器渲染预览内容

use gpui::*;
use crate::markdown::{LatexRenderer, MermaidRenderer};
use crate::editor::SyntaxHighlighter;

/// Markdown 预览器
/// 
/// 负责渲染解析后的 Markdown 内容
pub struct MarkdownPreview {
    /// 当前显示的 Markdown 内容
    markdown_content: SharedString,
    /// 语法高亮器
    syntax_highlighter: SyntaxHighlighter,
}

impl MarkdownPreview {
    /// 创建新的预览器
    pub fn new() -> Self {
        Self {
            markdown_content: SharedString::default(),
            syntax_highlighter: SyntaxHighlighter::new(),
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
        let content = self.markdown_content.to_string();
        
        // 检查是否包含 LaTeX 公式
        if LatexRenderer::contains_latex(&content) {
            let rendered = LatexRenderer::render(&content);
            return div().text_sm().p_4().child(rendered);
        }
        
        // 检查是否包含 Mermaid 图表
        if MermaidRenderer::contains_mermaid(&content) {
            let diagrams = MermaidRenderer::extract_mermaid(&content);
            let mut element = div().text_sm().p_4();
            
            for (diagram, diagram_type) in diagrams {
                let svg = MermaidRenderer::render(&diagram, diagram_type);
                element = element.child(
                    div()
                        .mb_4()
                        .child(svg)
                );
            }
            
            return element;
        }
        
        // 使用自定义的 Markdown 渲染器
        render_markdown_preview(&content, &self.syntax_highlighter)
    }
}

/// 渲染 Markdown 预览
fn render_markdown_preview(markdown: &str, highlighter: &SyntaxHighlighter) -> Div {
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
    let mut in_table = false;
    let mut table_rows = Vec::new();
    let mut code_block_language = String::new();
    
    // 将 markdown 转换为 owned 字符串以避免生命周期问题
    let markdown_owned = markdown.to_string();
    let mut lines = markdown_owned.lines();
    
    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        
        // 处理代码块
        if trimmed.starts_with("```") {
            if in_code_block {
                // 结束代码块
                in_code_block = false;
                let code_content = code_block_content.trim().to_string();
                
                // 使用语法高亮
                let highlighted = if !code_block_language.is_empty() {
                    highlighter.highlight(&code_content, &code_block_language)
                } else {
                    format!("<pre style=\"background-color: #f5f5f5; padding: 1em; border-radius: 4px;\">{}</pre>", 
                           html_escape(&code_content))
                };
                
                element = element.child(
                    div()
                        .mb_3()
                        .child(highlighted)
                );
                code_block_content.clear();
                code_block_language.clear();
            } else {
                // 开始代码块，提取语言
                in_code_block = true;
                code_block_language = if trimmed.len() > 3 {
                    trimmed[3..].trim().to_string()
                } else {
                    String::new()
                };
            }
            continue;
        }
        
        if in_code_block {
            code_block_content.push_str(line);
            code_block_content.push('\n');
            continue;
        }
        
        // 处理表格
        if trimmed.starts_with('|') && trimmed.ends_with('|') {
            if !in_table {
                in_table = true;
                table_rows.clear();
            }
            table_rows.push(trimmed.to_string());
            continue;
        } else if in_table {
            // 表格结束，渲染表格
            in_table = false;
            element = element.child(render_table(&table_rows));
            table_rows.clear();
        }
        
        // 处理任务列表
        if trimmed.starts_with("- [ ] ") {
            let content = trimmed[6..].to_string();
            element = element.child(
                div()
                    .ml_4()
                    .mb_1()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .child(
                                div()
                                    .w_4()
                                    .h_4()
                                    .border_1()
                                    .border_color(rgb(0x999999))
                                    .mr_2()
                            )
                            .child(content)
                    )
            );
            continue;
        } else if trimmed.starts_with("- [x] ") {
            let content = trimmed[6..].to_string();
            element = element.child(
                div()
                    .ml_4()
                    .mb_1()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .child(
                                div()
                                    .w_4()
                                    .h_4()
                                    .bg(rgb(0x0066cc))
                                    .mr_2()
                            )
                            .child(content)
                    )
            );
            continue;
        }
        
        // 处理图片
        if trimmed.starts_with("![") && trimmed.contains("](") {
            if let Some(start) = trimmed.find('[') {
                if let Some(middle) = trimmed.find("](") {
                    if let Some(end) = trimmed.find(')') {
                        let alt_text = &trimmed[start + 1..middle];
                        let url = &trimmed[middle + 2..end];
                        element = element.child(
                            div()
                                .mb_3()
                                .child(
                                    div()
                                        .text_color(rgb(0x0066cc))
                                        .child(format!("🖼️ 图片: {} ({})", alt_text, url))
                                )
                        );
                        continue;
                    }
                }
            }
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
    
    // 处理剩余的表格
    if in_table && !table_rows.is_empty() {
        element = element.child(render_table(&table_rows));
    }
    
    element
}

/// HTML 转义函数
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// 渲染表格
fn render_table(rows: &[String]) -> Div {
    let mut table_element = div().mb_3().border_1().border_color(rgb(0xdddddd));
    
    for (i, row) in rows.iter().enumerate() {
        let mut row_element = div().flex();
        let cells: Vec<&str> = row.split('|').filter(|s| !s.is_empty()).collect();
        
        for cell in cells {
            let cell_content = cell.trim().to_string();
            row_element = row_element.child(
                div()
                    .flex_1()
                    .p_2()
                    .border_r(px(1.0))
                    .border_color(rgb(0xdddddd))
                    .bg(if i == 0 { rgb(0xf5f5f5) } else { rgb(0xffffff) })
                    .font_weight(if i == 0 { FontWeight::BOLD } else { FontWeight::NORMAL })
                    .child(cell_content)
            );
        }
        
        table_element = table_element.child(row_element);
    }
    
    table_element
}