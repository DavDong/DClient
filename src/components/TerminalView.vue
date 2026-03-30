<template>
  <div
    ref="terminalEl"
    class="terminal-container"
    :style="{ display: isVisible ? 'block' : 'none' }"
  ></div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { Terminal } from 'xterm'
import { FitAddon } from '@xterm/addon-fit'
import 'xterm/css/xterm.css'

const props = defineProps<{
  ptyId: string
  isVisible: boolean
}>()

const emit = defineEmits<{
  exit: []
  info: [info: { cols: number; rows: number }]
}>()

const terminalEl = ref<HTMLElement>()
let terminal: Terminal | null = null
let fitAddon: FitAddon | null = null
let unlistenOutput: UnlistenFn | null = null
let unlistenExit: UnlistenFn | null = null
let resizeObserver: ResizeObserver | null = null

// 赛博朋克终端主题
const cyberpunkTheme = {
  background: '#0a0a0f',
  foreground: '#d0d0e0',
  cursor: '#00ff88',
  cursorAccent: '#0a0a0f',
  selectionBackground: 'rgba(0,212,255,0.3)',
  selectionForeground: '#ffffff',
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

onMounted(async () => {
  if (!terminalEl.value) return

  terminal = new Terminal({
    theme: cyberpunkTheme,
    fontFamily: "'Fira Code', monospace",
    fontSize: 14,
    cursorBlink: true,
    cursorStyle: 'block',
    allowProposedApi: true,
  })

  fitAddon = new FitAddon()
  terminal.loadAddon(fitAddon)
  terminal.open(terminalEl.value)
  fitAddon.fit()

  // 发送终端尺寸信息
  emitInfo()

  // 键盘输入 → PTY
  terminal.onData(async (data: string) => {
    await invoke('write_pty', { id: props.ptyId, data })
  })

  // PTY 输出 → 终端
  unlistenOutput = await listen<string>(`pty-output-${props.ptyId}`, (event) => {
    terminal?.write(event.payload)
  })

  // PTY 退出
  unlistenExit = await listen(`pty-exit-${props.ptyId}`, () => {
    emit('exit')
  })

  // 窗口大小变化
  resizeObserver = new ResizeObserver(() => {
    if (props.isVisible) {
      fitAddon?.fit()
      const dims = fitAddon?.proposeDimensions()
      if (dims) {
        invoke('resize_pty', { id: props.ptyId, cols: dims.cols, rows: dims.rows })
        emitInfo()
      }
    }
  })
  resizeObserver.observe(terminalEl.value)
})

watch(() => props.isVisible, (visible) => {
  if (visible) {
    setTimeout(() => {
      fitAddon?.fit()
      terminal?.focus()
    }, 50)
  }
})

function emitInfo() {
  if (terminal) {
    emit('info', { cols: terminal.cols, rows: terminal.rows })
  }
}

onBeforeUnmount(() => {
  unlistenOutput?.()
  unlistenExit?.()
  resizeObserver?.disconnect()
  terminal?.dispose()
})
</script>

<style scoped>
.terminal-container {
  width: 100%;
  height: 100%;
  padding: 4px;
}

.terminal-container :deep(.xterm) {
  height: 100%;
}

.terminal-container :deep(.xterm-viewport) {
  overflow-y: auto !important;
}
</style>
