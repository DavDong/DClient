<template>
  <div id="dclient">
    <TitleBar />
    <div class="main-area">
      <Sidebar />
      <div class="content-area">
        <TabBar @new="createTab" @close="closeTab" />
        <div class="terminal-area">
          <TerminalView
            v-for="tab in store.tabs"
            :key="tab.id"
            :pty-id="tab.id"
            :is-visible="tab.id === store.activeTabId"
            @exit="onTabExit(tab.id)"
            @info="onTabInfo"
          />
          <div v-if="store.tabs.length === 0" class="empty-state">
            <div class="empty-logo">&#x2593;&#x2593;&#x2593;</div>
            <p>DCLIENT</p>
            <button class="empty-btn" @click="createTab">+ NEW TERMINAL</button>
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
import { computed, reactive, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
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

const currentTabConnected = computed(() => {
  const tab = store.tabs.find(t => t.id === store.activeTabId)
  return tab?.isConnected ?? false
})

async function createTab() {
  try {
    const id = await invoke<string>('spawn_pty', { cols: 80, rows: 24 })
    store.addTab(id)
  } catch (e) {
    console.error('Failed to create PTY:', e)
  }
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

// 启动时自动创建第一个标签
onMounted(() => {
  createTab()
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
  color: var(--neon-green);
  text-shadow: var(--glow-green);
}

.empty-state p {
  font-size: 24px;
  color: var(--text-secondary);
  letter-spacing: 8px;
  font-weight: 600;
}

.empty-btn {
  padding: 10px 24px;
  border: 1px solid var(--neon-purple);
  background: transparent;
  color: var(--neon-purple);
  font-family: 'Fira Code', monospace;
  font-size: 14px;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s;
}

.empty-btn:hover {
  box-shadow: var(--glow-purple);
  background: rgba(191, 90, 242, 0.1);
}
</style>
