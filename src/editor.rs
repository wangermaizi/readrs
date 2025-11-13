// src/editor.rs
use gpui::*;

pub struct MarkdownEditor {
    text: SharedString,
}

impl MarkdownEditor {
    pub fn new() -> Self {
        Self {
            text: String::new().into(),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text.into();
    }

    pub fn get_text(&self) -> SharedString {
        self.text.clone()
    }

    pub fn update_text(&mut self, text: SharedString) {
        self.text = text;
    }
}

impl Render for MarkdownEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::default();
        
        div()
            .size_full()
            .p(px(16.0))
            .bg(theme.editor_bg)
            .child(
                div()
                    .size_full()
                    .border(px(1.0))
                    .border_color(theme.border)
                    .rounded(px(6.0))
                    .p(px(12.0))
                    .bg(theme.editor_bg)
                    .child(
                        text(self.text.clone())
                            .font_family("Monaco, Consolas, monospace")
                            .text_size(px(14.0))
                            .line_height(relative(1.5))
                            .w_full()
                            .h_full()
                    )
            )
    }
}

// 临时主题定义，后续会从theme模块导入
struct Theme {
    editor_bg: Hsla,
    border: Hsla,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            editor_bg: hsla(0.0, 0.0, 1.0, 1.0), // 白色
            border: hsla(0.0, 0.0, 0.8, 1.0), // 浅灰
        }
    }
}
