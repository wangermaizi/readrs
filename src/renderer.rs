// src/renderer.rs
use gpui::*;
use pulldown_cmark::{Parser, Options, Event, Tag, TagEnd, CodeBlockKind, HeadingLevel};

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
        let mut in_code_block = false;
        let mut code_block_content = String::new();
        let mut code_block_lang: Option<String> = None;
        let mut current_heading_level: Option<HeadingLevel> = None;

        for event in parser {
            match event {
                Event::Start(tag) => {
                    if !current_text.is_empty() && !in_code_block {
                        if current_heading_level.is_none() {
                            elements.push(self.create_text_element(&current_text));
                        }
                        current_text.clear();
                    }

                    match tag {
                        Tag::Paragraph => {
                            // 段落开始，稍后处理
                        }
                        Tag::Heading { level, .. } => {
                            current_heading_level = Some(level);
                        }
                        Tag::List(_) => {
                            // 列表开始
                        }
                        Tag::Item => {
                            // 列表项开始
                        }
                        Tag::BlockQuote(_) => {
                            // 引用块开始
                        }
                        Tag::CodeBlock(kind) => {
                            in_code_block = true;
                            code_block_content.clear();
                            match kind {
                                CodeBlockKind::Fenced(lang) => {
                                    code_block_lang = Some(lang.to_string());
                                }
                                CodeBlockKind::Indented => {
                                    code_block_lang = None;
                                }
                            }
                        }
                        Tag::Emphasis => {
                            // 斜体开始
                        }
                        Tag::Strong => {
                            // 粗体开始
                        }
                        Tag::Strikethrough => {
                            // 删除线开始
                        }
                        Tag::Link { .. } => {
                            // 链接开始
                        }
                        Tag::Image { .. } => {
                            // 图片开始
                        }
                        _ => {}
                    }
                }
                Event::End(tag_end) => {
                    match tag_end {
                        TagEnd::Paragraph => {
                            if !current_text.is_empty() {
                                elements.push(self.create_paragraph_element(&current_text));
                                current_text.clear();
                            }
                        }
                        TagEnd::Heading(level) => {
                            if !current_text.is_empty() {
                                elements.push(self.create_heading_element(&current_text, level));
                                current_text.clear();
                            }
                            current_heading_level = None;
                        }
                        TagEnd::List(_) => {
                            // 列表结束
                        }
                        TagEnd::CodeBlock => {
                            if !code_block_content.is_empty() {
                                elements.push(self.create_code_block_element(&code_block_content, code_block_lang.as_deref()));
                                code_block_content.clear();
                                code_block_lang = None;
                            }
                            in_code_block = false;
                        }
                        TagEnd::BlockQuote => {
                            if !current_text.is_empty() {
                                elements.push(self.create_blockquote_element(&current_text));
                                current_text.clear();
                            }
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
                    elements.push(self.create_inline_code_element(&code));
                }
                Event::Html(html) => {
                    current_text.push_str(&html);
                }
                Event::Rule => {
                    elements.push(self.create_rule_element());
                }
                Event::SoftBreak => {
                    current_text.push(' ');
                }
                Event::HardBreak => {
                    current_text.push('\n');
                }
                Event::TaskListMarker(checked) => {
                    let marker = if checked { "[x] " } else { "[ ] " };
                    current_text.push_str(marker);
                }
                _ => {}
            }
        }

        if !current_text.is_empty() {
            elements.push(self.create_text_element(&current_text));
        }

        elements
    }

    fn create_text_element(&self, text: &str) -> AnyElement {
        div()
            .child(text(text))
            .text_color(hsla(0.0, 0.0, 0.0, 1.0))
            .into_any_element()
    }

    fn create_paragraph_element(&self, text: &str) -> AnyElement {
        div()
            .mt(px(8.0))
            .mb(px(16.0))
            .child(text(text))
            .line_height(relative(1.6))
            .into_any_element()
    }

    fn create_heading_element(&self, text: &str, level: HeadingLevel) -> AnyElement {
        let (size, margin_top, margin_bottom) = match level {
            HeadingLevel::H1 => (px(32.0), px(24.0), px(16.0)),
            HeadingLevel::H2 => (px(24.0), px(20.0), px(12.0)),
            HeadingLevel::H3 => (px(20.0), px(16.0), px(8.0)),
            HeadingLevel::H4 => (px(18.0), px(12.0), px(6.0)),
            HeadingLevel::H5 => (px(16.0), px(10.0), px(4.0)),
            HeadingLevel::H6 => (px(14.0), px(8.0), px(4.0)),
        };

        div()
            .child(text(text))
            .text_size(size)
            .font_weight(FontWeight::BOLD)
            .mt(margin_top)
            .mb(margin_bottom)
            .text_color(hsla(0.0, 0.0, 0.1, 1.0))
            .into_any_element()
    }

    fn create_inline_code_element(&self, code: &str) -> AnyElement {
        div()
            .display(Display::Inline)
            .child(text(code))
            .bg(hsla(0.0, 0.0, 0.95, 1.0))
            .px(px(4.0))
            .py(px(2.0))
            .rounded(px(3.0))
            .font_family("Monaco, Consolas, monospace")
            .text_size(px(14.0))
            .into_any_element()
    }

    fn create_code_block_element(&self, code: &str, _lang: Option<&str>) -> AnyElement {
        div()
            .w_full()
            .border(px(1.0))
            .border_color(hsla(0.0, 0.0, 0.8, 1.0))
            .bg(hsla(0.0, 0.0, 0.05, 1.0))
            .p(px(16.0))
            .rounded(px(6.0))
            .my(px(16.0))
            .child(
                div()
                    .child(text(code))
                    .font_family("Monaco, Consolas, monospace")
                    .text_size(px(14.0))
                    .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                    .line_height(relative(1.5))
            )
            .into_any_element()
    }

    fn create_blockquote_element(&self, text: &str) -> AnyElement {
        div()
            .border_l(px(4.0))
            .border_color(hsla(0.0, 0.0, 0.7, 1.0))
            .pl(px(16.0))
            .ml(px(8.0))
            .my(px(8.0))
            .child(text(text))
            .text_color(hsla(0.0, 0.0, 0.5, 1.0))
            .italic()
            .into_any_element()
    }

    fn create_rule_element(&self) -> AnyElement {
        div()
            .w_full()
            .h(px(1.0))
            .bg(hsla(0.0, 0.0, 0.8, 1.0))
            .my(px(24.0))
            .into_any_element()
    }
}
