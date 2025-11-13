# Windows 环境配置指南

## 问题：`link.exe` not found

这是因为 Rust 默认使用 MSVC 工具链，需要 Visual Studio 的链接器。

## 解决方案

### 方案 1：安装 Visual Studio Build Tools（推荐 ⭐）

**对于 Rust 开发，这是最佳选择**：体积小（约 6GB），只包含编译工具，不包含 IDE。

1. **下载 Visual Studio Build Tools**
   - 访问：https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
   - 下载 "Build Tools for Visual Studio 2022"（**不是完整的 Visual Studio**）

2. **安装时选择工作负载**
   - 运行安装程序
   - 勾选 **"使用 C++ 的桌面开发"** 工作负载
   - 点击安装（约 6GB）

3. **重新打开终端**
   - 关闭当前终端窗口
   - 重新打开 PowerShell 或 CMD
   - 运行 `cargo run`

### 方案 1.5：如果必须安装完整 Visual Studio

如果您需要完整的 Visual Studio IDE（用于 C++、C# 等其他开发），可以选择：

- **Visual Studio Community**（推荐）⭐
  - ✅ **完全免费**，适合个人开发者
  - ✅ 功能完整，包含所有必要的编译工具
  - ✅ 支持 C++、C#、.NET、Python 等开发
  - ✅ 对于个人独立开发者完全够用
  - 下载：https://visualstudio.microsoft.com/downloads/#visual-studio-community-2022

- **Visual Studio Professional / Enterprise**
  - ❌ 需要付费订阅（约 $45/月起）
  - ❌ 主要面向企业用户
  - ❌ 对于个人开发者来说功能过剩
  - ⚠️ **不推荐**：除非您的公司已购买许可证

**安装步骤**（以 Community 为例）：
1. 下载 Visual Studio Community 2022
2. 运行安装程序
3. 勾选 **"使用 C++ 的桌面开发"** 工作负载
4. 点击安装（约 10-15GB，包含 IDE）
5. 重新打开终端，运行 `cargo run`

**总结**：
- 🎯 **只做 Rust 开发** → 选择 **Build Tools**（方案 1）
- 🎯 **需要完整 IDE 且是个人开发者** → 选择 **Community 版本**（方案 1.5）
- 🎯 **企业用户** → 根据公司许可证选择 Professional/Enterprise

### 方案 2：切换到 GNU 工具链（无需 Visual Studio）

如果您不想安装 Visual Studio，可以切换到 GNU 工具链：

1. **安装 MinGW-w64**
   - 下载：https://www.mingw-w64.org/downloads/
   - 或使用 MSYS2：https://www.msys2.org/
   - 安装后确保 `gcc` 在 PATH 中

2. **安装 GNU 工具链**
   ```bash
   rustup toolchain install stable-x86_64-pc-windows-gnu
   rustup default stable-x86_64-pc-windows-gnu
   ```

3. **配置 Cargo**
   创建或编辑 `%USERPROFILE%\.cargo\config.toml`：
   ```toml
   [target.x86_64-pc-windows-gnu]
   linker = "gcc"
   ```

4. **重新编译**
   ```bash
   cargo clean
   cargo run
   ```

### 方案 3：使用预编译的二进制文件

如果只是想要运行程序，可以考虑：
- 使用 GitHub Actions 或其他 CI 构建
- 使用 WSL（Windows Subsystem for Linux）

## 常见错误解决

### 错误：`dlltool.exe` not found

**错误信息**：
```
error: error calling dlltool 'dlltool.exe': program not found
error: could not compile `parking_lot_core` (lib) due to 1 previous error
```

**原因**：
- 您可能切换到了 GNU 工具链（`x86_64-pc-windows-gnu`），但没有完整安装 MinGW-w64
- 或者 MinGW-w64 的 `bin` 目录不在系统 PATH 中
- `dlltool.exe` 是 MinGW-w64 工具链的一部分，用于创建 DLL 导入库

**解决方案**：

#### 方案 A：切换回 MSVC 工具链（推荐 ⭐）

如果您已经安装了 Visual Studio Build Tools 或 Visual Studio Community，最简单的方法是切换回 MSVC 工具链：

```bash
# 检查当前工具链
rustup show

# 切换到 MSVC 工具链（默认）
rustup default stable-x86_64-pc-windows-msvc

# 如果 MSVC 工具链未安装，先安装
rustup toolchain install stable-x86_64-pc-windows-msvc
rustup default stable-x86_64-pc-windows-msvc

# 清理之前的编译缓存
cargo clean

# 重新编译
cargo run
```

#### 方案 B：完整安装 MinGW-w64（如果必须使用 GNU 工具链）

1. **安装 MSYS2**（推荐方式）
   - 下载：https://www.msys2.org/
   - 安装后，打开 MSYS2 终端
   - 运行以下命令安装 MinGW-w64：
     ```bash
     pacman -Syu
     pacman -S mingw-w64-x86_64-toolchain
     ```

2. **配置环境变量**
   - 将 MinGW-w64 的 `bin` 目录添加到系统 PATH
   - 默认路径：`C:\msys64\mingw64\bin`
   - 或者：`C:\msys64\usr\bin`（如果使用 MSYS2）

3. **验证安装**
   ```bash
   # 在 PowerShell 中验证
   dlltool --version
   gcc --version
   ```

4. **重新打开终端并编译**
   ```bash
   cargo clean
   cargo run
   ```

#### 方案 C：使用预编译的 MinGW-w64

1. **下载预编译版本**
   - 访问：https://www.mingw-w64.org/downloads/
   - 下载 Win64 版本

2. **解压并添加到 PATH**
   - 解压到 `C:\mingw64`（或任意目录）
   - 将 `C:\mingw64\bin` 添加到系统 PATH

3. **验证并重新编译**
   ```bash
   dlltool --version
   cargo clean
   cargo run
   ```

**推荐**：如果您已经安装了 Visual Studio Build Tools，使用**方案 A**最简单快捷。

## 验证安装

安装完成后，运行以下命令验证：

```bash
rustc --version
cargo --version
```

如果仍然报错，请：
1. 确保已重新打开终端
2. 检查环境变量 PATH 中是否包含 Rust 工具链路径
3. 尝试运行 `rustup self update` 更新 rustup
4. 检查是否意外切换到了 GNU 工具链：`rustup show`

