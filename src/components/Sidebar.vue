<template>
  <div class="sidebar">
    <div class="sidebar-nav">
      <button
        v-for="item in menuItems"
        :key="item.id"
        :class="['nav-item', { active: activePanel === item.id }]"
        @click="togglePanel(item.id)"
      >
        <span class="nav-icon">{{ item.icon }}</span>
        <span class="nav-label">{{ item.label }}</span>
      </button>
    </div>
    <div v-if="activePanel" class="sidebar-panel">
      <div class="panel-header">
        <span class="panel-title">{{ currentLabel }}</span>
        <button class="panel-close" @click="activePanel = null">&#x2715;</button>
      </div>
      <div class="panel-content">
        <p class="panel-placeholder">{{ currentLabel }}功能开发中...</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

const menuItems = [
  { id: 'projects', icon: '◫', label: '项目' },
  { id: 'history', icon: '◷', label: '历史' },
  { id: 'skills', icon: '◆', label: '技能' },
  { id: 'plugins', icon: '⊞', label: '插件' },
  { id: 'settings', icon: '⚙', label: '设置' },
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

.sidebar-nav {
  display: flex;
  flex-direction: column;
  width: 64px;
  background: var(--bg-sidebar);
  border-right: 1px solid var(--border);
  padding: 8px 0;
  gap: 2px;
}

.nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 3px;
  height: 52px;
  margin: 0 6px;
  background: transparent;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.15s;
  position: relative;
}

.nav-icon {
  font-size: 18px;
  color: var(--text-secondary);
  transition: color 0.15s;
  line-height: 1;
}

.nav-label {
  font-family: var(--font-ui);
  font-size: 10px;
  color: var(--text-muted);
  transition: color 0.15s;
}

.nav-item:hover {
  background: var(--bg-sidebar-hover);
}

.nav-item:hover .nav-icon {
  color: var(--text-primary);
}

.nav-item:hover .nav-label {
  color: var(--text-secondary);
}

.nav-item.active {
  background: var(--accent-dim);
}

.nav-item.active::before {
  content: '';
  position: absolute;
  left: -6px;
  top: 10px;
  bottom: 10px;
  width: 3px;
  background: var(--accent);
  border-radius: 0 2px 2px 0;
}

.nav-item.active .nav-icon {
  color: var(--accent);
}

.nav-item.active .nav-label {
  color: var(--accent-light);
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
  padding: 12px 14px;
  border-bottom: 1px solid var(--border);
}

.panel-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
}

.panel-close {
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  font-size: 12px;
  padding: 4px 6px;
  border-radius: 4px;
  transition: all 0.15s;
}

.panel-close:hover {
  color: var(--color-error);
  background: rgba(248, 113, 113, 0.1);
}

.panel-content {
  flex: 1;
  padding: 16px 14px;
  overflow-y: auto;
}

.panel-placeholder {
  color: var(--text-muted);
  font-size: 12px;
  text-align: center;
  padding-top: 40px;
}
</style>
