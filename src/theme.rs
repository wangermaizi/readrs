// src/theme.rs
use gpui::*;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub background: Hsla,
    pub text: Hsla,
    pub primary: Hsla,
    pub secondary: Hsla,
    pub accent: Hsla,
    pub border: Hsla,
    pub editor_bg: Hsla,
    pub preview_bg: Hsla,
    pub sidebar_bg: Hsla,
    pub tab_active_bg: Hsla,
    pub tab_inactive_bg: Hsla,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            name: "Light".to_string(),
            background: hsla(0.0, 0.0, 0.98, 1.0), // 浅灰白
            text: hsla(0.0, 0.0, 0.1, 1.0), // 深灰
            primary: hsla(210.0, 0.8, 0.5, 1.0), // 蓝色
            secondary: hsla(210.0, 0.1, 0.4, 1.0), // 中蓝灰
            accent: hsla(210.0, 0.8, 0.5, 1.0), // 蓝色
            border: hsla(0.0, 0.0, 0.9, 1.0), // 浅灰
            editor_bg: hsla(0.0, 0.0, 1.0, 1.0), // 白色
            preview_bg: hsla(0.0, 0.0, 1.0, 1.0), // 白色
            sidebar_bg: hsla(0.0, 0.0, 1.0, 1.0), // 白色
            tab_active_bg: hsla(0.0, 0.0, 1.0, 1.0), // 白色
            tab_inactive_bg: hsla(0.0, 0.0, 0.95, 1.0), // 浅灰
        }
    }

    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            background: hsla(0.0, 0.0, 0.1, 1.0), // 深灰
            text: hsla(0.0, 0.0, 0.9, 1.0), // 浅灰白
            primary: hsla(210.0, 0.8, 0.5, 1.0), // 蓝色
            secondary: hsla(210.0, 0.1, 0.6, 1.0), // 中蓝灰
            accent: hsla(210.0, 0.8, 0.5, 1.0), // 蓝色
            border: hsla(0.0, 0.0, 0.2, 1.0), // 深灰
            editor_bg: hsla(0.0, 0.0, 0.15, 1.0), // 中等灰
            preview_bg: hsla(0.0, 0.0, 0.1, 1.0), // 深灰
            sidebar_bg: hsla(0.0, 0.0, 0.12, 1.0), // 深灰
            tab_active_bg: hsla(0.0, 0.0, 0.1, 1.0), // 深灰
            tab_inactive_bg: hsla(0.0, 0.0, 0.2, 1.0), // 中等灰
        }
    }
}

pub struct ThemeManager {
    themes: Vec<Theme>,
    current_theme_index: usize,
}

impl ThemeManager {
    pub fn new() -> Self {
        let themes = vec![
            Theme::light(),
            Theme::dark(),
        ];
        
        Self {
            themes,
            current_theme_index: 0,
        }
    }

    pub fn current_theme(&self) -> Theme {
        self.themes[self.current_theme_index].clone()
    }

    pub fn next_theme(&mut self) {
        self.current_theme_index = (self.current_theme_index + 1) % self.themes.len();
    }

    pub fn previous_theme(&mut self) {
        self.current_theme_index = if self.current_theme_index == 0 {
            self.themes.len() - 1
        } else {
            self.current_theme_index - 1
        };
    }

    pub fn set_theme(&mut self, index: usize) {
        if index < self.themes.len() {
            self.current_theme_index = index;
        }
    }
}

