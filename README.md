# ReadRS - Markdown 编辑器

ReadRS 是一个现代化的、类 Typora 的「所见即所得 Markdown 编辑器」，使用 Rust + GPUI 构建，充分利用 GPU 加速优势，提供流畅的编辑体验。

## 项目状态

**当前阶段：阶段 1 - 项目初始化与基础架构搭建** ✅

- ✅ GPUI 项目脚手架创建完成
- ✅ Cargo.toml 依赖配置完成
- ✅ 基础窗口实现完成
- ✅ GPUI 环境验证完成

## 功能特性（规划中）

### 已实现
- ✅ 基础窗口（标题、大小、最小化/最大化/关闭）

### 计划实现
- [ ] **实时预览** - 编辑时即时查看渲染效果（阶段 2）
- [ ] **Markdown 全语法支持** - 标题、列表、表格、代码块、公式等（阶段 2-3）
- [ ] **LaTeX 公式渲染** - 支持数学公式实时渲染（阶段 3）
- [ ] **Mermaid 流程图** - 支持流程图、时序图等（阶段 3）
- [ ] **文件管理** - 新建、打开、保存、文件夹树视图（阶段 4）
- [ ] **主题定制** - 明亮/暗黑主题，自定义 CSS（阶段 5）
- [ ] **导出功能** - PDF、Word、HTML 导出（阶段 6）
- [ ] **语法高亮** - 代码块语法高亮（阶段 3）
- [ ] **搜索功能** - 文档内搜索，关键词高亮（阶段 4）
- [ ] **快捷键支持** - 丰富的键盘快捷键（阶段 5）

## 技术栈

### 核心框架
- **GPUI** - GPU 加速 UI 框架（从 GitHub 仓库引入）
- **gpui-component** - GPUI 组件库（从 GitHub 仓库引入）

### Markdown 相关
- **pulldown-cmark** - Markdown 解析库（CommonMark 标准）
- **katex-rs** - LaTeX 数学公式渲染
- **mermaid-rs** - Mermaid 流程图渲染
- **syntect** - 代码语法高亮

### 文件操作
- **walkdir** - 目录遍历
- **tokio** - 异步运行时

### 导出功能
- **pdf-writer** - PDF 导出
- **docx-rs** - Word 导出

### 工具库
- **thiserror** - 错误类型定义
- **anyhow** - 错误处理
- **serde** - 序列化支持

## 快速开始

### 前置要求

- Rust 1.70+ 和 Cargo
- Git（用于获取 GPUI 依赖）
- 平台特定构建工具（见 [BUILD.md](BUILD.md)）

### 编译与运行

```bash
# 克隆项目
git clone <repository-url>
cd ReadRS

# 编译项目
cargo build

# 运行项目
cargo run
```

详细说明请参考 [BUILD.md](BUILD.md)。

### 验证阶段 1

运行程序后，应该看到：

1. ✅ 窗口标题："ReadRS - Markdown 编辑器"
2. ✅ 窗口大小：1200x800 像素
3. ✅ 窗口功能：最小化、最大化、关闭、调整大小正常
4. ✅ 窗口内容：显示欢迎信息和阶段说明

## 项目结构

```
ReadRS/
├── src/
│   └── main.rs              # 应用程序入口（阶段 1 完成）
├── Cargo.toml               # 项目配置
├── BUILD.md                  # 编译运行说明
├── PROJECT_STRUCTURE.md      # 项目结构说明
└── README.md                 # 本文件
```

详细结构说明请参考 [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md)。

## 开发计划

### 阶段 1：项目初始化与基础架构搭建 ✅
- [x] 创建 GPUI 项目脚手架
- [x] 配置 Cargo.toml 依赖
- [x] 实现基础窗口
- [x] 验证 GPUI 环境配置

### 阶段 2：核心功能 - Markdown 实时预览基础版 ⏳
- [ ] 编辑区 + 预览区布局
- [ ] pulldown-cmark 集成
- [ ] 实时渲染实现
- [ ] 基础文本编辑功能

### 阶段 3：增强 Markdown 语法支持
- [ ] 表格、代码块、超链接、图片
- [ ] LaTeX 公式渲染
- [ ] Mermaid 流程图渲染
- [ ] 目录自动生成

### 阶段 4：文件管理功能
- [ ] 文件操作（新建、打开、保存）
- [ ] 文件夹树视图
- [ ] 文档内搜索

### 阶段 5：界面定制与交互优化
- [ ] 主题切换
- [ ] 专注模式、打字机模式
- [ ] 快捷键支持

### 阶段 6：导出功能与性能优化
- [ ] PDF/Word/HTML 导出
- [ ] 大文档性能优化
- [ ] GPU 渲染优化

### 阶段 7：测试与打包发布
- [ ] 单元测试
- [ ] 跨平台打包
- [ ] 发布文档

## 性能目标

- ✅ 支持 10 万字文档无卡顿
- ✅ 实时渲染延迟 < 50ms
- ✅ 利用 GPU 加速优势

## 跨平台支持

- ✅ Windows 10+
- ✅ macOS 10.14+
- ✅ Linux (Ubuntu 18.04+)

## 贡献

欢迎提交 Issue 和 Pull Request 来改进这个项目！

## 许可证

MIT License
