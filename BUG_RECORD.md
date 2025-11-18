# BUG 记录

## 问题描述

在删除 `target` 文件夹并重新执行 `cargo run` 时，项目出现多个编译错误。

## 错误详情

### 1. InputEvent 命名冲突
```
error[E0659]: `InputEvent` is ambiguous
  --> src\editor\text_editor.rs:58:20
   |
58 |             if let InputEvent::Change = event {
   |                    ^^^^^^^^^^ ambiguous name
   |
   = note: ambiguous because of multiple glob imports of a name in the same module
```

### 2. 缺失方法错误
```
error[E0599]: no method named `overflow_y_scroll` found for struct `gpui::Div` in the current scope
  --> src\preview\renderer.rs:58:14

error[E0599]: no method named `whitespace_pre_wrap` found for struct `gpui::Div` in the current scope
  --> src\preview\renderer.rs:66:22
```

### 3. 生命周期和闭包问题
```
error[E0521]: borrowed data escapes outside of closure
error[E0596]: cannot borrow `*cx` as mutable
```

### 4. 文档注释中换行符问题
```
error: bare CR not allowed in doc-comment
```

## 解决方案

### 1. 修复命名冲突
- 在 `src/editor/text_editor.rs` 中显式导入 `InputEvent`：
  ```rust
  use gpui_component::input::{InputEvent, InputState, Input};
  ```

### 2. 修复缺失方法
- 将 `whitespace_pre_wrap()` 替换为 `whitespace_nowrap()`
- 移除 `overflow_y_scroll()` 方法调用

### 3. 解决生命周期问题
- 在 `src/preview/renderer.rs` 中将 `self.html_content.as_ref()` 替换为克隆后再使用

### 4. 修复文档注释
- 重新写入文件，使用正确的换行符（LF 而不是 CRLF）

## 原因分析

这些问题主要是由于：
1. 依赖库版本更新后 API 发生变化
2. Windows 换行符（CRLF）与 Rust 编译器不兼容
3. 代码中存在一些生命周期管理问题

## 修复后状态

- ✅ 项目可以成功编译
- ✅ 项目可以正常运行
- ✅ 没有编译错误