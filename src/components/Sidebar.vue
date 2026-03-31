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
      <div class="nav-spacer"></div>
      <span class="nav-version">v{{ appVersion }}</span>
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
              :class="['project-item', { linked: !!store.findTabByCwd(path), active: isActiveProject(path) }]"
              @click="onClickProject(path)"
            >
              <div class="project-info">
                <span class="project-name">{{ getProjectName(path) }}</span>
                <span class="project-path">{{ path }}</span>
              </div>
              <div class="project-actions">
                <span v-if="store.findTabByCwd(path)" class="project-linked" title="已关联终端">●</span>
                <button class="project-remove" @click.stop="requestRemove(path)" title="移除">✕</button>
              </div>
            </div>
          </div>
          <div v-if="store.projects.length === 0" class="panel-empty">
            暂无项目，点击下方添加
          </div>
          <button class="add-project-btn" @click="onAddProject">+ 添加项目目录</button>
        </template>

        <!-- Claude 会话历史面板 -->
        <template v-else-if="activePanel === 'history'">
          <div class="history-list">
            <div
              v-for="session in store.claudeSessions"
              :key="session.session_id"
              :class="['history-item', { active: browsingSession === session.session_id }]"
              @click="browseSession(session.session_id)"
            >
              <div class="history-info">
                <span class="history-name">{{ session.first_message }}</span>
                <span class="history-path">{{ getProjectName(session.project) }}</span>
                <div class="history-meta">
                  <span class="history-time">{{ formatTimestamp(session.last_timestamp) }}</span>
                  <span class="history-count">{{ session.message_count }} 条消息</span>
                </div>
              </div>
            </div>
          </div>
          <div v-if="store.claudeSessions.length === 0" class="panel-empty">
            暂无 Claude 会话记录
          </div>
          <button class="add-project-btn" @click="store.loadClaudeSessions()">↻ 刷新</button>
        </template>

        <!-- 设置面板 -->
        <template v-else-if="activePanel === 'settings'">
          <div class="settings-section">
            <div class="settings-label">深色主题</div>
            <div class="theme-grid">
              <div
                v-for="t in darkThemes"
                :key="t.id"
                :class="['theme-card', { active: currentTheme === t.id }]"
                @click="switchTheme(t.id)"
              >
                <span class="theme-dot" :style="{ background: t.preview }"></span>
                <span class="theme-name">{{ t.name }}</span>
              </div>
            </div>
          </div>
          <div class="settings-section">
            <div class="settings-label">浅色主题</div>
            <div class="theme-grid">
              <div
                v-for="t in lightThemes"
                :key="t.id"
                :class="['theme-card', { active: currentTheme === t.id }]"
                @click="switchTheme(t.id)"
              >
                <span class="theme-dot" :style="{ background: t.preview }"></span>
                <span class="theme-name">{{ t.name }}</span>
              </div>
            </div>
          </div>
          <!-- 版本更新 -->
          <div class="settings-section">
            <div class="settings-label">版本更新</div>
            <div class="version-info">
              <span class="version-current">当前版本：v{{ appVersion }}</span>
              <div v-if="updateInfo && updateInfo.has_update" class="version-new">
                <span class="version-badge">新版本 v{{ updateInfo.latest }}</span>
                <p class="version-notes" v-if="updateInfo.release_notes">{{ updateInfo.release_notes }}</p>
                <button class="version-download" @click="downloadUpdate" :disabled="downloading">
                  {{ downloading ? '下载中...' : '下载更新' }}
                </button>
                <span v-if="downloadResult" class="version-result">{{ downloadResult }}</span>
              </div>
              <div v-else-if="updateInfo && !updateInfo.has_update" class="version-uptodate">
                已是最新版本
              </div>
              <button class="add-project-btn" @click="checkForUpdate" :disabled="checkingUpdate">
                {{ checkingUpdate ? '检查中...' : '检查更新' }}
              </button>
            </div>
          </div>
        </template>

        <!-- 技能面板 -->
        <template v-else-if="activePanel === 'skills'">
          <div class="skill-list">
            <div
              v-for="skill in store.skills"
              :key="skill.id"
              :class="['skill-item', { active: expandedSkill === skill.id }]"
              @click="expandedSkill = expandedSkill === skill.id ? null : skill.id"
            >
              <div class="skill-header">
                <span class="skill-name">{{ skill.name }}</span>
                <span class="skill-source">{{ skill.source === 'global' ? '全局' : '项目' }}</span>
              </div>
              <span class="skill-desc">{{ skill.description }}</span>
            </div>
          </div>
          <div v-if="store.skills.length === 0" class="panel-empty">
            暂无已安装的技能
          </div>
          <button class="add-project-btn" @click="store.loadSkills()">↻ 刷新</button>
        </template>

        <!-- 插件面板 -->
        <template v-else-if="activePanel === 'plugins'">
          <div class="plugin-list">
            <div
              v-for="plugin in store.plugins"
              :key="plugin.id"
              class="plugin-item"
            >
              <div class="plugin-info">
                <span class="plugin-name">{{ plugin.name }}</span>
                <span class="plugin-desc" v-if="plugin.description">{{ plugin.description }}</span>
                <div class="plugin-meta">
                  <span class="plugin-market">{{ plugin.marketplace }}</span>
                  <span class="plugin-version">v{{ plugin.version }}</span>
                </div>
              </div>
            </div>
          </div>
          <div v-if="store.plugins.length === 0" class="panel-empty">
            暂无已安装的插件
          </div>
          <button class="add-project-btn" @click="store.loadPlugins()">↻ 刷新</button>
        </template>

        <!-- MCP 面板 -->
        <template v-else-if="activePanel === 'mcp'">
          <div class="mcp-list">
            <div
              v-for="server in store.mcpServers"
              :key="server.name"
              class="mcp-item"
            >
              <div class="mcp-info">
                <span class="mcp-name">{{ server.name }}</span>
                <span class="mcp-cmd">{{ server.command }} {{ server.args.join(' ') }}</span>
                <span class="skill-source">{{ server.source === 'global' ? '全局' : '项目' }}</span>
              </div>
            </div>
          </div>
          <div v-if="store.mcpServers.length === 0" class="panel-empty">
            暂无 MCP 服务
          </div>
          <button class="add-project-btn" @click="store.loadMcpServers()">↻ 刷新</button>
        </template>

        <!-- 其他面板占位 -->
        <template v-else>
          <p class="panel-placeholder">{{ currentLabel }}功能开发中...</p>
        </template>
      </div>
    </div>
    <!-- 会话浏览器（独立右侧面板） -->
    <div v-if="browsingSession" class="session-panel">
      <div class="panel-header">
        <span class="panel-title">会话详情</span>
        <button class="panel-close" @click="browsingSession = null">✕</button>
      </div>
      <div class="session-browser">
        <div v-if="sessionLoading" class="panel-empty">加载中...</div>
        <div v-else class="message-list">
          <div
            v-for="(msg, i) in sessionMessages"
            :key="i"
            :class="['message-item', msg.role]"
          >
            <div class="message-role">{{ msg.role === 'user' ? '👤 用户' : '🤖 Claude' }}</div>
            <div class="message-text">{{ msg.text }}</div>
            <div class="message-time" v-if="msg.timestamp">{{ formatTimestamp(new Date(msg.timestamp).getTime()) }}</div>
          </div>
        </div>
      </div>
      <div class="session-footer">
        <button class="history-btn resume" @click="emit('resume-session', browsingSession!, browsingProject)">恢复会话</button>
      </div>
    </div>

    <!-- 技能详情面板 -->
    <div v-if="expandedSkill && activePanel === 'skills'" class="session-panel">
      <div class="panel-header">
        <span class="panel-title">{{ currentSkill?.name }}</span>
        <button class="panel-close" @click="expandedSkill = null">✕</button>
      </div>
      <div class="session-browser">
        <div class="skill-detail-header">
          <span class="skill-source">{{ currentSkill?.source === 'global' ? '全局技能' : '项目技能' }}</span>
        </div>
        <p class="skill-detail-desc">{{ currentSkill?.description }}</p>
        <pre class="skill-content">{{ currentSkill?.content }}</pre>
      </div>
    </div>

    <!-- 移除确认弹窗 -->
    <Teleport to="body">
      <div v-if="removingPath" class="confirm-overlay" @click.self="removingPath = null">
        <div class="confirm-dialog">
          <p class="confirm-text">确定移除项目「{{ getProjectName(removingPath) }}」？</p>
          <div class="confirm-actions">
            <button class="confirm-btn cancel" @click="removingPath = null">取消</button>
            <button class="confirm-btn ok" @click="confirmRemove">确定移除</button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useTerminalStore } from '@/stores/terminal'
import { themes, applyTheme, getStoredThemeId } from '@/themes'

interface SessionMessage {
  role: string
  text: string
  timestamp: string
}

const store = useTerminalStore()

// 版本信息
const appVersion = ref('0.1.0')
const updateInfo = ref<{ latest: string; has_update: boolean; download_url: string; release_notes: string } | null>(null)
const checkingUpdate = ref(false)

invoke<string>('get_current_version').then(v => { appVersion.value = v })

async function checkForUpdate() {
  checkingUpdate.value = true
  try {
    const info = await invoke<typeof updateInfo.value>('check_update')
    updateInfo.value = info
  } catch (e) {
    console.error('检查更新失败:', e)
    updateInfo.value = null
  }
  checkingUpdate.value = false
}

const downloading = ref(false)
const downloadResult = ref('')

async function downloadUpdate() {
  if (!updateInfo.value?.download_url) return
  downloading.value = true
  downloadResult.value = ''
  try {
    const path = await invoke<string>('download_update', { url: updateInfo.value.download_url })
    downloadResult.value = `已下载到：${path}`
  } catch (e) {
    downloadResult.value = `下载失败：${e}`
  }
  downloading.value = false
}

const emit = defineEmits<{
  'open-project': [path: string]
  'add-project': []
  'resume-session': [sessionId: string, project: string]
}>()

const menuItems = [
  { id: 'projects', icon: '◫', label: '项目' },
  { id: 'history', icon: '◷', label: '历史' },
  { id: 'skills', icon: '◆', label: '技能' },
  { id: 'plugins', icon: '⊞', label: '插件' },
  { id: 'mcp', icon: '⬡', label: 'MCP' },
  { id: 'settings', icon: '⚙', label: '设置' },
]

const activePanel = ref<string | null>(null)

const currentLabel = computed(() =>
  menuItems.find(i => i.id === activePanel.value)?.label || ''
)

function togglePanel(id: string) {
  activePanel.value = activePanel.value === id ? null : id
  expandedSkill.value = null
  browsingSession.value = null
  if (activePanel.value === 'history') {
    store.loadClaudeSessions()
  }
  if (activePanel.value === 'mcp') {
    const activeTab = store.tabs.find(t => t.id === store.activeTabId)
    store.loadMcpServers(activeTab?.cwd || undefined)
  }
  if (activePanel.value === 'plugins') {
    store.loadPlugins()
  }
  if (activePanel.value === 'skills') {
    // 当前激活标签的项目路径
    const activeTab = store.tabs.find(t => t.id === store.activeTabId)
    store.loadSkills(activeTab?.cwd || undefined)
  }
}

const browsingSession = ref<string | null>(null)
const browsingProject = ref('')
const sessionMessages = ref<SessionMessage[]>([])
const sessionLoading = ref(false)
const removingPath = ref<string | null>(null)
const expandedSkill = ref<string | null>(null)
const currentSkill = computed(() => store.skills.find(s => s.id === expandedSkill.value))

async function browseSession(sessionId: string) {
  browsingSession.value = sessionId
  const session = store.claudeSessions.find(s => s.session_id === sessionId)
  browsingProject.value = session?.project || ''
  sessionLoading.value = true
  try {
    sessionMessages.value = await invoke<SessionMessage[]>('get_session_messages', { sessionId })
  } catch (e) {
    console.error('Failed to load session:', e)
    sessionMessages.value = []
  }
  sessionLoading.value = false
}

function requestRemove(path: string) {
  removingPath.value = path
}

function confirmRemove() {
  if (removingPath.value) {
    store.removeProject(removingPath.value)
    removingPath.value = null
  }
}

function isActiveProject(path: string): boolean {
  const activeTab = store.tabs.find(t => t.id === store.activeTabId)
  return !!activeTab && activeTab.cwd === path
}

function onClickProject(path: string) {
  emit('open-project', path)
}

function onAddProject() {
  emit('add-project')
}

// 主题
const darkThemes = computed(() => themes.filter(t => t.mode === 'dark'))
const lightThemes = computed(() => themes.filter(t => t.mode === 'light'))
const currentTheme = ref('dark-orange')
getStoredThemeId().then(id => { currentTheme.value = id })

function switchTheme(id: string) {
  currentTheme.value = id
  applyTheme(id)
}

function getProjectName(path: string) {
  return path.split('/').pop() || path.split('\\').pop() || path
}

function formatTimestamp(ts: number): string {
  const d = new Date(ts)
  const now = new Date()
  const isToday = d.toDateString() === now.toDateString()
  const time = d.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' })
  if (isToday) return `今天 ${time}`
  const yesterday = new Date(now)
  yesterday.setDate(yesterday.getDate() - 1)
  if (d.toDateString() === yesterday.toDateString()) return `昨天 ${time}`
  return `${d.getMonth() + 1}/${d.getDate()} ${time}`
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
  color: var(--text-secondary);
  transition: color 0.15s;
}

.nav-spacer {
  flex: 1;
}

.nav-version {
  font-size: 9px;
  color: var(--text-muted);
  font-family: var(--font-mono);
  text-align: center;
  padding: 8px 0;
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
  position: relative;
  z-index: 100;
}

.session-panel {
  width: 420px;
  background: var(--bg-panel);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  z-index: 100;
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
  background: transparent;
}

.project-item.active {
  background: var(--accent-dim);
  border-left: 3px solid var(--accent);
}

.project-item.active .project-name {
  color: var(--accent-light);
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
  color: #e5e5e5;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.project-path {
  font-size: 10px;
  color: var(--text-secondary);
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

/* 历史列表 */
.history-list {
  flex: 1;
}

.history-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 14px;
  cursor: pointer;
  transition: background 0.15s;
}

.history-item:hover {
  background: var(--bg-sidebar-hover);
}

.history-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
  flex: 1;
}

.history-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.history-path {
  font-size: 10px;
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-family: var(--font-mono);
}

.history-time {
  font-size: 10px;
  color: var(--text-muted);
}

.history-item.active {
  background: var(--accent-dim);
  border-left: 3px solid var(--accent);
}

.history-item.active .history-name {
  color: var(--accent-light);
}

.history-meta {
  display: flex;
  gap: 8px;
  align-items: center;
}

.history-count {
  font-size: 10px;
  color: var(--text-muted);
}

.history-actions {
  display: flex;
  gap: 6px;
  margin-top: 6px;
}

.history-btn {
  padding: 4px 10px;
  border-radius: 4px;
  font-family: var(--font-ui);
  font-size: 11px;
  cursor: pointer;
  transition: all 0.15s;
  border: 1px solid var(--border);
  background: transparent;
}

.history-btn.resume {
  color: var(--accent);
  border-color: var(--accent);
}

.history-btn.resume:hover {
  background: var(--accent-dim);
}

.history-btn.open {
  color: var(--text-secondary);
}

.history-btn.open:hover {
  color: var(--text-primary);
  border-color: var(--text-secondary);
}

.session-footer {
  padding: 10px 14px;
  border-top: 1px solid var(--border);
  display: flex;
  justify-content: center;
}

.session-footer .history-btn {
  width: 100%;
  padding: 8px;
  text-align: center;
}

/* 会话浏览器 */
.session-browser {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.message-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.message-item {
  padding: 10px 12px;
  border-radius: 8px;
  border-left: 3px solid transparent;
}

.message-item.user {
  background: rgba(255, 107, 43, 0.08);
  border-left-color: var(--accent);
}

.message-item.assistant {
  background: rgba(96, 165, 250, 0.08);
  border-left-color: var(--color-info);
}

.message-role {
  font-size: 11px;
  font-weight: 600;
  margin-bottom: 4px;
  color: var(--text-secondary);
}

.message-text {
  font-size: 12px;
  color: var(--text-primary);
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-word;
  max-height: 200px;
  overflow-y: auto;
}

.message-time {
  font-size: 10px;
  color: var(--text-muted);
  margin-top: 4px;
  text-align: right;
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

/* 设置面板 */
.settings-section {
  padding: 12px 14px;
}

.settings-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 8px;
}

.theme-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 6px;
}

.theme-card {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 10px;
  border-radius: 6px;
  cursor: pointer;
  border: 1px solid var(--border);
  transition: all 0.15s;
}

.theme-card:hover {
  border-color: var(--text-muted);
  background: var(--bg-sidebar-hover);
}

.theme-card.active {
  border-color: var(--accent);
  background: var(--accent-dim);
}

.theme-dot {
  width: 14px;
  height: 14px;
  border-radius: 50%;
  flex-shrink: 0;
}

.theme-name {
  font-size: 11px;
  color: var(--text-primary);
  white-space: nowrap;
}

.theme-card.active .theme-name {
  color: var(--accent-light);
}

/* 版本更新 */
.version-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.version-current {
  font-size: 12px;
  color: var(--text-secondary);
  font-family: var(--font-mono);
}

.version-new {
  padding: 8px 10px;
  background: var(--accent-dim);
  border: 1px solid rgba(255, 107, 43, 0.2);
  border-radius: 6px;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.version-badge {
  font-size: 12px;
  font-weight: 600;
  color: var(--accent);
}

.version-notes {
  font-size: 11px;
  color: var(--text-secondary);
  line-height: 1.4;
  max-height: 80px;
  overflow-y: auto;
  white-space: pre-wrap;
}

.version-download {
  padding: 6px 12px;
  background: var(--accent);
  border: none;
  border-radius: 4px;
  color: white;
  font-family: var(--font-ui);
  font-size: 12px;
  cursor: pointer;
  transition: all 0.15s;
  align-self: flex-start;
}

.version-download:hover {
  background: var(--accent-light);
}

.version-uptodate {
  font-size: 12px;
  color: var(--color-success);
}

.version-result {
  font-size: 10px;
  color: var(--text-secondary);
  word-break: break-all;
}

/* 技能面板 */
.skill-list {
  flex: 1;
  overflow-y: auto;
}

.skill-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 10px 14px;
  cursor: pointer;
  transition: background 0.15s;
  border-bottom: 1px solid var(--border);
}

.skill-item:hover {
  background: var(--bg-sidebar-hover);
}

.skill-item.expanded {
  background: var(--accent-dim);
}

.skill-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.skill-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.skill-source {
  font-size: 9px;
  color: var(--text-muted);
  padding: 1px 6px;
  border: 1px solid var(--border);
  border-radius: 3px;
  font-family: var(--font-mono);
}

.skill-desc {
  font-size: 11px;
  color: var(--text-secondary);
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.skill-detail {
  margin-top: 8px;
  max-height: 300px;
  overflow-y: auto;
}

.skill-detail-header {
  margin-bottom: 8px;
}

.skill-detail-desc {
  font-size: 13px;
  color: var(--text-primary);
  line-height: 1.6;
  margin-bottom: 12px;
  padding: 0 4px;
}

.skill-content {
  font-size: 11px;
  color: var(--text-secondary);
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-word;
  font-family: var(--font-mono);
  background: var(--bg-primary);
  padding: 10px;
  border-radius: 6px;
  margin: 0;
}

/* 插件面板 */
.plugin-list {
  flex: 1;
  overflow-y: auto;
}

.plugin-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
  border-bottom: 1px solid var(--border);
}

.plugin-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.plugin-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.plugin-desc {
  font-size: 11px;
  color: var(--text-secondary);
  line-height: 1.4;
}

.plugin-meta {
  display: flex;
  gap: 8px;
  align-items: center;
}

.plugin-market, .plugin-version {
  font-size: 9px;
  color: var(--text-muted);
  font-family: var(--font-mono);
}

.plugin-market {
  padding: 1px 6px;
  border: 1px solid var(--border);
  border-radius: 3px;
}

/* MCP 面板 */
.mcp-list {
  flex: 1;
  overflow-y: auto;
}

.mcp-item {
  padding: 10px 14px;
  border-bottom: 1px solid var(--border);
}

.mcp-info {
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.mcp-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.mcp-cmd {
  font-size: 10px;
  color: var(--text-secondary);
  font-family: var(--font-mono);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
