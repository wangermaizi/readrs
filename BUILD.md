# ReadRS - Rust Markdown Editor

## 项目结构

```
ReadRS/
├── Cargo.toml          # 项目配置和依赖
├── Cargo.lock          # 依赖锁定文件
├── README.md           # 项目说明文档
├── src/
│   ├── main.rs         # 主程序入口
│   ├── lib.rs          # 模块声明
│   ├── editor.rs       # Markdown编辑器组件
│   ├── preview.rs      # 实时预览组件
│   ├── renderer.rs     # Markdown渲染器
│   ├── file_manager.rs # 文件管理功能
│   ├── export.rs       # 导出功能
│   ├── theme.rs        # 主题和界面定制
│   └── features.rs     # 高级辅助功能
```

## 功能实现概述

ReadRS 是一款使用 Rust 和 GPUI 框架开发的现代化 Markdown 编辑器，具有以下核心功能：

1. **实时预览**：所见即所得的 Markdown 编辑体验
2. **文件管理**：新建、打开、保存、另存为功能
3. **多格式导出**：支持导出为 HTML、PDF、Word 等格式
4. **主题定制**：内置多套主题，支持个性化定制
5. **大纲视图**：快速导航文档结构
6. **拼写检查**：英文拼写错误实时检测
7. **快捷键支持**：丰富的快捷键提升编辑效率
8. **搜索功能**：文档内搜索和替换

## 技术栈

- Rust 语言
- GPUI 图形用户界面框架
- pulldown-cmark Markdown 解析库
- tokio 异步运行时
- regex 正则表达式库

## 项目特点

- 跨平台支持（Windows、macOS、Linux）
- 高性能的实时渲染
- 简洁直观的用户界面
- 可扩展的架构设计
- 完全开源的 MIT 许可证