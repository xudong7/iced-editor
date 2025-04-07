# Iced Editor

A minimalist text editor built with Iced, a cross-platform GUI library for the Rust programming language.

[English](README.md) | [中文](README.zh-CN.md)

> This project is inspired by the [iced-rs/iced](https://github.com/iced-rs/iced) repository and [this YouTube video](https://www.youtube.com/watch?v=gcBJ7cPSALo)

## Features

- **Syntax Highlighting**: Supports Rust, Markdown, and other common programming languages
- **Text Editing**: Basic text editing capabilities (create, open, save)
- **Theme Switching**: Supports both light and dark modes
- **Status Bar**: Displays file path and cursor position
- **Keyboard Shortcuts**: Supports common shortcuts like Ctrl+S for saving

## Screenshot

![Editor Screenshot](/assets/screenshot.png)

## System Requirements

- Rust (1.60 or newer)
- Cargo (comes with Rust)

## Dependencies

- **iced**: GUI framework
- **tokio**: Async runtime
- **rfd**: File dialogs

## Installation

1. Make sure Rust and Cargo are installed

```bash
rustc --version
cargo --version
```

2. Clone the repository

```bash
git clone https://github.com/xudong7/iced-editor.git
cd iced-editor
```

3. Build and run

```bash
cargo build
cargo run
```

## Usage

- **New File**: Click the new button in the toolbar or select from menu
- **Open File**: Click the open button and select a file to edit
- **Save File**: Click the save button or use Ctrl+S shortcut
- **Switch Theme**: Use the theme selection dropdown in the top right

## Project Structure

```
iced-editor/
├── src/
│   ├── main.rs      # Main program entry
│   └── ...
├── fonts/
│   └── editor-icons.ttf  # Editor icon font
├── Cargo.toml       # Project dependencies
└── README.md        # Project documentation
```

## Future Plans

- [ ] Add more editing features (find/replace, autocomplete)
- [ ] Support special handling for Chinese paths and file content
- [ ] Support syntax highlighting for more programming languages
- [ ] Tab support for editing multiple files simultaneously
- [ ] File tree view
- [ ] Settings configuration interface

## Contributing

Pull requests and issues are welcome! Any contribution will be seriously considered.

## License

MIT License
