# DClient

轻量级跨平台 AI 终端管理器，赛博朋克风格。

## 技术栈

- **框架**: Tauri 2 (Rust 后端 + WebView 前端)
- **前端**: Vue 3 + Vite + TypeScript + Pinia
- **终端**: xterm.js 5.x + @xterm/addon-fit
- **PTY**: portable-pty 0.9 (Rust)
- **UI**: 赛博朋克风格（黑底 + 荧光绿/紫/蓝，Fira Code 字体）
- **平台**: macOS + Windows 10 1809+

## 项目结构

```
DClient/
├── docs/                       # 设计文档
├── src/                        # Vue 前端
│   ├── components/
│   │   ├── TitleBar.vue        # 自定义标题栏（无边框窗口拖拽）
│   │   ├── TabBar.vue          # 标签栏（多标签管理）
│   │   ├── TerminalView.vue    # xterm.js 终端组件
│   │   └── StatusBar.vue       # 底部状态栏
│   ├── stores/terminal.ts      # Pinia 状态管理
│   ├── styles/cyberpunk.css    # 赛博朋克主题
│   ├── App.vue                 # 主布局
│   └── main.ts                 # 入口
├── src-tauri/                  # Rust 后端
│   ├── src/
│   │   ├── main.rs             # 入口
│   │   ├── lib.rs              # Tauri Builder + 命令注册
│   │   ├── pty_manager.rs      # PTY 管理器（spawn/write/resize/kill）
│   │   └── commands.rs         # Tauri 命令定义
│   ├── capabilities/default.json  # 权限配置
│   └── Cargo.toml
├── package.json
└── vite.config.ts
```

## 开发命令

```bash
# 前置要求：Node.js >= 18, Rust >= 1.70

# 安装前端依赖
npm install

# 开发模式（前端热更新 + Rust 编译）
npx tauri dev

# 生产构建
npx tauri build
```

## 架构

```
Vue 3 (xterm.js) ←→ Tauri IPC ←→ Rust (PtyManager) ←→ System Shell
```

- 前端通过 `invoke()` 调用 Rust 命令：spawn_pty / write_pty / resize_pty / kill_pty
- Rust 通过 `emit()` 事件推送 PTY 输出：pty-output-{id} / pty-exit-{id}
- 每个标签独立 PTY 实例，互不干扰

## 配色方案

- 背景: #0a0a0f, 面板: #12121a
- 荧光绿: #00ff88（主色调）
- 荧光紫: #bf5af2（次色调）
- 荧光蓝: #00d4ff（信息）
- 荧光红: #ff3b5c（警告/关闭）
