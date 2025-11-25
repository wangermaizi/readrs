//! 编辑器模块
//! 
//! 提供 Markdown 文本编辑功能，包括：
//! - 多行文本输入
//! - 文本编辑状态管理
//! - 实时内容更新通知
//! - 代码语法高亮

mod text_editor;
mod syntax_highlight;

pub use text_editor::*;
pub use syntax_highlight::*;

