//! 文本编辑器组件
//! 
//! 使用 gpui-component 的 Input 组件实现多行文本编辑器

use gpui::*;
use gpui_component::{input::*, *};

/// 文本编辑器视图
/// 
/// 提供多行文本编辑功能，支持 Markdown 语法编辑
pub struct TextEditor {
    /// 输入状态管理
    input_state: Entity<InputState>,
    /// 当前文本内容
    content: SharedString,
}

impl TextEditor {
    /// 创建新的文本编辑器
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        // 创建多行输入状态，支持自动增长
        let input_state = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line()  // 启用多行模式
                .auto_grow(10, 50)  // 自动增长，最小 10 行，最大 50 行
                .placeholder("在这里输入 Markdown 内容...\n\n支持标题、列表、引用、代码块等语法。")
        });

        Self {
            input_state,
            content: SharedString::default(),
        }
    }

    /// 获取当前文本内容
    pub fn content(&self, cx: &Context<Self>) -> SharedString {
        self.input_state.read(cx).value()
    }

    /// 设置文本内容
    pub fn set_content(&mut self, content: impl Into<SharedString>, window: &mut Window, cx: &mut Context<Self>) {
        let content = content.into();
        self.content = content.clone();
        self.input_state.update(cx, |state, cx| {
            state.set_value(content.to_string(), window, cx);
        });
    }

    /// 订阅输入变化事件
    /// 
    /// 当用户输入内容时，会触发回调函数
    pub fn subscribe_changes<F>(&self, window: &mut Window, cx: &mut Context<Self>, callback: F)
    where
        F: Fn(SharedString, &mut Window, &mut Context<Self>) + 'static,
    {
        let callback = Box::new(callback);
        cx.subscribe_in(&self.input_state, window, move |_view, state, event, window, cx| {
            if let InputEvent::Change = event {
                let content = state.read(cx).value();
                callback(content, window, cx);
            }
        });
    }
}

impl Render for TextEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // 使用 Input 组件渲染多行文本编辑器
        Input::new(&self.input_state)
            .h_full()  // 占据全部可用高度
            .w_full()  // 占据全部可用宽度
    }
}

