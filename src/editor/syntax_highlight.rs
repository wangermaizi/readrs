//! 代码语法高亮模块
//!
//! 使用 syntect 为代码块提供语法高亮

use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

/// 代码语法高亮器
pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl SyntaxHighlighter {
    /// 创建新的语法高亮器
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    /// 高亮代码
    ///
    /// # 参数
    /// - `code`: 代码内容
    /// - `language`: 语言类型（如 "rust", "python", "javascript"）
    ///
    /// # 返回
    /// 高亮后的 HTML 字符串
    pub fn highlight(&self, code: &str, language: &str) -> String {
        let syntax = self
            .syntax_set
            .find_syntax_by_token(language)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let theme = &self.theme_set.themes["InspiredGitHub"];
        let mut highlighter = HighlightLines::new(syntax, theme);

        let mut html_output = String::new();
        html_output.push_str("<pre style=\"background-color: #f5f5f5; padding: 1em; border-radius: 4px; overflow-x: auto;\">");

        for line in LinesWithEndings::from(code) {
            let ranges: Vec<(Style, &str)> = highlighter.highlight_line(line, &self.syntax_set).unwrap();
            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            html_output.push_str(&escaped);
        }

        html_output.push_str("</pre>");
        html_output
    }

    /// 获取支持的语言列表
    pub fn supported_languages(&self) -> Vec<String> {
        self.syntax_set
            .syntaxes()
            .iter()
            .filter_map(|s| s.file_extensions.first().map(|e| e.to_string()))
            .collect()
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_rust() {
        let highlighter = SyntaxHighlighter::new();
        let code = r#"fn main() {
    println!("Hello, world!");
}"#;
        let result = highlighter.highlight(code, "rust");
        assert!(result.contains("fn"));
        assert!(result.contains("main"));
    }

    #[test]
    fn test_highlight_python() {
        let highlighter = SyntaxHighlighter::new();
        let code = r#"def hello():
    print("Hello, world!")"#;
        let result = highlighter.highlight(code, "python");
        assert!(result.contains("def"));
        assert!(result.contains("hello"));
    }

    #[test]
    fn test_supported_languages() {
        let highlighter = SyntaxHighlighter::new();
        let languages = highlighter.supported_languages();
        assert!(languages.contains(&"rs".to_string()));
        assert!(languages.contains(&"py".to_string()));
        assert!(languages.contains(&"js".to_string()));
    }
}