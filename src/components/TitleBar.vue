<template>
  <div class="title-bar" data-tauri-drag-region>
    <div class="title-bar-logo" data-tauri-drag-region>
      <span class="logo-text" data-tauri-drag-region>&block;&block;</span>
      <span class="title-text" data-tauri-drag-region>DCLIENT</span>
    </div>
    <div class="title-bar-controls">
      <button class="ctrl-btn minimize" @click="minimize">&#x2500;</button>
      <button class="ctrl-btn maximize" @click="toggleMaximize">&#x25A1;</button>
      <button class="ctrl-btn close" @click="close">&#x2715;</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'

const appWindow = getCurrentWindow()

function minimize() { appWindow.minimize() }
function toggleMaximize() { appWindow.toggleMaximize() }
function close() { appWindow.close() }
</script>

<style scoped>
.title-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 36px;
  background: var(--bg-tab);
  border-bottom: 1px solid var(--border);
  padding: 0 12px;
  -webkit-app-region: drag;
}

.title-bar-logo {
  display: flex;
  align-items: center;
  gap: 8px;
}

.logo-text {
  color: var(--neon-green);
  font-size: 14px;
  text-shadow: var(--glow-green);
}

.title-text {
  color: var(--text-primary);
  font-size: 13px;
  font-weight: 600;
  letter-spacing: 2px;
}

.title-bar-controls {
  display: flex;
  gap: 2px;
  -webkit-app-region: no-drag;
}

.ctrl-btn {
  width: 36px;
  height: 36px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  font-size: 12px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-family: 'Fira Code', monospace;
}

.ctrl-btn:hover {
  background: var(--bg-tab-hover);
  color: var(--text-primary);
}

.ctrl-btn.close:hover {
  background: var(--neon-red);
  color: white;
}
</style>
