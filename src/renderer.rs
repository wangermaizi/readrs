// src/renderer.rs
use gpui::*;
use pulldown_cmark::{Parser, Options, Event, Tag, CodeBlockKind};
use std::sync::Arc;

pub struct MarkdownRenderer {
    content: SharedString,
}

impl MarkdownRenderer {
    pub fn new(content: SharedString) -> Self {
        Self { content }
    }

    pub fn render_to_elements(&self) -> Vec<AnyElement> {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);

        let parser = Parser::new_ext(self.content.as_ref(), options);
        let mut elements = Vec::new();
        let mut current_text = String::new();
        let mut in_paragraph = false;
        let mut in_heading = false;
        let mut in_list = false;
        let mut in_blockquote = false;
        let mut in_code_block = false;
        let mut code_block_content = String::new();
        let mut list_level = 0;

        for event in parser {
            match event {
                Event::Start(tag) => {
                    // 保存当前文本
                    if !current_text.is_empty() {
                        elements.push(self.create_text_element(&current_text));
                        current_text.clear();
                    }

                    match tag {
                        Tag::Paragraph => {
                            in_paragraph = true;
                            // 段落开始，稍后添加
                        }
                        Tag::Heading(level, _, _) => {
                            in_heading = true;
                            // 标题处理
                            let heading_text = self.extract_next_text(&mut parser.clone());
                            if !heading_text.is_empty() {
                                elements.push(self.create_heading_element(&heading_text, level));
                            }
                        }
                        Tag::List(_) => {
                            in_list = true;
                            list_level += 1;
                        }
                        Tag::Item => {
                            // 列表项开始
                        }
                        Tag::BlockQuote => {
                            in_blockquote = true;
                        }
                        Tag::CodeBlock(kind) => {
                            in_code_block = true;
                            code_block_content.clear();
                            match kind {
                                CodeBlockKind::Fenced(lang) => {
                                    // 代码块语言类型
                                }
                                CodeBlockKind::Indented => {
                                    // 缩进代码块
                                }
                            }
                        }
                        Tag::Emphasis => {
                            // 斜体
                        }
                        Tag::Strong => {
                            // 粗体
                        }
                        Tag::Strikethrough => {
                            // 删除线
                        }
                        Tag::Link(_, _, _) => {
                            // 链接
                        }
                        Tag::Image(_, _, _) => {
                            // 图片
                        }
                        _ => {}
                    }
                }
                Event::End(tag) => {
                    match tag {
                        Tag::Paragraph => {
                            if !current_text.is_empty() {
                                elements.push(self.create_paragraph_element(&current_text));
                                current_text.clear();
                            }
                            in_paragraph = false;
                        }
                        Tag::Heading(_, _, _) => {
                            in_heading = false;
                        }
                        Tag::List(_) => {
                            in_list = false;
                            list_level -= 1;
                        }
                        Tag::CodeBlock(_) => {
                            if !code_block_content.is_empty() {
                                elements.push(self.create_code_block_element(&code_block_content));
                                code_block_content.clear();
                            }
                            in_code_block = false;
                        }
                        Tag::BlockQuote => {
                            if !current_text.is_empty() {
                                elements.push(self.create_blockquote_element(&current_text));
                                current_text.clear();
                            }
                            in_blockquote = false;
                        }
                        _ => {}
                    }
                }
                Event::Text(text) => {
                    if in_code_block {
                        code_block_content.push_str(&text);
                    } else {
                        current_text.push_str(&text);
                    }
                }
                Event::Code(code) => {
                    // 行内代码
                    elements.push(self.create_inline_code_element(&code));
                }
                Event::Html(html) => {
                    // HTML内容
                    current_text.push_str(&html);
                }
                Event::Rule => {
                    // 分割线
                    elements.push(self.create_rule_element());
                }
                Event::SoftBreak => {
                    current_text.push(' ');
                }
                Event::HardBreak => {
                    current_text.push('\n');
                }
                Event::FootnoteReference(name) => {
                    // 脚注引用
                    elements.push(self.create_footnote_element(&name));
                }
                Event::TaskListMarker(checked) => {
                    // 任务列表标记
                    let marker = if checked { "[x] " } else { "[ ] " };
                    elements.push(self.create_task_marker_element(marker));
                }
            }
        }

        // 处理剩余文本
        if !current_text.is_empty() {
            elements.push(self.create_text_element(&current_text));
        }

        elements
    }

    fn create_text_element(&self, text: &str) -> AnyElement {
        gpui::TextElement::new(text.to_string().into())
            .color(gpui::black())
            .size(gpui::TextSize::Default)
            .into_any_element()
    }

    fn create_paragraph_element(&self, text: &str) -> AnyElement {
        div()
            .mt_1()
            .mb_2()
            .child(self.create_text_element(text))
            .into_any_element()
    }

    fn create_heading_element(&self, text: &str, level: pulldown_cmark::HeadingLevel) -> AnyElement {
        let (size, margin) = match level {
            pulldown_cmark::HeadingLevel::H1 => (gpui::TextSize::XXXLarge, 4),
            pulldown_cmark::HeadingLevel::H2 => (gpui::TextSize::XXLarge, 3),
            pulldown_cmark::HeadingLevel::H3 => (gpui::TextSize::XLarge, 2),
            pulldown_cmark::HeadingLevel::H4 => (gpui::TextSize::Large, 2),
            pulldown_cmark::HeadingLevel::H5 => (gpui::TextSize::Medium, 1),
            pulldown_cmark::HeadingLevel::H6 => (gpui::TextSize::Small, 1),
        };

        gpui::TextElement::new(text.to_string().into())
            .size(size)
            .mt_2()
            .mb_1()
            .into_any_element()
    }

    fn create_inline_code_element(&self, code: &str) -> AnyElement {
        gpui::TextElement::new(code.to_string().into())
            .size(gpui::TextSize::Default)
            .bg(gpui::rgb(0xf0f0f0))
            .px_1()
            .rounded_sm()
            .into_any_element()
    }

    fn create_code_block_element(&self, code: &str) -> AnyElement {
        div()
            .w_full()
            .border_1()
            .border_color(gpui::gray_300())
            .bg(gpui::rgb(0xf8f8f8))
            .p_2()
            .rounded_sm()
            .child(gpui::TextElement::new(code.to_string().into())
                .size(gpui::TextSize::Default)
                .font_family("monospace")
            )
            .into_any_element()
    }

    fn create_blockquote_element(&self, text: &str) -> AnyElement {
        div()
            .border_l_4()
            .border_color(gpui::gray_400())
            .pl_3()
            .ml_2()
            .child(self.create_text_element(text))
            .into_any_element()
    }

    fn create_rule_element(&self) -> AnyElement {
        div()
            .w_full()
            .h_0_5()
            .bg(gpui::gray_300())
            .my_3()
            .into_any_element()
    }

    fn create_footnote_element(&self, name: &str) -> AnyElement {
        gpui::TextElement::new(format!("[{}]", name).into())
            .size(gpui::TextSize::Small)
            .text_color(gpui::blue_500())
            .into_any_element()
    }

    fn create_task_marker_element(&self, marker: &str) -> AnyElement {
        gpui::TextElement::new(marker.into())
            .size(gpui::TextSize::Default)
            .text_color(gpui::green_500())
            .into_any_element()
    }

    // 辅助函数：提取下一个文本内容
    fn extract_next_text(&self, parser: &mut std::slice::Iter<Event>) -> String {
        let mut text = String::new();
        for event in parser {
            match event {
                Event::Text(t) => text.push_str(t),
                Event::Code(c) => text.push_str(c),
                Event::SoftBreak => text.push(' '),
                Event::HardBreak => text.push('\n'),
                _ => break,
            }
        }
        text
    }
}