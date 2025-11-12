# ReadRS - GPUI Markdown 编辑器

ReadRS 是一款使用 Rust 和 GPUI 框架开发的现代化 Markdown 编辑器，具有实时预览、跨平台支持和丰富的编辑功能。

## 功能特性

- **实时预览**：所见即所得的 Markdown 编辑体验
- **跨平台支持**：支持 Windows、macOS 和 Linux
- **多种主题**：内置多套主题，支持个性化定制
- **文件管理**：完整的新建、打开、保存、导出功能
- **大纲视图**：快速导航文档结构
- **拼写检查**：英文拼写错误实时检测
- **快捷键支持**：丰富的快捷键提升编辑效率
- **多格式导出**：支持导出为 HTML、PDF、Word 等格式

## 安装

目前项目仍在开发中，可通过以下方式构建：

```bash
git clone https://github.com/wangermaizi/readrs.git
cd readrs
cargo run
```

## 使用方法

1. 启动应用后，您可以新建一个 Markdown 文件，或打开现有文件
2. 在左侧编辑区输入 Markdown 语法内容
3. 右侧实时预览区会即时渲染 Markdown 内容
4. 使用顶部工具栏进行文件管理、主题切换等操作
5. 使用侧边栏的大纲视图快速定位文档结构

## 开发

本项目使用 GPUI 框架构建，主要模块包括：

- `editor.rs` - 编辑器核心组件
- `preview.rs` - 实时预览功能
- `file_manager.rs` - 文件管理功能
- `export.rs` - 多格式导出功能
- `theme.rs` - 主题和界面定制
- `features.rs` - 高级辅助功能

## 贡献

欢迎提交 Issue 和 Pull Request 来改进 ReadRS！

## 许可证

MIT