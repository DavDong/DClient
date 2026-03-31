<template>
  <div class="title-bar" data-tauri-drag-region>
    <div class="title-bar-logo" data-tauri-drag-region>
      <span class="logo-icon" data-tauri-drag-region>◎</span>
      <span class="title-text" data-tauri-drag-region>DClient</span>
    </div>
    <div class="title-bar-controls">
      <button class="ctrl-btn" @click="minimize">─</button>
      <button class="ctrl-btn" @click="toggleMaximize">□</button>
      <button class="ctrl-btn close" @click="close">✕</button>
    </div>

    <Teleport to="body">
      <div v-if="showConfirm" class="confirm-overlay" @click.self="cancelClose">
        <div class="confirm-dialog">
          <div class="confirm-icon">⚠</div>
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
  height: 40px;
  background: var(--bg-tab);
  border-bottom: 1px solid var(--border);
  padding: 0 14px;
  -webkit-app-region: drag;
}

.title-bar-logo {
  display: flex;
  align-items: center;
  gap: 8px;
}

.logo-icon {
  color: var(--accent);
  font-size: 16px;
}

.title-text {
  color: var(--text-primary);
  font-family: var(--font-ui);
  font-size: 14px;
  font-weight: 600;
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
  font-size: 13px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  transition: all 0.15s;
}

.ctrl-btn:hover {
  background: var(--bg-tab-hover);
  color: var(--text-primary);
}

.ctrl-btn.close:hover {
  background: var(--color-error);
  color: white;
}

.confirm-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.confirm-dialog {
  background: var(--bg-panel);
  border: 1px solid var(--border-light);
  border-radius: 12px;
  padding: 28px;
  min-width: 360px;
  text-align: center;
}

.confirm-icon {
  font-size: 32px;
  color: var(--color-warning);
  margin-bottom: 12px;
}

.confirm-text {
  color: var(--text-primary);
  font-size: 14px;
  margin-bottom: 24px;
  line-height: 1.5;
}

.confirm-actions {
  display: flex;
  justify-content: center;
  gap: 12px;
}

.confirm-btn {
  padding: 8px 28px;
  border-radius: 6px;
  font-family: var(--font-ui);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.15s;
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
  background: var(--color-error);
  border: 1px solid var(--color-error);
  color: white;
}

.confirm-btn.ok:hover {
  opacity: 0.9;
}
</style>
