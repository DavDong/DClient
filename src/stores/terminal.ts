import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface Tab {
  id: string        // PTY ID
  name: string      // 显示名称
  cwd: string       // 工作目录
  isConnected: boolean
}

export const useTerminalStore = defineStore('terminal', () => {
  const tabs = ref<Tab[]>([])
  const activeTabId = ref<string | null>(null)

  // 已保存的项目目录列表（持久化到 localStorage）
  const projects = ref<string[]>(JSON.parse(localStorage.getItem('dclient-projects') || '[]'))

  function saveProjects() {
    localStorage.setItem('dclient-projects', JSON.stringify(projects.value))
  }

  function addProject(path: string) {
    if (!projects.value.includes(path)) {
      projects.value.push(path)
      saveProjects()
    }
  }

  function removeProject(path: string) {
    const idx = projects.value.indexOf(path)
    if (idx !== -1) {
      projects.value.splice(idx, 1)
      saveProjects()
    }
  }

  // 查找与目录关联的终端标签
  function findTabByCwd(cwd: string): Tab | undefined {
    return tabs.value.find(t => t.cwd === cwd && t.isConnected)
  }

  function addTab(id: string, name?: string, cwd?: string) {
    const tabName = name || `Terminal ${tabs.value.length + 1}`
    tabs.value.push({ id, name: tabName, cwd: cwd || '', isConnected: true })
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

  return { tabs, activeTabId, projects, addTab, removeTab, setActive, renameTab, setDisconnected, addProject, removeProject, findTabByCwd }
})
