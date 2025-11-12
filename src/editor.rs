// src/editor.rs
use gpui::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct EditorEvent {
    pub text: SharedString,
}

pub struct MarkdownEditor {
    text: SharedString,
    cursor_pos: usize,
}

impl MarkdownEditor {
    pub fn new(text: SharedString, cx: &mut ViewContext<Self>) -> Self {
        cx.subscribe(&cx.window_handle(), Self::handle_window_events).detach();
        Self {
            text,
            cursor_pos: 0,
        }
    }

    fn handle_window_events(
        &mut self,
        _window: View<MainWindow>,
        event: &EditorEvent,
        _cx: &mut ViewContext<Self>,
    ) {
        self.text = event.text.clone();
    }
}

impl Render for MarkdownEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
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
                    .child(
                        gpui::TextElement::new(self.text.clone())
                            .color(gpui::black())
                            .size(gpui::TextSize::Medium)
                    )
                    .on_mouse_down(MouseButton::Left, |_, cx| {
                        cx.focus_self();
                    })
                    .on_key_event("Backspace", |_, _, cx| {
                        // 简单的删除逻辑
                        let mut text: String = cx.view().read(cx).text.to_string();
                        if text.len() > 0 {
                            text.pop();
                            cx.emit(EditorEvent {
                                text: text.into()
                            });
                        }
                    })
                    .on_key_event("a", Modifiers::command(), |_, _, cx| {
                        // 全选
                        cx.propagate();
                    })
            )
            .child(
                div()
                    .mt_2()
                    .text_xs()
                    .text_color(gpui::gray_500())
                    .child(Label::new("Markdown Editor - Type markdown syntax here"))
            )
    }
}

// 为了使代码能编译，添加缺少的引用
use crate::MainWindow;
use gpui::Label;