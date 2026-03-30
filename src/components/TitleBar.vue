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

    <!-- 关闭确认弹窗 -->
    <Teleport to="body">
      <div v-if="showConfirm" class="confirm-overlay" @click.self="cancelClose">
        <div class="confirm-dialog">
          <p class="confirm-text">当前有 {{ store.tabs.length }} 个终端会话运行中，确定关闭？</p>
          <div class="confirm-actions">
            <button class="confirm-btn cancel" @click="cancelClose">取消</button>
            <button class="confirm-btn ok" @click="confirmClose">确定关闭</button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useTerminalStore } from '@/stores/terminal'

const appWindow = getCurrentWindow()
const store = useTerminalStore()

function minimize() { appWindow.minimize() }
function toggleMaximize() { appWindow.toggleMaximize() }

const showConfirm = ref(false)

function close() {
  if (store.tabs.length > 0) {
    showConfirm.value = true
  } else {
    appWindow.destroy()
  }
}

function confirmClose() {
  showConfirm.value = false
  appWindow.destroy()
}

function cancelClose() {
  showConfirm.value = false
}
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

.confirm-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.confirm-dialog {
  background: var(--bg-panel);
  border: 1px solid var(--neon-purple);
  box-shadow: var(--glow-purple);
  border-radius: 8px;
  padding: 24px;
  min-width: 360px;
}

.confirm-text {
  color: var(--text-primary);
  font-size: 14px;
  margin-bottom: 20px;
  text-align: center;
}

.confirm-actions {
  display: flex;
  justify-content: center;
  gap: 12px;
}

.confirm-btn {
  padding: 8px 24px;
  border-radius: 4px;
  font-family: 'Fira Code', monospace;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.confirm-btn.cancel {
  background: transparent;
  border: 1px solid var(--border);
  color: var(--text-secondary);
}

.confirm-btn.cancel:hover {
  border-color: var(--text-secondary);
  color: var(--text-primary);
}

.confirm-btn.ok {
  background: transparent;
  border: 1px solid var(--neon-red);
  color: var(--neon-red);
}

.confirm-btn.ok:hover {
  background: var(--neon-red);
  color: white;
  box-shadow: var(--glow-red);
}
</style>
