// src/features.rs
use gpui::*;
use regex::Regex;
use std::collections::HashMap;

// 拼写检查器
pub struct SpellChecker {
    dictionary: HashMap<String, bool>, // 简化的词典实现
    enabled: bool,
}

impl SpellChecker {
    pub fn new() -> Self {
        let mut dictionary = HashMap::new();
        // 添加一些常用英文单词到词典中
        let common_words = vec![
            "the", "be", "to", "of", "and", "a", "in", "that", "have", "I",
            "it", "for", "not", "on", "with", "he", "as", "you", "do", "at",
            "this", "but", "his", "by", "from", "they", "we", "say", "her", "she",
            "or", "an", "will", "my", "one", "all", "would", "there", "their",
            "what", "so", "up", "out", "if", "about", "who", "get", "which", "go",
            "me", "when", "make", "can", "like", "time", "no", "just", "him", "know",
            "take", "people", "into", "year", "your", "good", "some", "could", "them",
            "see", "other", "than", "then", "now", "look", "only", "come", "its",
            "over", "think", "also", "back", "after", "use", "two", "how", "our",
            "work", "first", "well", "way", "even", "new", "want", "because", "any",
            "these", "give", "day", "most", "us"
        ];
        
        for word in common_words {
            dictionary.insert(word.to_lowercase(), true);
        }
        
        Self {
            dictionary,
            enabled: true,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn check_word(&self, word: &str) -> bool {
        if !self.enabled {
            return true;
        }
        
        // 简单的拼写检查实现
        let clean_word = word.to_lowercase();
        self.dictionary.contains_key(&clean_word)
    }

    pub fn check_text(&self, text: &str) -> Vec<SpellError> {
        if !self.enabled {
            return vec![];
        }
        
        let mut errors = Vec::new();
        let word_regex = Regex::new(r"\b[a-zA-Z]+\b").unwrap();
        
        for mat in word_regex.find_iter(text) {
            let word = mat.as_str();
            if !self.check_word(word) {
                errors.push(SpellError {
                    word: word.to_string(),
                    position: mat.start(),
                    length: mat.end() - mat.start(),
                });
            }
        }
        
        errors
    }
}

#[derive(Debug, Clone)]
pub struct SpellError {
    pub word: String,
    pub position: usize,
    pub length: usize,
}

// 大纲视图事件
#[derive(Debug, Clone)]
pub struct OutlineNavigationEvent {
    pub line_number: usize,
}

// 大纲视图
pub struct OutlineView {
    headings: Vec<Heading>,
}

#[derive(Debug, Clone)]
pub struct Heading {
    pub level: usize,
    pub text: String,
    pub line_number: usize,
}

impl OutlineView {
    pub fn new() -> Self {
        Self {
            headings: vec![],
        }
    }

    pub fn update_from_markdown(&mut self, markdown: &str) {
        self.headings.clear();
        let lines: Vec<&str> = markdown.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            if line.starts_with('#') {
                let mut level = 0;
                let mut content_start = 0;
                
                for (i, ch) in line.chars().enumerate() {
                    if ch == '#' {
                        level += 1;
                    } else if ch != ' ' {
                        content_start = i;
                        break;
                    }
                }
                
                if level > 0 && level <= 6 {
                    let text = line[content_start..].trim().to_string();
                    self.headings.push(Heading {
                        level,
                        text,
                        line_number: line_num,
                    });
                }
            }
        }
    }

    pub fn get_headings(&self) -> &[Heading] {
        &self.headings
    }
}

impl Render for OutlineView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p_3()
            .child(
                div()
                    .text_lg()
                    .font_bold()
                    .mb_3()
                    .child(Label::new("大纲视图"))
            )
            .child(
                div()
                    .flex_col()
                    .gap_1()
                    .children(self.headings.iter().map(|heading| {
                        let indent = (heading.level - 1) * 16;
                        let line_number = heading.line_number;
                        div()
                            .pl(px(indent as f32))
                            .child(
                                Button::new(heading.line_number.to_string(), &heading.text)
                                    .style(ButtonStyle::Subtle)
                                    .on_click(move |_, cx| {
                                        cx.emit(OutlineNavigationEvent {
                                            line_number,
                                        });
                                    })
                            )
                    }))
            )
    }
}

// 文件列表视图
pub struct FileListView {
    recent_files: Vec<String>,
}

impl FileListView {
    pub fn new() -> Self {
        Self {
            recent_files: vec![
                "readme.md".to_string(),
                "guide.md".to_string(),
                "notes.md".to_string(),
                "blog_post.md".to_string(),
                "documentation.md".to_string(),
                "tutorial.md".to_string(),
                "spec.md".to_string(),
            ],
        }
    }

    pub fn add_recent_file(&mut self, file_path: String) {
        // 如果文件已经存在，先移除它
        self.recent_files.retain(|f| f != &file_path);
        // 将文件添加到列表开头
        self.recent_files.insert(0, file_path);
        
        // 限制最近文件数量
        if self.recent_files.len() > 20 {
            self.recent_files.truncate(20);
        }
    }

    pub fn get_recent_files(&self) -> &[String] {
        &self.recent_files
    }
}

// 文件列表事件
#[derive(Debug, Clone)]
pub struct FileOpenEvent {
    pub file_path: String,
}

impl Render for FileListView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p_3()
            .child(
                div()
                    .text_lg()
                    .font_bold()
                    .mb_3()
                    .child(Label::new("最近打开"))
            )
            .child(
                div()
                    .flex_col()
                    .gap_1()
                    .children(self.recent_files.iter().map(|file| {
                        let file_path = file.clone();
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .cursor_pointer()
                            .on_click(move |_, cx| {
                                cx.emit(FileOpenEvent {
                                    file_path: file_path.clone(),
                                });
                            })
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(div().text_color(gpui::gray_500()).child(Label::new("📄")))
                                    .child(Label::new(file))
                            )
                    }))
            )
    }
}

// 快捷键管理器
pub struct ShortcutManager {
    shortcuts: HashMap<String, ShortcutAction>,
}

#[derive(Debug, Clone)]
pub enum ShortcutAction {
    NewFile,
    OpenFile,
    SaveFile,
    ExportFile,
    ToggleTheme,
    ToggleSpellCheck,
    ToggleFocusMode,
    InsertTable,
    InsertCodeBlock,
    ToggleSourceMode,
}

impl ShortcutManager {
    pub fn new() -> Self {
        let mut shortcuts = HashMap::new();
        
        // 注册默认快捷键
        shortcuts.insert("Ctrl+N".to_string(), ShortcutAction::NewFile);
        shortcuts.insert("Ctrl+O".to_string(), ShortcutAction::OpenFile);
        shortcuts.insert("Ctrl+S".to_string(), ShortcutAction::SaveFile);
        shortcuts.insert("Ctrl+E".to_string(), ShortcutAction::ExportFile);
        shortcuts.insert("Ctrl+T".to_string(), ShortcutAction::ToggleTheme);
        shortcuts.insert("Ctrl+Shift+S".to_string(), ShortcutAction::ToggleSpellCheck);
        shortcuts.insert("F11".to_string(), ShortcutAction::ToggleFocusMode);
        shortcuts.insert("Ctrl+Shift+T".to_string(), ShortcutAction::InsertTable);
        shortcuts.insert("Ctrl+Shift+C".to_string(), ShortcutAction::InsertCodeBlock);
        shortcuts.insert("Ctrl+/".to_string(), ShortcutAction::ToggleSourceMode);
        
        Self { shortcuts }
    }

    pub fn get_action(&self, shortcut: &str) -> Option<&ShortcutAction> {
        self.shortcuts.get(shortcut)
    }

    pub fn register_shortcut(&mut self, shortcut: String, action: ShortcutAction) {
        self.shortcuts.insert(shortcut, action);
    }

    pub fn get_shortcuts(&self) -> &HashMap<String, ShortcutAction> {
        &self.shortcuts
    }
}

// 搜索功能
pub struct SearchManager {
    search_term: String,
    case_sensitive: bool,
    whole_word: bool,
}

impl SearchManager {
    pub fn new() -> Self {
        Self {
            search_term: String::new(),
            case_sensitive: false,
            whole_word: false,
        }
    }

    pub fn set_search_term(&mut self, term: String) {
        self.search_term = term;
    }

    pub fn toggle_case_sensitive(&mut self) {
        self.case_sensitive = !self.case_sensitive;
    }

    pub fn toggle_whole_word(&mut self) {
        self.whole_word = !self.whole_word;
    }

    pub fn search_in_text(&self, text: &str) -> Vec<SearchResult> {
        if self.search_term.is_empty() {
            return vec![];
        }
        
        let mut results = Vec::new();
        let search_text = if self.case_sensitive {
            text.to_string()
        } else {
            text.to_lowercase()
        };
        
        let search_term = if self.case_sensitive {
            self.search_term.clone()
        } else {
            self.search_term.to_lowercase()
        };
        
        let mut pos = 0;
        while let Some(found_pos) = search_text[pos..].find(&search_term) {
            let actual_pos = pos + found_pos;
            results.push(SearchResult {
                position: actual_pos,
                length: search_term.len(),
            });
            pos = actual_pos + search_term.len();
        }
        
        results
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub position: usize,
    pub length: usize,
}

// 搜索面板组件
pub struct SearchPanel {
    search_manager: SearchManager,
    is_visible: bool,
}

impl SearchPanel {
    pub fn new() -> Self {
        Self {
            search_manager: SearchManager::new(),
            is_visible: false,
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Render for SearchPanel {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }
        
        div()
            .absolute()
            .top_10()
            .right_4()
            .w_80()
            .bg(gpui::white())
            .border_1()
            .border_color(gpui::gray_300())
            .rounded_sm()
            .p_3()
            .child(Label::new("Search"))
            .child(
                div()
                    .mt_2()
                    .child(
                        input("search-input")
                            .placeholder("Enter search term...")
                            .w_full()
                    )
            )
            .child(
                div()
                    .mt_2()
                    .flex()
                    .gap_2()
                    .child(
                        Button::new("case", "Aa")
                            .style(ButtonStyle::Subtle)
                    )
                    .child(
                        Button::new("word", "Word")
                            .style(ButtonStyle::Subtle)
                    )
                    .child(
                        Button::new("prev", "Previous")
                            .style(ButtonStyle::Subtle)
                    )
                    .child(
                        Button::new("next", "Next")
                            .style(ButtonStyle::Subtle)
                    )
            )
            .into_any_element()
    }
}