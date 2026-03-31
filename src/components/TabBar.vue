<template>
  <div class="tab-bar">
    <button
      v-if="store.tabs.length > 1"
      :class="['grid-btn', { active: store.gridMode }]"
      @click="store.gridMode = !store.gridMode"
      :title="store.gridMode ? '退出分屏' : '分屏显示'"
    >⊞</button>
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
      <button class="tab-close" @click.stop="requestClose(tab)" title="关闭">✕</button>
    </div>
    <button class="tab-add" @click="$emit('new')">
      <span>+ NEW</span>
    </button>
    <button v-if="store.tabs.length > 0" class="tab-clear" @click="showClearConfirm = true" title="清空所有标签">✕ ALL</button>

    <!-- 关闭标签确认弹窗 -->
    <Teleport to="body">
      <div v-if="showClearConfirm" class="confirm-overlay" @click.self="showClearConfirm = false">
        <div class="confirm-dialog">
          <p class="confirm-text">确定关闭全部 {{ store.tabs.length }} 个终端？</p>
          <div class="confirm-actions">
            <button class="confirm-btn cancel" @click="showClearConfirm = false">取消</button>
            <button class="confirm-btn ok" @click="confirmClearAll">确定清空</button>
          </div>
        </div>
      </div>
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
const showClearConfirm = ref(false)

const emit = defineEmits<{
  close: [id: string]
  'clear-all': []
  new: []
}>()

function requestClose(tab: Tab) {
  if (tab.isConnected && tab.hasInput) {
    closingTab.value = tab
  } else {
    emit('close', tab.id)
  }
}

function confirmClearAll() {
  showClearConfirm.value = false
  emit('clear-all')
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
  flex-wrap: wrap;
  min-height: 38px;
  background: var(--bg-tab);
  border-bottom: 1px solid var(--border);
  padding: 4px 8px;
  gap: 2px;
}

.tab {
  display: flex;
  align-items: center;
  gap: 6px;
  height: 30px;
  padding: 0 12px;
  border-radius: 4px;
  cursor: pointer;
  color: var(--text-primary);
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
  color: var(--accent);
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
  font-size: 14px;
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  opacity: 0;
  transition: all 0.2s;
}

.tab:hover .tab-close {
  opacity: 1;
}

.tab-close:hover {
  color: var(--color-error);
  background: rgba(248, 113, 113, 0.1);
}

.tab-add {
  display: flex;
  align-items: center;
  height: 30px;
  padding: 0 10px;
  border-radius: 4px;
  border: none;
  background: transparent;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: 11px;
  cursor: pointer;
  transition: all 0.2s;
}

.tab-add:hover {
  color: var(--text-secondary);
  background: var(--bg-tab-hover);
}

.tab-clear {
  display: flex;
  align-items: center;
  height: 30px;
  padding: 0 10px;
  border-radius: 4px;
  border: none;
  background: transparent;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: 11px;
  cursor: pointer;
  transition: all 0.2s;
  flex-shrink: 0;
}

.tab-clear:hover {
  color: var(--color-error);
  background: rgba(248, 113, 113, 0.1);
}

.grid-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 30px;
  height: 30px;
  border-radius: 4px;
  border: none;
  background: transparent;
  color: var(--text-muted);
  font-size: 16px;
  cursor: pointer;
  transition: all 0.2s;
  flex-shrink: 0;
}

.grid-btn:hover {
  color: var(--text-secondary);
  background: var(--bg-tab-hover);
}

.grid-btn.active {
  color: var(--accent);
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
