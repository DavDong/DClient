# DClient 设计与实施计划

## Context

用户需要一个轻量级桌面程序来管理多个 CLI 智能体（如 Claude Code），支持 Mac + Windows。要求流畅、不占用太多性能。选择 Tauri 2 + Vue 3 + xterm.js 方案。UI 风格：赛博朋克（黑底 + 荧光绿/紫/蓝，Fira Code 字体）。

## 技术栈

- **框架**: Tauri 2（Rust 后端 + 系统 WebView）
- **前端**: Vue 3 + Vite + TypeScript
- **终端模拟**: xterm.js 5.x + xterm-addon-fit + xterm-addon-webgl
- **PTY 管理**: tauri-plugin-pty（基于 portable-pty）
- **字体**: Fira Code（等宽，支持连字）
- **平台**: macOS + Windows 10 1809+

## 架构

```
┌─────────────────────────────────────────────┐
│ Vue 3 Frontend                              │
│  ├─ TabBar — 标签栏（赛博朋克风格）          │
│  ├─ TerminalView — xterm.js 实例            │
│  ├─ StatusBar — 底部状态栏                   │
│  └─ SettingsModal — 预设命令配置             │
└──────────────┬──────────────────────────────┘
               │ Tauri IPC
┌──────────────┴──────────────────────────────┐
│ Rust Backend                                │
│  ├─ PtyManager — 管理多个 PTY 实例          │
│  ├─ Commands — spawn/write/resize/kill      │
│  └─ Events — pty_output 数据流              │
└─────────────────────────────────────────────┘
```

## UI 设计 — 赛博朋克风格

### 配色方案

```
背景:        #0a0a0f (近纯黑，微紫)
面板背景:    #12121a (深紫灰)
标签栏背景:  #0d0d15
活跃标签:    #00ff88 (荧光绿) 边框发光
非活跃标签:  #2a2a3a
终端背景:    #0a0a0f
终端前景:    #d0d0e0 (亮灰白)
荧光绿:      #00ff88 (主色调，状态/活跃)
荧光紫:      #bf5af2 (次色调，标签/按钮)
荧光蓝:      #00d4ff (第三色，链接/信息)
荧光红:      #ff3b5c (警告/关闭)
边框:        #1a1a2e (微光线条)
光晕效果:    box-shadow: 0 0 10px rgba(0,255,136,0.3)
```

### 布局

```
╔══════════════════════════════════════════════════════╗
║ ▓▓ DCLIENT                              ─  □  ✕  ║  ← 标题栏（无边框窗口）
╠══════════════════════════════════════════════════════╣
║ ▸ dd-admin  │ ▸ claude-sync  │ ▸ [+ NEW]          ║  ← 标签栏（荧光色边框）
╠══════════════════════════════════════════════════════╣
║                                                      ║
║  $ claude                                            ║
║  ╭─────────────────────────────────────────╮         ║  ← 终端区域
║  │ Hello! How can I help you today?        │         ║
║  │ > Working on dd-admin backend...        │         ║
║  ╰─────────────────────────────────────────╯         ║
║                                                      ║
║  $ _                                                 ║
║                                                      ║
╠══════════════════════════════════════════════════════╣
║ ◉ CONNECTED │ zsh │ 120×36 │ RAM: 42MB │ CPU: 1%  ║  ← 状态栏
╚══════════════════════════════════════════════════════╝
```

### 标签栏细节

- 活跃标签：荧光绿底边发光 + 白色文字
- 非活跃标签：暗紫灰 + 灰色文字
- Hover：微光效果（box-shadow 扩散）
- 新建按钮 `[+]`：荧光紫，hover 时发光
- 关闭按钮 `✕`：hover 时荧光红
- 标签可双击重命名

### xterm.js 终端主题

```js
{
  background: '#0a0a0f',
  foreground: '#d0d0e0',
  cursor: '#00ff88',
  cursorAccent: '#0a0a0f',
  selectionBackground: 'rgba(0,212,255,0.3)',
  black: '#1a1a2e',
  red: '#ff3b5c',
  green: '#00ff88',
  yellow: '#ffd60a',
  blue: '#00d4ff',
  magenta: '#bf5af2',
  cyan: '#00d4ff',
  white: '#d0d0e0',
  brightBlack: '#3a3a5a',
  brightRed: '#ff6b8a',
  brightGreen: '#33ffaa',
  brightYellow: '#ffe066',
  brightBlue: '#33ddff',
  brightMagenta: '#d07aff',
  brightCyan: '#33ddff',
  brightWhite: '#ffffff',
}
```

## MVP 功能

1. **多标签终端** — 每个标签独立 PTY，互不干扰
2. **标签命名** — 双击重命名，右键菜单关闭/重命名
3. **快速启动** — 新建标签可选预设（如 `cd ~/dongdong && claude`）
4. **窗口自适应** — xterm.js addon-fit 自动适配
5. **无边框窗口** — 自定义标题栏，拖拽移动
6. **状态栏** — 显示连接状态、shell 类型、终端尺寸

## 关键文件结构

```
DClient/
├── src/                          # Vue 前端
│   ├── App.vue                   # 主布局
│   ├── components/
│   │   ├── TitleBar.vue          # 自定义标题栏
│   │   ├── TabBar.vue            # 标签栏
│   │   ├── TerminalView.vue      # xterm.js 终端组件
│   │   ├── StatusBar.vue         # 底部状态栏
│   │   └── NewTabMenu.vue        # 新建标签菜单
│   ├── stores/
│   │   └── terminal.ts           # Pinia 状态（标签列表、活跃标签）
│   ├── styles/
│   │   └── cyberpunk.css         # 赛博朋克主题样式
│   └── main.ts
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── main.rs
│   │   ├── pty_manager.rs        # PTY 管理器
│   │   └── commands.rs           # Tauri 命令
│   └── Cargo.toml
├── package.json
└── vite.config.ts
```

## 实施步骤

1. 创建设计文档到 `docs/specs/2026-03-30-dclient-design.md`
2. 用 `npm create tauri-app@latest` 初始化 Tauri 2 + Vue 3 + TS 项目
3. 安装 xterm.js、xterm-addon-fit，配置赛博朋克主题
4. Rust 端集成 portable-pty，实现 PtyManager
5. 实现 Tauri 命令：spawn_pty / write_pty / resize_pty / kill_pty
6. 前端实现 TerminalView 组件（xterm.js + IPC 桥接）
7. 实现 TabBar 多标签管理
8. 实现 TitleBar 无边框窗口 + StatusBar
9. 添加赛博朋克 CSS 样式和发光效果
10. Mac + Windows 构建测试

## 验证

- `npm run tauri dev` 开发模式运行
- 能打开多个终端标签，每个独立运行 shell
- 赛博朋克 UI 风格正确（荧光色、发光效果）
- 在终端中运行 `claude` 能正常交互
- Mac 和 Windows 都能正常构建
- 内存占用 < 100MB（3 个标签）
