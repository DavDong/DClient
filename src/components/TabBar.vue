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
      <button class="tab-close" @click.stop="$emit('close', tab.id)">&#x2715;</button>
    </div>
    <button class="tab-add" @click="$emit('new')">
      <span>+ NEW</span>
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick } from 'vue'
import { useTerminalStore, type Tab } from '@/stores/terminal'

const store = useTerminalStore()
const editingId = ref<string | null>(null)
const editName = ref('')
const renameInput = ref<HTMLInputElement[]>()

defineEmits<{
  close: [id: string]
  new: []
}>()

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
  border-color: var(--neon-green);
  box-shadow: var(--glow-green);
}

.tab-indicator {
  font-size: 8px;
  color: var(--text-secondary);
}

.tab-indicator.connected {
  color: var(--neon-green);
  text-shadow: var(--glow-green);
}

.tab-name {
  max-width: 120px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.tab-rename-input {
  background: transparent;
  border: 1px solid var(--neon-blue);
  color: var(--text-primary);
  font-family: 'Fira Code', monospace;
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
  color: var(--neon-red);
  text-shadow: var(--glow-red);
}

.tab-add {
  display: flex;
  align-items: center;
  height: 30px;
  padding: 0 12px;
  border-radius: 4px;
  border: 1px dashed var(--border);
  background: transparent;
  color: var(--neon-purple);
  font-family: 'Fira Code', monospace;
  font-size: 11px;
  cursor: pointer;
  transition: all 0.2s;
}

.tab-add:hover {
  border-color: var(--neon-purple);
  box-shadow: var(--glow-purple);
  background: rgba(191, 90, 242, 0.05);
}
</style>
