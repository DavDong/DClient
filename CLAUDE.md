# DClient

轻量级跨平台 AI 终端管理客户端，Reqable 橙黑风格。

## 技术栈

- **框架**: Tauri 2 (Rust 后端 + WebView 前端)
- **前端**: Vue 3 + Vite + TypeScript + Pinia
- **终端**: xterm.js 5.x + @xterm/addon-fit
- **PTY**: portable-pty 0.9 (Rust)
- **UI**: Reqable 风格（深黑 + 橙色强调，Inter + Fira Code 字体）
- **平台**: macOS + Windows 10 1809+

## 项目结构

```
DClient/
├── docs/                          # 设计文档
├── src/                           # Vue 前端
│   ├── components/
│   │   ├── TitleBar.vue           # 自定义标题栏（无边框窗口拖拽）
│   │   ├── Sidebar.vue            # 左侧操作栏（项目/历史/技能/插件/设置）
│   │   ├── TabBar.vue             # 标签栏（多标签管理）
│   │   ├── TerminalView.vue       # xterm.js 终端组件
│   │   ├── ChatPanel.vue          # 底部对话面板（智能输入）
│   │   └── StatusBar.vue          # 底部状态栏
│   ├── stores/terminal.ts         # Pinia 状态（标签、项目目录）
│   ├── styles/cyberpunk.css       # 主题样式
│   ├── App.vue                    # 主布局
│   └── main.ts                    # 入口
├── src-tauri/                     # Rust 后端
│   ├── src/
│   │   ├── main.rs                # 入口
│   │   ├── lib.rs                 # Tauri Builder + 命令注册
│   │   ├── pty_manager.rs         # PTY 管理器（spawn/write/resize/kill）
│   │   └── commands.rs            # Tauri 命令定义
│   ├── capabilities/default.json  # 权限配置
│   └── Cargo.toml
├── package.json
└── vite.config.ts
```

## 布局

```
╔═══════╦══════════════════════════════════╗
║ 侧边栏 ║  标题栏                          ║
║       ╠══════════════════════════════════╣
║ 项目  ║  标签栏 [Tab1] [Tab2] [+]        ║
║ 历史  ╠══════════════════════════════════╣
║ 技能  ║  CLI 终端输出（xterm.js）         ║
║ 插件  ╠══════════════════════════════════╣
║ 设置  ║  对话面板（聊天输入框）            ║
╚═══════╩══════════════════════════════════╝
```

## 开发命令

```bash
# 前置要求：Node.js >= 18, Rust >= 1.70
npm install           # 安装前端依赖
npx tauri dev         # 开发模式
npx tauri build       # 生产构建
```

## 架构

```
Vue 3 (xterm.js) ←→ Tauri IPC ←→ Rust (PtyManager) ←→ System Shell
```

- `invoke()` 调用 Rust 命令：spawn_pty / write_pty / resize_pty / kill_pty
- `emit()` 推送 PTY 输出：pty-output-{id} / pty-exit-{id}
- 每个标签独立 PTY 实例，关联项目目录

## 配色方案（Reqable 风格）

- 背景: #1a1a1a, 面板: #222222
- 橙色主色: #ff6b2b（强调、活跃状态）
- 成功: #4ade80, 错误: #f87171, 信息: #60a5fa
- 文字: #e5e5e5 / #888888 / #555555

## 核心功能

- 多标签终端，每个标签独立 PTY
- 左侧项目目录面板，点击已关联目录自动切换终端
- 底部对话面板，输入指令发送到终端
- 关闭窗口/标签二次确认
- 项目目录持久化（localStorage）
