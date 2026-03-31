import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface Tab {
  id: string        // PTY ID
  name: string      // 显示名称
  cwd: string       // 工作目录
  isConnected: boolean
  hasInput: boolean  // 用户是否有输入过命令
}

// 持久化的会话信息（不含运行时 PTY ID）
interface SavedSession {
  tabs: { name: string; cwd: string }[]
  activeIndex: number
}

export interface ClaudeSession {
  session_id: string
  project: string
  first_message: string
  last_timestamp: number
  message_count: number
}

// 文件读写工具
async function readFile<T>(filename: string, fallback: T): Promise<T> {
  try {
    const raw = await invoke<string>('read_config', { filename })
    return raw && raw !== '{}' ? JSON.parse(raw) : fallback
  } catch {
    return fallback
  }
}

async function writeFile(filename: string, data: unknown): Promise<void> {
  try {
    await invoke('write_config', { filename, data: JSON.stringify(data, null, 2) })
  } catch (e) {
    console.error(`Failed to write ${filename}:`, e)
  }
}

export interface ClaudeSkill {
  id: string
  name: string
  description: string
  content: string
  source: string       // "global" 或项目路径
}

export interface McpServer {
  name: string
  command: string
  args: string[]
  source: string
}

export interface ClaudePlugin {
  id: string
  name: string
  marketplace: string
  version: string
  description: string
  installed_at: string
}

export const useTerminalStore = defineStore('terminal', () => {
  const tabs = ref<Tab[]>([])
  const activeTabId = ref<string | null>(null)
  const gridMode = ref(false)
  const projects = ref<string[]>([])
  const claudeSessions = ref<ClaudeSession[]>([])
  const skills = ref<ClaudeSkill[]>([])
  const plugins = ref<ClaudePlugin[]>([])
  const mcpServers = ref<McpServer[]>([])

  // 初始化：从文件加载持久化数据
  async function init() {
    projects.value = await readFile<string[]>('projects.json', [])
    await loadSkills()
    await loadPlugins()
    await loadMcpServers()
  }

  // 读取 Claude 技能（全局 + 当前项目）
  async function loadSkills(projectPath?: string) {
    try {
      skills.value = await invoke<ClaudeSkill[]>('get_claude_skills', { projectPath: projectPath || null })
    } catch (e) {
      console.error('Failed to load skills:', e)
    }
  }

  async function loadClaudeSessions() {
    try {
      claudeSessions.value = await invoke<ClaudeSession[]>('get_claude_history')
    } catch (e) {
      console.error('Failed to load Claude history:', e)
    }
  }

  function addProject(path: string) {
    if (!projects.value.includes(path)) {
      projects.value.push(path)
      writeFile('projects.json', projects.value)
    }
  }

  function removeProject(path: string) {
    const idx = projects.value.indexOf(path)
    if (idx !== -1) {
      projects.value.splice(idx, 1)
      writeFile('projects.json', projects.value)
    }
  }

  function findTabByCwd(cwd: string): Tab | undefined {
    return tabs.value.find(t => t.cwd === cwd && t.isConnected)
  }

  function addTab(id: string, name?: string, cwd?: string) {
    const tabName = name || `Terminal ${tabs.value.length + 1}`
    tabs.value.push({ id, name: tabName, cwd: cwd || '', isConnected: true, hasInput: false })
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

  function markInput(id: string) {
    const tab = tabs.value.find(t => t.id === id)
    if (tab) tab.hasInput = true
  }

  // 保存会话状态到文件
  function saveSession() {
    const activeIndex = tabs.value.findIndex(t => t.id === activeTabId.value)
    const session: SavedSession = {
      tabs: tabs.value.map(t => ({ name: t.name, cwd: t.cwd })),
      activeIndex: activeIndex >= 0 ? activeIndex : 0,
    }
    writeFile('session.json', session)
  }

  // 读取上次的会话状态
  async function getLastSession(): Promise<SavedSession | null> {
    return await readFile<SavedSession | null>('session.json', null)
  }

  // 读取 MCP 服务
  async function loadMcpServers(projectPath?: string) {
    try {
      mcpServers.value = await invoke<McpServer[]>('get_mcp_servers', { projectPath: projectPath || null })
    } catch (e) {
      console.error('Failed to load MCP servers:', e)
    }
  }

  // 读取已安装插件
  async function loadPlugins() {
    try {
      plugins.value = await invoke<ClaudePlugin[]>('get_claude_plugins')
    } catch (e) {
      console.error('Failed to load plugins:', e)
    }
  }

  return { tabs, activeTabId, gridMode, projects, claudeSessions, skills, plugins, mcpServers, init, addTab, removeTab, setActive, renameTab, setDisconnected, markInput, addProject, removeProject, findTabByCwd, saveSession, getLastSession, loadClaudeSessions, loadSkills, loadPlugins, loadMcpServers }
})
