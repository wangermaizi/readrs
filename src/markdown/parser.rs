//! Markdown 解析器
//! 
//! 使用 pulldown-cmark 解析 Markdown 文本为 HTML

use pulldown_cmark::{Parser, Options, html};

/// Markdown 解析器
pub struct MarkdownParser;

impl MarkdownParser {
    /// 解析 Markdown 文本为 HTML
    /// 
    /// # 参数
    /// - `markdown`: Markdown 文本内容
    /// 
    /// # 返回
    /// 解析后的 HTML 字符串
    pub fn parse_to_html(markdown: &str) -> String {
        // 启用所有 CommonMark 扩展选项
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);

        // 创建解析器
        let parser = Parser::new_ext(markdown, options);

        // 将解析结果转换为 HTML
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    }

    /// 解析 Markdown 文本，返回解析后的 HTML 片段
    /// 
    /// 这个方法会添加基本的 HTML 结构，包括样式
    pub fn parse_with_styles(markdown: &str) -> String {
        let html_content = Self::parse_to_html(markdown);
        
        // 添加基本的样式和结构
        format!(
            r#"
            <div class="markdown-preview" style="
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Helvetica Neue', Arial, sans-serif;
                line-height: 1.6;
                color: #333;
                padding: 20px;
                max-width: 100%;
            ">
                <style>
                    .markdown-preview h1 {{ font-size: 2em; font-weight: bold; margin-top: 0.67em; margin-bottom: 0.67em; }}
                    .markdown-preview h2 {{ font-size: 1.5em; font-weight: bold; margin-top: 0.83em; margin-bottom: 0.83em; }}
                    .markdown-preview h3 {{ font-size: 1.17em; font-weight: bold; margin-top: 1em; margin-bottom: 1em; }}
                    .markdown-preview h4 {{ font-size: 1em; font-weight: bold; margin-top: 1.33em; margin-bottom: 1.33em; }}
                    .markdown-preview h5 {{ font-size: 0.83em; font-weight: bold; margin-top: 1.67em; margin-bottom: 1.67em; }}
                    .markdown-preview h6 {{ font-size: 0.67em; font-weight: bold; margin-top: 2.33em; margin-bottom: 2.33em; }}
                    .markdown-preview p {{ margin-top: 1em; margin-bottom: 1em; }}
                    .markdown-preview ul, .markdown-preview ol {{ margin-top: 1em; margin-bottom: 1em; padding-left: 2em; }}
                    .markdown-preview li {{ margin-top: 0.5em; margin-bottom: 0.5em; }}
                    .markdown-preview blockquote {{ 
                        border-left: 4px solid #ddd; 
                        padding-left: 1em; 
                        margin-left: 0; 
                        color: #666; 
                        margin-top: 1em; 
                        margin-bottom: 1em;
                    }}
                    .markdown-preview code {{
                        background-color: #f5f5f5;
                        padding: 2px 4px;
                        border-radius: 3px;
                        font-family: 'Courier New', monospace;
                        font-size: 0.9em;
                    }}
                    .markdown-preview pre {{
                        background-color: #f5f5f5;
                        padding: 1em;
                        border-radius: 4px;
                        overflow-x: auto;
                        margin-top: 1em;
                        margin-bottom: 1em;
                    }}
                    .markdown-preview pre code {{
                        background-color: transparent;
                        padding: 0;
                    }}
                    .markdown-preview strong {{ font-weight: bold; }}
                    .markdown-preview em {{ font-style: italic; }}
                    .markdown-preview a {{ color: #0066cc; text-decoration: none; }}
                    .markdown-preview a:hover {{ text-decoration: underline; }}
                    .markdown-preview hr {{
                        border: none;
                        border-top: 1px solid #ddd;
                        margin: 2em 0;
                    }}
                </style>
                {}
            </div>
            "#,
            html_content
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heading() {
        let markdown = "# Hello World";
        let html = MarkdownParser::parse_to_html(markdown);
        assert!(html.contains("<h1>"));
        assert!(html.contains("Hello World"));
    }

    #[test]
    fn test_parse_bold() {
        let markdown = "**bold text**";
        let html = MarkdownParser::parse_to_html(markdown);
        assert!(html.contains("<strong>"));
        assert!(html.contains("bold text"));
    }

    #[test]
    fn test_parse_list() {
        let markdown = "- item 1\n- item 2";
        let html = MarkdownParser::parse_to_html(markdown);
        assert!(html.contains("<ul>"));
        assert!(html.contains("item 1"));
        assert!(html.contains("item 2"));
    }
}

