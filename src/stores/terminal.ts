import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface Tab {
  id: string        // PTY ID
  name: string      // 显示名称
  isConnected: boolean
}

export const useTerminalStore = defineStore('terminal', () => {
  const tabs = ref<Tab[]>([])
  const activeTabId = ref<string | null>(null)

  function addTab(id: string, name?: string) {
    const tabName = name || `Terminal ${tabs.value.length + 1}`
    tabs.value.push({ id, name: tabName, isConnected: true })
    activeTabId.value = id
  }

  function removeTab(id: string) {
    const index = tabs.value.findIndex(t => t.id === id)
    if (index !== -1) {
      tabs.value.splice(index, 1)
      if (activeTabId.value === id) {
        activeTabId.value = tabs.value.length > 0 ? tabs.value[Math.max(0, index - 1)].id : null
      }
    }
  }

  function setActive(id: string) {
    activeTabId.value = id
  }

  function renameTab(id: string, name: string) {
    const tab = tabs.value.find(t => t.id === id)
    if (tab) tab.name = name
  }

  function setDisconnected(id: string) {
    const tab = tabs.value.find(t => t.id === id)
    if (tab) tab.isConnected = false
  }

  return { tabs, activeTabId, addTab, removeTab, setActive, renameTab, setDisconnected }
})
