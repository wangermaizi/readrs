//! 文本编辑器组件
//! 
//! 使用 gpui-component 的 Input 组件实现多行文本编辑器

use gpui::*;
use gpui_component::input::{InputState, Input};

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

    /// 获取输入状态的实体引用，用于订阅变化事件
    pub fn input_state(&self) -> Entity<InputState> {
        self.input_state.clone()
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