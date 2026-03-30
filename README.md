# DClient

轻量级跨平台 AI 终端管理器，用于管理多个 CLI 智能体（如 Claude Code）。

## 特性

- 多标签终端，每个标签独立运行 CLI agent
- 轻量高效（Tauri + Rust），内存占用低
- 跨平台支持 Mac + Windows
- 标签命名和分组管理

## 技术栈

- **Tauri 2** — Rust 后端，系统原生 WebView
- **Vue 3 + TypeScript** — 前端界面
- **xterm.js** — 终端模拟
- **portable-pty** — 伪终端管理

## 开发

```bash
# 前置要求：Node.js >= 18, Rust >= 1.70

# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 构建发布
npm run tauri build
```

## 许可

MIT
