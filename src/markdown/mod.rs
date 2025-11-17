//! Markdown 解析模块
//! 
//! 使用 pulldown-cmark 解析 Markdown 文本，支持：
//! - CommonMark 标准语法
//! - 标题、列表、引用、代码块等基础语法
//! - 后续将扩展支持表格、公式、流程图等

mod parser;

pub use parser::*;

