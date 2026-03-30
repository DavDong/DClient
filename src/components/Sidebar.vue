<template>
  <div class="sidebar">
    <div class="sidebar-icons">
      <button
        v-for="item in menuItems"
        :key="item.id"
        :class="['sidebar-icon', { active: activePanel === item.id }]"
        :title="item.label"
        @click="togglePanel(item.id)"
      >
        <span class="icon-text">{{ item.icon }}</span>
      </button>
    </div>
    <div v-if="activePanel" class="sidebar-panel">
      <div class="panel-header">
        <span class="panel-title">{{ currentLabel }}</span>
        <button class="panel-close" @click="activePanel = null">&#x2715;</button>
      </div>
      <div class="panel-content">
        <p class="panel-placeholder">{{ currentLabel }}（开发中）</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

const menuItems = [
  { id: 'projects', icon: 'DIR', label: '项目目录' },
  { id: 'history', icon: 'LOG', label: '会话历史' },
  { id: 'skills', icon: 'SKL', label: '技能' },
  { id: 'plugins', icon: 'PLG', label: '插件' },
  { id: 'settings', icon: 'SET', label: '设置' },
]

const activePanel = ref<string | null>(null)

const currentLabel = computed(() =>
  menuItems.find(i => i.id === activePanel.value)?.label || ''
)

function togglePanel(id: string) {
  activePanel.value = activePanel.value === id ? null : id
}
</script>

<style scoped>
.sidebar {
  display: flex;
  height: 100%;
  flex-shrink: 0;
}

.sidebar-icons {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 48px;
  background: var(--bg-tab);
  border-right: 1px solid var(--border);
  padding-top: 8px;
  gap: 4px;
}

.sidebar-icon {
  position: relative;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.icon-text {
  font-family: 'Fira Code', monospace;
  font-size: 9px;
  font-weight: 600;
  color: var(--text-secondary);
  letter-spacing: 0.5px;
  transition: color 0.2s;
}

.sidebar-icon:hover {
  background: var(--bg-tab-hover);
}

.sidebar-icon:hover .icon-text {
  color: var(--text-primary);
}

.sidebar-icon.active {
  background: var(--bg-tab-active);
}

.sidebar-icon.active::before {
  content: '';
  position: absolute;
  left: 0;
  top: 6px;
  bottom: 6px;
  width: 2px;
  background: var(--neon-green);
  box-shadow: var(--glow-green);
  border-radius: 0 1px 1px 0;
}

.sidebar-icon.active .icon-text {
  color: var(--neon-green);
  text-shadow: var(--glow-green);
}

.sidebar-panel {
  width: 220px;
  background: var(--bg-panel);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 12px;
  border-bottom: 1px solid var(--border);
}

.panel-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-primary);
  letter-spacing: 1px;
}

.panel-close {
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 12px;
  font-family: 'Fira Code', monospace;
  padding: 2px 4px;
  border-radius: 2px;
  transition: all 0.2s;
}

.panel-close:hover {
  color: var(--neon-red);
  text-shadow: var(--glow-red);
}

.panel-content {
  flex: 1;
  padding: 16px 12px;
  overflow-y: auto;
}

.panel-placeholder {
  color: var(--text-secondary);
  font-size: 12px;
  text-align: center;
  padding-top: 40px;
}
</style>
