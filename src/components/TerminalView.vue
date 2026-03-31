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
import { useTerminalStore } from '@/stores/terminal'
import 'xterm/css/xterm.css'

const store = useTerminalStore()

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

// Reqable 橙黑终端主题
const termTheme = {
  background: '#1a1a1a',
  foreground: '#e5e5e5',
  cursor: '#ff6b2b',
  cursorAccent: '#1a1a1a',
  selectionBackground: 'rgba(255,107,43,0.2)',
  selectionForeground: '#ffffff',
  black: '#1a1a1a',
  red: '#f87171',
  green: '#4ade80',
  yellow: '#fbbf24',
  blue: '#60a5fa',
  magenta: '#c084fc',
  cyan: '#22d3ee',
  white: '#e5e5e5',
  brightBlack: '#555555',
  brightRed: '#fca5a5',
  brightGreen: '#86efac',
  brightYellow: '#fde68a',
  brightBlue: '#93c5fd',
  brightMagenta: '#d8b4fe',
  brightCyan: '#67e8f9',
  brightWhite: '#ffffff',
}

onMounted(async () => {
  if (!terminalEl.value) return

  terminal = new Terminal({
    theme: termTheme,
    fontFamily: "'Fira Code', 'Consolas', monospace",
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
    // 回车键表示用户执行了命令
    if (data.includes('\r') || data.includes('\n')) {
      store.markInput(props.ptyId)
    }
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
