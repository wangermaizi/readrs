# ReadRS 项目目录结构说明

本文档描述了 ReadRS 项目的目录结构和各模块的职责。

## 项目根目录

```
ReadRS/
├── Cargo.toml              # Rust 项目配置文件，包含所有依赖声明
├── Cargo.lock              # 依赖版本锁定文件（自动生成）
├── README.md               # 项目说明文档
├── PROJECT_STRUCTURE.md    # 本文件：项目结构说明
├── BUILD.md                # 编译与运行说明（阶段 1 创建）
├── src/                    # 源代码目录
│   ├── main.rs            # 应用程序入口点（阶段 1 完成）
│   ├── editor/             # 编辑器模块（阶段 2 开始）
│   │   ├── mod.rs
│   │   └── text_editor.rs
│   ├── markdown/          # Markdown 解析与渲染模块（阶段 2-3）
│   │   ├── mod.rs
│   │   ├── parser.rs      # Markdown 解析器
│   │   ├── renderer.rs    # Markdown 渲染器
│   │   └── syntax.rs      # 语法高亮支持
│   ├── file_manager/      # 文件管理模块（阶段 4）
│   │   ├── mod.rs
│   │   ├── file_ops.rs    # 文件操作（新建、打开、保存）
│   │   └── file_tree.rs   # 文件夹树视图
│   ├── export/             # 导出功能模块（阶段 6）
│   │   ├── mod.rs
│   │   ├── pdf.rs         # PDF 导出
│   │   ├── docx.rs         # Word 导出
│   │   └── html.rs         # HTML 导出
│   ├── theme/              # 主题管理模块（阶段 5）
│   │   ├── mod.rs
│   │   ├── theme.rs        # 主题定义
│   │   └── css_loader.rs   # 自定义 CSS 主题加载
│   └── ui/                 # UI 组件模块（阶段 2-5）
│       ├── mod.rs
│       ├── toolbar.rs      # 工具栏组件
│       ├── sidebar.rs      # 侧边栏组件
│       └── preview.rs      # 预览区组件
├── assets/                 # 静态资源目录（阶段 5）
│   ├── themes/            # 主题文件
│   │   ├── light.css
│   │   └── dark.css
│   └── icons/             # 图标资源
└── tests/                  # 测试文件目录（阶段 7）
    ├── markdown_test.rs
    └── file_manager_test.rs
```

## 模块职责说明

### src/main.rs
- **职责**：应用程序入口点，初始化 GPUI 应用和窗口
- **状态**：阶段 1 完成基础窗口实现

### src/editor/（阶段 2 开始）
- **职责**：文本编辑器核心功能
- **功能**：
  - 文本输入与编辑
  - 光标控制
  - 复制/粘贴/剪切
  - 撤销/重做
  - 实时预览触发

### src/markdown/（阶段 2-3）
- **职责**：Markdown 解析与渲染
- **功能**：
  - 使用 pulldown-cmark 解析 Markdown
  - 渲染为 GPUI 元素
  - 支持 LaTeX 公式（katex-rs）
  - 支持 Mermaid 流程图（mermaid-rs）
  - 代码块语法高亮（syntect）

### src/file_manager/（阶段 4）
- **职责**：文件系统操作与管理
- **功能**：
  - 新建/打开/保存文件
  - 文件夹树视图
  - 文件搜索
  - 自动保存

### src/export/（阶段 6）
- **职责**：多格式导出功能
- **功能**：
  - PDF 导出（pdf-writer）
  - Word 导出（docx-rs）
  - HTML 导出

### src/theme/（阶段 5）
- **职责**：主题管理与定制
- **功能**：
  - 内置主题（明亮/暗黑）
  - 自定义 CSS 主题加载
  - 主题切换

### src/ui/（阶段 2-5）
- **职责**：可复用的 UI 组件
- **功能**：
  - 工具栏组件
  - 侧边栏组件
  - 预览区组件
  - 对话框组件

## 开发规范

### 代码风格
- 遵循 Rust 官方规范
- 变量/函数使用 `snake_case`
- 类型/结构体使用 `PascalCase`
- 模块使用 `snake_case` 目录名

### 错误处理
- 使用 `thiserror` 定义自定义错误类型
- 使用 `anyhow::Result` 处理应用级错误
- 所有文件操作必须处理错误情况

### 性能要求
- 支持 10 万字文档无卡顿
- 实时渲染延迟 < 50ms
- 利用 GPUI GPU 加速优势

### 跨平台支持
- Windows 10+
- macOS 10.14+
- Linux (Ubuntu 18.04+)

## 当前阶段状态

**阶段 1：项目初始化与基础架构搭建** ✅
- [x] Cargo.toml 配置完成
- [x] 项目目录结构设计完成
- [x] 基础窗口实现完成
- [x] GPUI 环境验证完成

**阶段 2：核心功能 - Markdown 实时预览基础版** ⏳
- [ ] 编辑区 + 预览区布局
- [ ] pulldown-cmark 集成
- [ ] 实时渲染实现
- [ ] 基础文本编辑功能

## 依赖说明

### 核心框架
- `gpui`: GPU 加速 UI 框架（从 GitHub 仓库引入）
- `gpui-component`: GPUI 组件库（从 GitHub 仓库引入）

### Markdown 相关
- `pulldown-cmark`: Markdown 解析库
- `katex-rs`: LaTeX 公式渲染（阶段 3）
- `mermaid-rs`: Mermaid 流程图渲染（阶段 3）
- `syntect`: 代码语法高亮（阶段 3）

### 文件操作
- `walkdir`: 目录遍历
- `tokio`: 异步运行时

### 导出功能（阶段 6）
- `pdf-writer`: PDF 导出
- `docx-rs`: Word 导出

### 工具库
- `thiserror`: 错误类型定义
- `anyhow`: 错误处理
- `serde`: 序列化支持
- `futures`: 异步工具
- `uuid`: UUID 生成

