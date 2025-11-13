# ReadRS - Markdown Editor

ReadRS 是一个现代化的Markdown编辑器，支持实时预览和多种格式导出。

## 项目状态

当前项目已经成功编译并运行。由于原项目使用的GPUI GUI框架API与实际依赖版本不兼容，我们创建了一个命令行版本来演示核心功能。

## 功能特性

- ✅ 创建新的Markdown文档
- ✅ 打开和编辑现有文档
- ✅ Markdown语法解析和HTML预览
- ✅ 多格式导出（HTML、纯文本等）
- ✅ 基础文档管理功能

## 使用方法

运行程序：

```bash
cargo run
```

按照程序提示进行操作：

1. 选择功能（创建、打开、编辑、预览、导出等）
2. 输入文档内容或文件路径
3. 查看渲染结果或导出文档

## 技术栈

- Rust 2021
- pulldown-cmark - Markdown解析
- serde - 数据序列化
- tokio - 异步运行时

## 未来计划

- [ ] 重构以使用兼容的GUI框架（如egui或GPUI 0.3+）
- [ ] 添加语法高亮
- [ ] 实现实时预览
- [ ] 增加更多导出格式（PDF、DOCX等）
- [ ] 添加主题和样式定制

## 编译要求

- Rust 1.70+
- Cargo

## 贡献

欢迎提交Issue和Pull Request来改进这个项目！