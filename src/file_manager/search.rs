//! 文档搜索模块
//!
//! 提供文档内搜索功能：
//! - 关键词搜索
//! - 搜索结果高亮
//! - 搜索历史

use std::collections::HashMap;

/// 搜索结果
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// 行号（从1开始）
    pub line_number: usize,
    /// 行内容
    pub line_content: String,
    /// 匹配位置（字符索引）
    pub match_positions: Vec<(usize, usize)>,
    /// 预览文本（包含上下文）
    pub preview: String,
}

impl SearchResult {
    /// 创建新的搜索结果
    pub fn new(line_number: usize, line_content: String, match_positions: Vec<(usize, usize)>) -> Self {
        let preview = Self::generate_preview(&line_content, &match_positions);

        Self {
            line_number,
            line_content,
            match_positions,
            preview,
        }
    }

    /// 生成预览文本（包含上下文）
    fn generate_preview(line_content: &str, match_positions: &[(usize, usize)]) -> String {
        if match_positions.is_empty() {
            return String::new();
        }

        let (start, end) = match_positions[0];
        let line_len = line_content.len();

        // 添加上下文（前后20个字符）
        let context_start = start.saturating_sub(20);
        let context_end = (end + 20).min(line_len);

        let mut preview = String::new();

        // 如果开头被截断，添加省略号
        if context_start > 0 {
            preview.push_str("...");
        }

        preview.push_str(&line_content[context_start..context_end]);

        // 如果结尾被截断，添加省略号
        if context_end < line_len {
            preview.push_str("...");
        }

        preview
    }

    /// 获取高亮后的行内容（HTML格式）
    pub fn highlighted_content(&self) -> String {
        if self.match_positions.is_empty() {
            return html_escape(&self.line_content);
        }

        let mut result = String::new();
        let mut last_end = 0;

        for &(start, end) in &self.match_positions {
            // 添加匹配前的文本
            if start > last_end {
                result.push_str(&html_escape(&self.line_content[last_end..start]));
            }

            // 添加高亮的匹配文本
            result.push_str("<mark>");
            result.push_str(&html_escape(&self.line_content[start..end]));
            result.push_str("</mark>");

            last_end = end;
        }

        // 添加剩余文本
        if last_end < self.line_content.len() {
            result.push_str(&html_escape(&self.line_content[last_end..]));
        }

        result
    }
}

/// 搜索管理器
pub struct SearchManager {
    /// 搜索历史
    history: Vec<String>,
    /// 历史记录最大数量
    max_history_size: usize,
}

impl SearchManager {
    /// 创建新的搜索管理器
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            max_history_size: 50,
        }
    }

    /// 在文本中搜索关键词
    pub fn search(&mut self, query: &str, text: &str) -> Vec<SearchResult> {
        if query.is_empty() {
            return Vec::new();
        }

        // 添加到搜索历史
        self.add_to_history(query.to_string());

        let mut results = Vec::new();
        let query_lower = query.to_lowercase();

        for (line_idx, line) in text.lines().enumerate() {
            let line_lower = line.to_lowercase();

            // 查找所有匹配位置
            let mut match_positions = Vec::new();
            let mut search_start = 0;

            while let Some(pos) = line_lower[search_start..].find(&query_lower) {
                let actual_pos = search_start + pos;
                match_positions.push((actual_pos, actual_pos + query.len()));
                search_start = actual_pos + query.len();

                // 防止无限循环
                if search_start >= line_lower.len() {
                    break;
                }
            }

            // 如果有匹配，添加到结果
            if !match_positions.is_empty() {
                results.push(SearchResult::new(
                    line_idx + 1, // 行号从1开始
                    line.to_string(),
                    match_positions,
                ));
            }
        }

        results
    }

    /// 在文件中搜索关键词
    pub fn search_in_file(&mut self, query: &str, file_path: &std::path::Path) -> anyhow::Result<Vec<SearchResult>> {
        let content = std::fs::read_to_string(file_path)
            .map_err(|e| anyhow::anyhow!("无法读取文件 {}: {}", file_path.display(), e))?;

        Ok(self.search(query, &content))
    }

    /// 在多个文件中搜索关键词
    pub fn search_in_files(
        &mut self,
        query: &str,
        file_paths: &[std::path::PathBuf],
    ) -> HashMap<String, Vec<SearchResult>> {
        let mut all_results = HashMap::new();

        for file_path in file_paths {
            match self.search_in_file(query, file_path) {
                Ok(results) => {
                    if !results.is_empty() {
                        all_results.insert(
                            file_path.to_string_lossy().to_string(),
                            results,
                        );
                    }
                }
                Err(e) => {
                    eprintln!("搜索文件 {} 时出错: {}", file_path.display(), e);
                }
            }
        }

        all_results
    }

    /// 添加搜索到历史记录
    fn add_to_history(&mut self, query: String) {
        // 避免重复
        if let Some(pos) = self.history.iter().position(|q| q == &query) {
            self.history.remove(pos);
        }

        self.history.insert(0, query);

        // 限制历史记录数量
        if self.history.len() > self.max_history_size {
            self.history.truncate(self.max_history_size);
        }
    }

    /// 获取搜索历史
    pub fn history(&self) -> &[String] {
        &self.history
    }

    /// 清空搜索历史
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// 设置历史记录最大数量
    pub fn set_max_history_size(&mut self, size: usize) {
        self.max_history_size = size;
        if self.history.len() > size {
            self.history.truncate(size);
        }
    }
}

impl Default for SearchManager {
    fn default() -> Self {
        Self::new()
    }
}

/// HTML 转义函数
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_search_simple() {
        let mut manager = SearchManager::new();
        let text = "Hello world\nThis is a test\nHello again";

        let results = manager.search("Hello", text);

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].line_number, 1);
        assert_eq!(results[1].line_number, 3);
    }

    #[test]
    fn test_search_case_insensitive() {
        let mut manager = SearchManager::new();
        let text = "Hello world\nhello World\nHELLO";

        let results = manager.search("hello", text);

        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_search_multiple_matches_in_line() {
        let mut manager = SearchManager::new();
        let text = "test test test";

        let results = manager.search("test", text);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].match_positions.len(), 3);
    }

    #[test]
    fn test_search_empty_query() {
        let mut manager = SearchManager::new();
        let text = "Hello world";

        let results = manager.search("", text);

        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_no_matches() {
        let mut manager = SearchManager::new();
        let text = "Hello world";

        let results = manager.search("test", text);

        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_highlighted_content() {
        let result = SearchResult::new(
            1,
            "Hello world".to_string(),
            vec![(0, 5)],
        );

        let highlighted = result.highlighted_content();

        assert!(highlighted.contains("<mark>Hello</mark>"));
        assert!(highlighted.contains("world"));
    }

    #[test]
    fn test_search_history() {
        let mut manager = SearchManager::new();

        manager.search("test1", "content");
        manager.search("test2", "content");
        manager.search("test1", "content"); // 重复

        let history = manager.history();

        assert_eq!(history.len(), 2);
        assert_eq!(history[0], "test1"); // 重复的被移到前面
        assert_eq!(history[1], "test2");
    }

    #[test]
    fn test_search_in_file() -> anyhow::Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "Hello world")?;
        writeln!(temp_file, "This is a test")?;
        writeln!(temp_file, "Hello again")?;

        let mut manager = SearchManager::new();
        let results = manager.search_in_file("Hello", temp_file.path())?;

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].line_number, 1);
        assert_eq!(results[1].line_number, 3);

        Ok(())
    }

    #[test]
    fn test_generate_preview() {
        let result = SearchResult::new(
            1,
            "This is a long line with search term in the middle".to_string(),
            vec![(25, 35)], // "search term"
        );

        let preview = &result.preview;

        // 预览应该包含上下文
        assert!(preview.contains("search term"));
        assert!(preview.len() < "This is a long line with search term in the middle".len());
    }

    #[test]
    fn test_clear_history() {
        let mut manager = SearchManager::new();

        manager.search("test1", "content");
        manager.search("test2", "content");

        assert_eq!(manager.history().len(), 2);

        manager.clear_history();

        assert_eq!(manager.history().len(), 0);
    }

    #[test]
    fn test_max_history_size() {
        let mut manager = SearchManager::new();
        manager.set_max_history_size(3);

        manager.search("test1", "content");
        manager.search("test2", "content");
        manager.search("test3", "content");
        manager.search("test4", "content");
        manager.search("test5", "content");

        assert_eq!(manager.history().len(), 3);
        assert_eq!(manager.history()[0], "test5");
        assert_eq!(manager.history()[1], "test4");
        assert_eq!(manager.history()[2], "test3");
    }
}
