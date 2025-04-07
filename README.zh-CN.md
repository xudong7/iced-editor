# Iced 编辑器

一个使用 Iced 构建的简洁文本编辑器，Iced 是 Rust 语言的跨平台 GUI 库。

[English](README.md) | [中文](README.zh-CN.md)

> 此项目受到 [iced-rs/iced](https://github.com/iced-rs/iced) 仓库和 [这个 YouTube 视频](https://www.youtube.com/watch?v=gcBJ7cPSALo) 的启发

## 功能特性

- **语法高亮**：支持 Rust、Markdown 和其他常见编程语言
- **文本编辑**：基本的文本编辑功能（新建、打开、保存）
- **主题切换**：支持浅色和深色模式
- **状态栏**：显示文件路径和光标位置
- **键盘快捷键**：支持常见快捷键如 Ctrl+S 保存

## 截图

![编辑器截图](screenshots/editor.png)

## 系统要求

- Rust (1.60 或更高版本)
- Cargo (随 Rust 安装)

## 依赖库

- **iced**: GUI 框架
- **tokio**: 异步运行时
- **rfd**: 文件对话框

## 安装方法

1. 确保已安装 Rust 和 Cargo

```bash
rustc --version
cargo --version
```

2. 克隆仓库

```bash
git clone https://github.com/xudong7/iced-editor.git
cd iced-editor
```

3. 构建并运行

```bash
cargo build 
cargo run 
```

## 使用方法

- **新建文件**：点击工具栏中的新建按钮或从菜单选择
- **打开文件**：点击打开按钮，选择要编辑的文件
- **保存文件**：点击保存按钮或使用 Ctrl+S 快捷键
- **切换主题**：使用右上角的主题选择下拉菜单

## 项目结构

```
iced-editor/
├── src/
│   ├── main.rs      # 主程序入口
│   └── ...
├── fonts/
│   └── editor-icons.ttf  # 编辑器图标字体
├── Cargo.toml       # 项目依赖
└── README.md        # 项目说明
```

## 未来计划

- [ ] 添加更多编辑功能（查找/替换、自动补全）
- [ ] 支持中文路径和文件内容的特殊处理
- [ ] 支持更多编程语言的语法高亮
- [ ] 标签页支持同时编辑多个文件
- [ ] 文件树视图
- [ ] 设置配置界面

## 贡献

欢迎提交 Pull Request 和 Issue！任何贡献都会被认真考虑。

## 许可证

MIT 许可证
