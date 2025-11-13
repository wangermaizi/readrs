// src/features.rs
use gpui::*;
use pulldown_cmark::{Parser, Event, Tag, TagEnd};
use std::collections::HashMap;

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
        
        // 简单解析：查找以#开头的行
        for (line_num, line) in markdown.lines().enumerate() {
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

    pub fn render_content(&self) -> Vec<AnyElement> {
        self.headings.iter().map(|heading| {
            let indent = (heading.level - 1) * 16;
            let heading_text = heading.text.clone();
            div()
                .flex()
                .pl(px(indent as f32))
                .p(px(4.0))
                .rounded(px(6.0))
                .hover(|style| style.bg(hsla(0.0, 0.0, 0.95, 1.0)))
                .cursor_pointer()
                .child(
                    div()
                        .child(text(&heading_text))
                        .text_size(px(14.0))
                        .text_color(hsla(210.0, 0.8, 0.5, 1.0))
                )
                .into_any_element()
        }).collect()
    }
}

impl Render for OutlineView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p(px(12.0))
            .flex()
            .flex_col()
            .child(
                div()
                    .child(text("大纲视图"))
                    .text_size(px(16.0))
                    .font_weight(FontWeight::BOLD)
                    .mb(px(12.0))
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .children(self.render_content())
            )
    }
}

pub struct SpellChecker {
    dictionary: HashMap<String, bool>,
    enabled: bool,
}

impl SpellChecker {
    pub fn new() -> Self {
        let mut dictionary = HashMap::new();
        let common_words = vec![
            "the", "be", "to", "of", "and", "a", "in", "that", "have", "I",
        ];
        
        for word in common_words {
            dictionary.insert(word.to_lowercase(), true);
        }
        
        Self {
            dictionary,
            enabled: true,
        }
    }

    pub fn check_word(&self, word: &str) -> bool {
        if !self.enabled {
            return true;
        }
        self.dictionary.contains_key(&word.to_lowercase())
    }
}

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
}

impl ShortcutManager {
    pub fn new() -> Self {
        let mut shortcuts = HashMap::new();
        shortcuts.insert("Ctrl+N".to_string(), ShortcutAction::NewFile);
        shortcuts.insert("Ctrl+O".to_string(), ShortcutAction::OpenFile);
        shortcuts.insert("Ctrl+S".to_string(), ShortcutAction::SaveFile);
        shortcuts.insert("Ctrl+E".to_string(), ShortcutAction::ExportFile);
        shortcuts.insert("Ctrl+T".to_string(), ShortcutAction::ToggleTheme);
        
        Self { shortcuts }
    }
}
