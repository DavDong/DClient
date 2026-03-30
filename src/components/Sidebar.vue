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
        <button class="panel-close" @click="activePanel = null">✕</button>
      </div>
      <div class="panel-content">
        <!-- 项目目录面板 -->
        <template v-if="activePanel === 'projects'">
          <div class="project-list">
            <div
              v-for="path in store.projects"
              :key="path"
              :class="['project-item', { linked: !!store.findTabByCwd(path) }]"
              @click="$emit('open-project', path)"
            >
              <div class="project-info">
                <span class="project-name">{{ getProjectName(path) }}</span>
                <span class="project-path">{{ path }}</span>
              </div>
              <div class="project-actions">
                <span v-if="store.findTabByCwd(path)" class="project-linked" title="已关联终端">●</span>
                <button class="project-remove" @click.stop="store.removeProject(path)" title="移除">✕</button>
              </div>
            </div>
          </div>
          <div v-if="store.projects.length === 0" class="panel-empty">
            暂无项目，点击下方添加
          </div>
          <button class="add-project-btn" @click="$emit('add-project')">+ 添加项目目录</button>
        </template>

        <!-- 其他面板占位 -->
        <template v-else>
          <p class="panel-placeholder">{{ currentLabel }}功能开发中...</p>
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useTerminalStore } from '@/stores/terminal'

const store = useTerminalStore()

defineEmits<{
  'open-project': [path: string]
  'add-project': []
}>()

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

function getProjectName(path: string) {
  return path.split('/').pop() || path.split('\\').pop() || path
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
  width: 240px;
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
  padding: 8px 0;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
}

.panel-placeholder {
  color: var(--text-muted);
  font-size: 12px;
  text-align: center;
  padding-top: 40px;
}

/* 项目列表 */
.project-list {
  flex: 1;
}

.project-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 14px;
  cursor: pointer;
  transition: background 0.15s;
}

.project-item:hover {
  background: var(--bg-sidebar-hover);
}

.project-item.linked {
  background: var(--accent-dim);
}

.project-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
  flex: 1;
}

.project-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.project-path {
  font-size: 10px;
  color: var(--text-muted);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-family: var(--font-mono);
}

.project-actions {
  display: flex;
  align-items: center;
  gap: 4px;
  margin-left: 8px;
}

.project-linked {
  color: var(--color-success);
  font-size: 8px;
}

.project-remove {
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  font-size: 10px;
  padding: 2px 4px;
  border-radius: 3px;
  opacity: 0;
  transition: all 0.15s;
}

.project-item:hover .project-remove {
  opacity: 1;
}

.project-remove:hover {
  color: var(--color-error);
  background: rgba(248, 113, 113, 0.1);
}

.panel-empty {
  color: var(--text-muted);
  font-size: 12px;
  text-align: center;
  padding: 40px 14px 16px;
}

.add-project-btn {
  margin: 8px 14px;
  padding: 8px;
  border: 1px dashed var(--border);
  background: transparent;
  color: var(--accent);
  font-family: var(--font-ui);
  font-size: 12px;
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.15s;
}

.add-project-btn:hover {
  border-color: var(--accent);
  background: var(--accent-dim);
}
</style>
