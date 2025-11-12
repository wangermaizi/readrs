// src/theme.rs
use gpui::*;

// 主题结构体
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
            background: hsl(0.0, 0.0, 1.0), // 白色
            text: hsl(0.0, 0.0, 0.0),       // 黑色
            primary: hsl(210.0, 0.15, 0.15), // 深蓝灰
            secondary: hsl(210.0, 0.1, 0.4), // 中蓝灰
            accent: hsl(210.0, 0.8, 0.5),   // 蓝色
            border: hsl(0.0, 0.0, 0.8),     // 浅灰
            editor_bg: hsl(0.0, 0.0, 0.95), // 浅灰白
            preview_bg: hsl(0.0, 0.0, 1.0), // 白色
            sidebar_bg: hsl(0.0, 0.0, 0.97), // 浅灰
            tab_active_bg: hsl(0.0, 0.0, 1.0), // 白色
            tab_inactive_bg: hsl(0.0, 0.0, 0.9), // 浅灰
        }
    }

    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            background: hsl(0.0, 0.0, 0.1),  // 深灰
            text: hsl(0.0, 0.0, 0.9),        // 浅灰白
            primary: hsl(210.0, 0.15, 0.85), // 浅蓝灰
            secondary: hsl(210.0, 0.1, 0.6), // 中蓝灰
            accent: hsl(210.0, 0.8, 0.5),    // 蓝色
            border: hsl(0.0, 0.0, 0.2),      // 深灰
            editor_bg: hsl(0.0, 0.0, 0.15),  // 中等灰
            preview_bg: hsl(0.0, 0.0, 0.1),  // 深灰
            sidebar_bg: hsl(0.0, 0.0, 0.12), // 深灰
            tab_active_bg: hsl(0.0, 0.0, 0.1), // 深灰
            tab_inactive_bg: hsl(0.0, 0.0, 0.2), // 中等灰
        }
    }

    pub fn solarized_light() -> Self {
        Self {
            name: "Solarized Light".to_string(),
            background: hsl(45.0, 0.15, 0.95), // 奶油色
            text: hsl(45.0, 0.3, 0.2),         // 深蓝灰
            primary: hsl(210.0, 0.3, 0.4),     // 蓝色
            secondary: hsl(210.0, 0.2, 0.6),   // 浅蓝
            accent: hsl(45.0, 0.8, 0.5),       // 黄色
            border: hsl(45.0, 0.2, 0.8),       // 浅奶油色
            editor_bg: hsl(45.0, 0.15, 0.9),   // 浅奶油色
            preview_bg: hsl(45.0, 0.15, 0.95), // 奶油色
            sidebar_bg: hsl(45.0, 0.15, 0.92), // 奶油色
            tab_active_bg: hsl(45.0, 0.15, 0.95), // 奶油色
            tab_inactive_bg: hsl(45.0, 0.15, 0.85), // 浅奶油色
        }
    }

    pub fn solarized_dark() -> Self {
        Self {
            name: "Solarized Dark".to_string(),
            background: hsl(45.0, 0.15, 0.1),  // 深奶油色
            text: hsl(45.0, 0.3, 0.8),         // 浅蓝灰
            primary: hsl(210.0, 0.3, 0.6),     // 浅蓝色
            secondary: hsl(210.0, 0.2, 0.4),   // 蓝色
            accent: hsl(45.0, 0.8, 0.5),       // 黄色
            border: hsl(45.0, 0.2, 0.2),       // 深奶油色
            editor_bg: hsl(45.0, 0.15, 0.15),  // 中等奶油色
            preview_bg: hsl(45.0, 0.15, 0.1),  // 深奶油色
            sidebar_bg: hsl(45.0, 0.15, 0.12), // 中等奶油色
            tab_active_bg: hsl(45.0, 0.15, 0.1), // 深奶油色
            tab_inactive_bg: hsl(45.0, 0.15, 0.2), // 浅奶油色
        }
    }
}

// 主题管理器
pub struct ThemeManager {
    themes: Vec<Theme>,
    current_theme_index: usize,
}

impl ThemeManager {
    pub fn new() -> Self {
        let themes = vec![
            Theme::light(),
            Theme::dark(),
            Theme::solarized_light(),
            Theme::solarized_dark(),
        ];
        
        Self {
            themes,
            current_theme_index: 0,
        }
    }

    pub fn current_theme(&self) -> &Theme {
        &self.themes[self.current_theme_index]
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

    pub fn theme_count(&self) -> usize {
        self.themes.len()
    }

    pub fn theme_names(&self) -> Vec<String> {
        self.themes.iter().map(|t| t.name.clone()).collect()
    }
}

// 字体设置
#[derive(Debug, Clone)]
pub struct FontSettings {
    pub editor_font_family: String,
    pub editor_font_size: f32,
    pub preview_font_family: String,
    pub preview_font_size: f32,
    pub line_height: f32,
}

impl Default for FontSettings {
    fn default() -> Self {
        Self {
            editor_font_family: "Monaco, Consolas, monospace".to_string(),
            editor_font_size: 14.0,
            preview_font_family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif".to_string(),
            preview_font_size: 16.0,
            line_height: 1.6,
        }
    }
}

// 界面布局设置
#[derive(Debug, Clone)]
pub struct LayoutSettings {
    pub show_toolbar: bool,
    pub show_status_bar: bool,
    pub editor_width: f32,  // 0.0 - 1.0
    pub preview_width: f32, // 0.0 - 1.0
    pub focus_mode: bool,   // 专注模式
    pub typewriter_mode: bool, // 打字机模式
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self {
            show_toolbar: true,
            show_status_bar: true,
            editor_width: 0.5,
            preview_width: 0.5,
            focus_mode: false,
            typewriter_mode: false,
        }
    }
}

// 应用设置
pub struct AppSettings {
    pub theme_manager: ThemeManager,
    pub font_settings: FontSettings,
    pub layout_settings: LayoutSettings,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme_manager: ThemeManager::new(),
            font_settings: FontSettings::default(),
            layout_settings: LayoutSettings::default(),
        }
    }
}

// 主题选择器组件
pub struct ThemeSelector {
    theme_manager: ThemeManager,
}

impl ThemeSelector {
    pub fn new(theme_manager: ThemeManager) -> Self {
        Self { theme_manager }
    }
}

impl Render for ThemeSelector {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .p_3()
            .bg(gpui::white())
            .border_1()
            .border_color(gpui::gray_300())
            .rounded_sm()
            .child(Label::new("Theme Selector"))
            .child(
                div()
                    .mt_2()
                    .flex()
                    .gap_2()
                    .child(
                        Button::new("prev", "<")
                            .on_click(|_, cx| {
                                cx.spawn(|_, _| async move {
                                    // TODO: 切换到上一个主题
                                }).detach();
                            })
                    )
                    .child(
                        Label::new(self.theme_manager.current_theme().name.clone())
                            .px_2()
                            .py_1()
                            .bg(gpui::gray_100())
                            .rounded_sm()
                    )
                    .child(
                        Button::new("next", ">")
                            .on_click(|_, cx| {
                                cx.spawn(|_, _| async move {
                                    // TODO: 切换到下一个主题
                                }).detach();
                            })
                    )
            )
    }
}