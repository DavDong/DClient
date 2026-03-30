# DClient

轻量级跨平台 AI 终端管理器。

## 技术栈

- **框架**: Tauri 2 (Rust 后端 + WebView 前端)
- **前端**: Vue 3 + Vite + TypeScript
- **终端**: xterm.js + portable-pty
- **平台**: Mac + Windows

## 项目结构

```
DClient/
├── docs/              # 设计文档和计划
├── src-tauri/         # Rust 后端（PTY 管理、系统交互）
└── src/               # Vue 前端（终端 UI、标签管理）
```

## 开发命令

```bash
# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 构建
npm run tauri build
```
