# ReadRS 编译与运行说明

本文档说明如何编译和运行 ReadRS 项目，适用于 Windows、macOS 和 Linux 平台。

## 前置要求

### 1. Rust 开发环境

确保已安装 Rust 1.70+ 和 Cargo：

```bash
# 检查 Rust 版本
rustc --version
cargo --version
```

如果未安装，请访问 [rustup.rs](https://rustup.rs/) 安装 Rust。

### 2. Git

项目依赖从 GitHub 仓库引入，需要安装 Git：

```bash
# 检查 Git 版本
git --version
```

### 3. 平台特定依赖

#### Windows 10+

**必需：**
- Windows 10 或更高版本
- Visual Studio Build Tools 或 Visual Studio（包含 C++ 构建工具）
- Windows SDK

**安装步骤：**
1. 安装 [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
   - 选择 "C++ 生成工具" 工作负载
   - 确保包含 Windows SDK

2. 或者安装完整的 [Visual Studio](https://visualstudio.microsoft.com/)
   - 选择 "使用 C++ 的桌面开发" 工作负载

#### macOS 10.14+

**必需：**
- Xcode Command Line Tools

**安装步骤：**
```bash
xcode-select --install
```

#### Linux (Ubuntu 18.04+)

**必需：**
- 构建工具和系统库

**安装步骤：**
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install build-essential pkg-config libssl-dev libasound2-dev \
    libgtk-3-dev libwebkit2gtk-4.0-dev libayatana-appindicator3-dev \
    librsvg2-dev

# Fedora
sudo dnf install gcc pkg-config openssl-devel alsa-lib-devel \
    gtk3-devel webkit2gtk3-devel libappindicator-gtk3-devel \
    librsvg2-devel

# Arch Linux
sudo pacman -S base-devel pkg-config openssl alsa-lib gtk3 \
    webkit2gtk libappindicator-gtk3 librsvg
```

## 编译步骤

### 1. 克隆或进入项目目录

```bash
cd ReadRS
```

### 2. 构建项目

**开发模式（带调试信息）：**
```bash
cargo build
```

**发布模式（优化编译）：**
```bash
cargo build --release
```

**首次编译说明：**
- 首次编译会下载并编译所有依赖，包括 GPUI 和 gpui-component
- 这可能需要较长时间（10-30 分钟，取决于网络和机器性能）
- 后续编译会快很多（只编译变更的代码）

### 3. 运行项目

**开发模式运行：**
```bash
cargo run
```

**发布模式运行：**
```bash
cargo run --release
```

**直接运行编译后的可执行文件：**

Windows:
```powershell
.\target\debug\readrs.exe
# 或发布版本
.\target\release\readrs.exe
```

macOS/Linux:
```bash
./target/debug/readrs
# 或发布版本
./target/release/readrs
```

## 验证方法

### 阶段 1 验证清单

运行程序后，应该看到：

1. ✅ **窗口正常显示**
   - 窗口标题显示："ReadRS - Markdown 编辑器"
   - 窗口大小：1200x800 像素
   - 窗口位置：屏幕左上角偏移 (100, 100)

2. ✅ **窗口功能正常**
   - **最小化**：点击窗口标题栏最小化按钮，窗口应最小化到任务栏
   - **最大化**：点击窗口标题栏最大化按钮，窗口应全屏显示
   - **关闭**：点击窗口标题栏关闭按钮，窗口应关闭，程序退出
   - **调整大小**：拖动窗口边缘，窗口大小应可调整

3. ✅ **窗口内容显示**
   - 窗口中心显示："ReadRS - Markdown 编辑器"（大标题）
   - 下方显示："欢迎使用！GPUI 环境配置成功 ✓"（副标题）
   - 底部显示阶段说明文本

4. ✅ **GPU 渲染正常**
   - 窗口内容清晰显示
   - 文本渲染无模糊
   - 窗口拖动和调整大小流畅

### 常见问题排查

#### 问题 1：编译失败 - "找不到 GPUI"

**错误信息：**
```
error: failed to resolve: use of undeclared crate or module `gpui`
```

**解决方案：**
- 确保网络连接正常，可以访问 GitHub
- 检查 `Cargo.toml` 中的 GPUI 依赖配置是否正确
- 尝试清理并重新构建：
  ```bash
  cargo clean
  cargo build
  ```

#### 问题 2：编译失败 - "链接错误"

**Windows：**
- 确保已安装 Visual Studio Build Tools 或 Visual Studio
- 确保 Windows SDK 已安装

**macOS：**
- 运行 `xcode-select --install` 安装命令行工具

**Linux：**
- 确保已安装所有必需的开发库（见前置要求）

#### 问题 3：运行时窗口不显示

**可能原因：**
- GPU 驱动问题
- 窗口被其他窗口遮挡
- 窗口位置超出屏幕范围

**解决方案：**
- 检查 GPU 驱动是否最新
- 尝试 Alt+Tab 切换窗口
- 检查任务栏是否有程序图标

#### 问题 4：编译时间过长

**原因：**
- GPUI 和 gpui-component 需要从源码编译
- 首次编译需要编译所有依赖

**解决方案：**
- 这是正常现象，请耐心等待
- 后续编译会快很多（增量编译）
- 可以使用 `cargo build --release` 进行优化编译（虽然首次更慢，但后续运行更快）

## 性能优化建议

### 开发模式 vs 发布模式

- **开发模式** (`cargo run`)：
  - 编译快，但运行慢
  - 包含调试信息
  - 适合开发和调试

- **发布模式** (`cargo run --release`)：
  - 编译慢，但运行快
  - 代码优化，无调试信息
  - 适合测试性能和最终发布

### 加速编译

1. **使用 sccache（编译缓存）：**
   ```bash
   cargo install sccache
   # 在 ~/.cargo/config.toml 中添加：
   # [build]
   # rustc-wrapper = "/path/to/sccache"
   ```

2. **并行编译：**
   ```bash
   # 在 ~/.cargo/config.toml 中添加：
   # [build]
   # jobs = 8  # 根据 CPU 核心数调整
   ```

## 下一步

完成阶段 1 验证后，可以继续：

- **阶段 2**：实现 Markdown 实时预览基础版
- 查看 `PROJECT_STRUCTURE.md` 了解项目结构
- 查看 `README.md` 了解项目概述

## 获取帮助

如果遇到问题：

1. 检查本文档的"常见问题排查"部分
2. 查看 Rust 和 Cargo 的官方文档
3. 检查 GPUI 和 gpui-component 的 GitHub 仓库 Issues
4. 提交 Issue 到项目仓库
