// src/preview.rs
use gpui::*;
use crate::renderer::MarkdownRenderer;

pub struct MarkdownPreview {
    markdown_text: SharedString,
}

impl MarkdownPreview {
    pub fn new() -> Self {
        Self {
            markdown_text: String::new().into(),
        }
    }

    pub fn update_text(&mut self, text: SharedString) {
        self.markdown_text = text;
    }
}

impl Render for MarkdownPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::default();
        let renderer = MarkdownRenderer::new(self.markdown_text.clone());
        let elements = renderer.render_to_elements();
        
        div()
            .size_full()
            .p(px(16.0))
            .bg(theme.preview_bg)
            .overflow_y_scroll()
            .child(
                div()
                    .size_full()
                    .border(px(1.0))
                    .border_color(theme.border)
                    .rounded(px(6.0))
                    .p(px(24.0))
                    .bg(theme.preview_bg)
                    .children(elements)
            )
    }
}

struct Theme {
    preview_bg: Hsla,
    border: Hsla,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            preview_bg: hsla(0.0, 0.0, 1.0, 1.0), // 白色
            border: hsla(0.0, 0.0, 0.8, 1.0), // 浅灰
        }
    }
}
