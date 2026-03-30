<template>
  <div class="tab-bar">
    <div
      v-for="tab in store.tabs"
      :key="tab.id"
      :class="['tab', { active: tab.id === store.activeTabId }]"
      @click="store.setActive(tab.id)"
      @dblclick="startRename(tab)"
    >
      <span class="tab-indicator" :class="{ connected: tab.isConnected }">&#x25C9;</span>
      <span v-if="editingId !== tab.id" class="tab-name">{{ tab.name }}</span>
      <input
        v-else
        ref="renameInput"
        v-model="editName"
        class="tab-rename-input"
        @blur="finishRename(tab.id)"
        @keyup.enter="finishRename(tab.id)"
        @keyup.escape="cancelRename"
      />
      <button class="tab-close" @click.stop="requestClose(tab)">&#x2715;</button>
    </div>
    <button class="tab-add" @click="$emit('new')">
      <span>+ NEW</span>
    </button>

    <!-- 关闭标签确认弹窗 -->
    <Teleport to="body">
      <div v-if="closingTab" class="confirm-overlay" @click.self="closingTab = null">
        <div class="confirm-dialog">
          <p class="confirm-text">确定关闭终端「{{ closingTab.name }}」？</p>
          <div class="confirm-actions">
            <button class="confirm-btn cancel" @click="closingTab = null">取消</button>
            <button class="confirm-btn ok" @click="confirmCloseTab">确定关闭</button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick } from 'vue'
import { useTerminalStore, type Tab } from '@/stores/terminal'

const store = useTerminalStore()
const editingId = ref<string | null>(null)
const editName = ref('')
const renameInput = ref<HTMLInputElement[]>()
const closingTab = ref<Tab | null>(null)

const emit = defineEmits<{
  close: [id: string]
  new: []
}>()

function requestClose(tab: Tab) {
  if (tab.isConnected) {
    closingTab.value = tab
  } else {
    emit('close', tab.id)
  }
}

function confirmCloseTab() {
  if (closingTab.value) {
    emit('close', closingTab.value.id)
    closingTab.value = null
  }
}

function startRename(tab: Tab) {
  editingId.value = tab.id
  editName.value = tab.name
  nextTick(() => {
    renameInput.value?.[0]?.focus()
  })
}

function finishRename(id: string) {
  if (editName.value.trim()) {
    store.renameTab(id, editName.value.trim())
  }
  editingId.value = null
}

function cancelRename() {
  editingId.value = null
}
</script>

<style scoped>
.tab-bar {
  display: flex;
  align-items: center;
  height: 38px;
  background: var(--bg-tab);
  border-bottom: 1px solid var(--border);
  padding: 0 8px;
  gap: 2px;
  overflow-x: auto;
}

.tab {
  display: flex;
  align-items: center;
  gap: 6px;
  height: 30px;
  padding: 0 12px;
  border-radius: 4px;
  cursor: pointer;
  color: var(--text-secondary);
  font-size: 12px;
  white-space: nowrap;
  border: 1px solid transparent;
  transition: all 0.2s;
}

.tab:hover {
  background: var(--bg-tab-hover);
  color: var(--text-primary);
}

.tab.active {
  background: var(--bg-tab-active);
  color: var(--text-primary);
  border-left: 3px solid var(--accent);
  border-color: transparent;
  border-left-color: var(--accent);
}

.tab-indicator {
  font-size: 8px;
  color: var(--text-secondary);
}

.tab-indicator.connected {
  color: var(--color-success);
}

.tab-name {
  max-width: 120px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.tab-rename-input {
  background: transparent;
  border: 1px solid var(--accent);
  color: var(--text-primary);
  font-family: var(--font-mono);
  font-size: 12px;
  padding: 0 4px;
  width: 100px;
  outline: none;
  border-radius: 2px;
}

.tab-close {
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 10px;
  padding: 2px;
  border-radius: 2px;
  font-family: 'Fira Code', monospace;
  opacity: 0;
  transition: all 0.2s;
}

.tab:hover .tab-close {
  opacity: 1;
}

.tab-close:hover {
  color: var(--color-error);
}

.tab-add {
  display: flex;
  align-items: center;
  height: 30px;
  padding: 0 12px;
  border-radius: 4px;
  border: 1px dashed var(--accent);
  background: transparent;
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: 11px;
  cursor: pointer;
  transition: all 0.2s;
}

.tab-add:hover {
  border-color: var(--accent-light);
  color: var(--accent-light);
  background: var(--accent-dim);
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
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 24px;
  min-width: 320px;
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
  border-radius: 6px;
  font-family: var(--font-ui);
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
  border: 1px solid var(--color-error);
  color: var(--color-error);
}

.confirm-btn.ok:hover {
  background: var(--color-error);
  color: white;
}
</style>
