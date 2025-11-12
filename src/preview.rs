// src/preview.rs
use gpui::*;
use std::sync::Arc;

mod renderer;
use renderer::MarkdownRenderer;

pub struct MarkdownPreview {
    markdown_text: SharedString,
}

impl MarkdownPreview {
    pub fn new(markdown_text: SharedString, cx: &mut ViewContext<Self>) -> Self {
        cx.subscribe(&cx.window_handle(), Self::handle_window_events).detach();
        Self {
            markdown_text,
        }
    }

    fn handle_window_events(
        &mut self,
        _window: View<MainWindow>,
        event: &EditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        self.markdown_text = event.text.clone();
        cx.notify();
    }
}

impl Render for MarkdownPreview {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let renderer = MarkdownRenderer::new(self.markdown_text.clone());
        let elements = renderer.render_to_elements();
        
        div()
            .size_full()
            .p_4()
            .bg(gpui::white())
            .child(
                div()
                    .size_full()
                    .border_1()
                    .border_color(gpui::gray_300())
                    .p_3()
                    .children(elements)
            )
            .child(
                div()
                    .mt_2()
                    .text_xs()
                    .text_color(gpui::gray_500())
                    .child(Label::new("Markdown Preview - Real-time rendering"))
            )
    }
}

// 为了使代码能编译，添加缺少的引用
use crate::MainWindow;
use crate::editor::EditorEvent;
use gpui::Label;