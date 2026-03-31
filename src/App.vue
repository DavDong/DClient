<template>
  <div id="dclient">
    <TitleBar />
    <div class="main-area">
      <Sidebar @open-project="openProject" @add-project="addProject" @resume-session="resumeSession" />
      <div class="content-area">
        <TabBar @new="createTab" @close="closeTab" @clear-all="clearAllTabs" />
        <div :class="['terminal-area', { grid: store.gridMode }]" :style="gridStyle">
          <div
            v-for="tab in store.tabs"
            :key="tab.id"
            :class="['terminal-cell', { active: tab.id === store.activeTabId, hidden: !store.gridMode && tab.id !== store.activeTabId }]"
            @click="store.setActive(tab.id)"
          >
            <div v-if="store.gridMode" class="cell-label">{{ tab.name }}</div>
            <TerminalView
              :pty-id="tab.id"
              :is-visible="store.gridMode || tab.id === store.activeTabId"
              @exit="onTabExit(tab.id)"
              @info="onTabInfo"
            />
          </div>
          <div v-if="store.tabs.length === 0" class="empty-state">
            <div class="empty-logo">&#x2593;&#x2593;&#x2593;</div>
            <p>DCLIENT</p>
            <button class="empty-btn" @click="() => createTab()">+ NEW TERMINAL</button>
          </div>
        </div>
        <ChatPanel :pty-id="store.activeTabId" />
      </div>
    </div>
    <StatusBar
      :connected="currentTabConnected"
      :shell="shell"
      :cols="termInfo.cols"
      :rows="termInfo.rows"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, reactive, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import TitleBar from '@/components/TitleBar.vue'
import TabBar from '@/components/TabBar.vue'
import TerminalView from '@/components/TerminalView.vue'
import StatusBar from '@/components/StatusBar.vue'
import Sidebar from '@/components/Sidebar.vue'
import ChatPanel from '@/components/ChatPanel.vue'
import { useTerminalStore } from '@/stores/terminal'

const store = useTerminalStore()
const shell = navigator.platform.includes('Win') ? 'PowerShell' : 'zsh'
const termInfo = reactive({ cols: 80, rows: 24 })

// 宫格布局：根据标签数量计算列数
const gridStyle = computed(() => {
  if (!store.gridMode) return {}
  const count = store.tabs.length
  const cols = Math.ceil(Math.sqrt(count))
  return {
    'grid-template-columns': `repeat(${cols}, 1fr)`,
  }
})

const currentTabConnected = computed(() => {
  const tab = store.tabs.find(t => t.id === store.activeTabId)
  return tab?.isConnected ?? false
})

async function createTab(cwd?: string) {
  if (store.tabs.length >= 12) return
  try {
    const id = await invoke<string>('spawn_pty', { cols: 80, rows: 24 })
    const name = cwd ? cwd.split('/').pop() || cwd.split('\\').pop() || 'Terminal' : undefined
    store.addTab(id, name, cwd)
    // 如果指定了目录，自动 cd
    if (cwd) {
      await invoke('write_pty', { id, data: `cd "${cwd}"\n` })
    }
  } catch (e) {
    console.error('Failed to create PTY:', e)
  }
}

// 打开项目目录：已有关联终端就切换，没有就新建
function openProject(path: string) {
  const existingTab = store.findTabByCwd(path)
  if (existingTab) {
    store.setActive(existingTab.id)
  } else {
    createTab(path)
  }
}

// 恢复 Claude 会话
async function resumeSession(sessionId: string, project: string) {
  try {
    const id = await invoke<string>('spawn_pty', { cols: 80, rows: 24 })
    const name = project ? project.split('/').pop() || 'Claude' : 'Claude'
    store.addTab(id, `Claude: ${name}`, project)
    // cd 到项目目录并恢复会话
    if (project) {
      await invoke('write_pty', { id, data: `cd "${project}" && claude --resume ${sessionId}\n` })
    } else {
      await invoke('write_pty', { id, data: `claude --resume ${sessionId}\n` })
    }
  } catch (e) {
    console.error('Failed to resume session:', e)
  }
}

// 添加项目目录
async function addProject() {
  const selected = await open({ directory: true, multiple: false, title: '选择项目目录' })
  if (selected) {
    store.addProject(selected as string)
  }
}

async function clearAllTabs() {
  const ids = store.tabs.map(t => t.id)
  for (const id of ids) {
    try { await invoke('kill_pty', { id }) } catch { /* ignore */ }
  }
  store.tabs.splice(0)
  store.activeTabId = null
  store.gridMode = false
}

async function closeTab(id: string) {
  try {
    await invoke('kill_pty', { id })
  } catch { /* ignore */ }
  store.removeTab(id)
}

function onTabExit(id: string) {
  store.setDisconnected(id)
}

function onTabInfo(info: { cols: number; rows: number }) {
  termInfo.cols = info.cols
  termInfo.rows = info.rows
}

// 标签变化时自动保存会话
watch(() => [store.tabs.length, store.activeTabId], () => {
  store.saveSession()
}, { deep: true })

// 启动时加载持久化数据并恢复上次会话
onMounted(async () => {
  await store.init()
  const session = await store.getLastSession()
  if (session && session.tabs.length > 0) {
    for (let i = 0; i < session.tabs.length; i++) {
      const { name, cwd } = session.tabs[i]
      await createTab(cwd || undefined)
      // 恢复自定义名称
      if (name && store.tabs.length > 0) {
        const lastTab = store.tabs[store.tabs.length - 1]
        store.renameTab(lastTab.id, name)
      }
    }
    // 恢复上次激活的标签
    if (session.activeIndex >= 0 && session.activeIndex < store.tabs.length) {
      store.setActive(store.tabs[session.activeIndex].id)
    }
  } else {
    createTab()
  }
})
</script>

<style>
#dclient {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--bg-primary);
}

.main-area {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.content-area {
  display: flex;
  flex-direction: column;
  flex: 1;
  overflow: hidden;
}

.terminal-area {
  flex: 1;
  position: relative;
  overflow: hidden;
}

.terminal-area.grid {
  display: grid;
  gap: 1px;
  background: var(--border);
}

.terminal-cell {
  position: relative;
  overflow: hidden;
}

.terminal-cell.hidden {
  display: none;
}

.terminal-area.grid .terminal-cell {
  display: flex;
  flex-direction: column;
  background: var(--bg-primary);
  cursor: pointer;
}

.terminal-area.grid .terminal-cell.active {
  outline: 2px solid var(--accent);
  outline-offset: -1px;
}

.cell-label {
  font-size: 10px;
  color: var(--text-muted);
  padding: 2px 8px;
  background: var(--bg-tab);
  border-bottom: 1px solid var(--border);
}

.terminal-area.grid .terminal-cell.active .cell-label {
  color: var(--accent);
  background: var(--accent-dim);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: 16px;
}

.empty-logo {
  font-size: 48px;
  color: var(--accent);
}

.empty-state p {
  font-size: 24px;
  color: var(--text-secondary);
  letter-spacing: 8px;
  font-weight: 600;
}

.empty-btn {
  padding: 10px 24px;
  border: 1px solid var(--accent);
  background: transparent;
  color: var(--accent);
  font-family: var(--font-ui);
  font-size: 14px;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s;
}

.empty-btn:hover {
  background: var(--accent-dim);
  color: var(--accent-light);
  border-color: var(--accent-light);
}</style>
