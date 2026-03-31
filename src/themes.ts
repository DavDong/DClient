export interface Theme {
  id: string
  name: string
  mode: 'dark' | 'light'
  preview: string // 主色预览色
  vars: Record<string, string>
}

const darkBase = {
  '--bg-primary': '#1a1a1a',
  '--bg-panel': '#222222',
  '--bg-tab': '#1e1e1e',
  '--bg-tab-active': '#2c2c2c',
  '--bg-tab-hover': '#2a2a2a',
  '--bg-sidebar': '#1a1a1a',
  '--bg-sidebar-hover': '#2a2a2a',
  '--bg-sidebar-active': '#2c2c2c',
  '--bg-input': '#2a2a2a',
  '--color-success': '#4ade80',
  '--color-warning': '#fbbf24',
  '--color-error': '#f87171',
  '--color-info': '#60a5fa',
  '--text-primary': '#f0f0f0',
  '--text-secondary': '#aaaaaa',
  '--text-muted': '#777777',
  '--border': '#333333',
  '--border-light': '#3a3a3a',
}

const lightBase = {
  '--bg-primary': '#f5f5f5',
  '--bg-panel': '#ffffff',
  '--bg-tab': '#e8e8e8',
  '--bg-tab-active': '#ffffff',
  '--bg-tab-hover': '#eeeeee',
  '--bg-sidebar': '#f0f0f0',
  '--bg-sidebar-hover': '#e4e4e4',
  '--bg-sidebar-active': '#dcdcdc',
  '--bg-input': '#ffffff',
  '--color-success': '#16a34a',
  '--color-warning': '#d97706',
  '--color-error': '#dc2626',
  '--color-info': '#2563eb',
  '--text-primary': '#1a1a1a',
  '--text-secondary': '#555555',
  '--text-muted': '#999999',
  '--border': '#d4d4d4',
  '--border-light': '#e0e0e0',
}

export const themes: Theme[] = [
  // 深色主题
  {
    id: 'dark-orange',
    name: '暗夜橙',
    mode: 'dark',
    preview: '#ff6b2b',
    vars: {
      ...darkBase,
      '--accent': '#ff6b2b',
      '--accent-light': '#ff8c55',
      '--accent-dim': 'rgba(255, 107, 43, 0.15)',
      '--accent-glow': '0 0 8px rgba(255, 107, 43, 0.25)',
    },
  },
  {
    id: 'dark-blue',
    name: '深海蓝',
    mode: 'dark',
    preview: '#3b82f6',
    vars: {
      ...darkBase,
      '--accent': '#3b82f6',
      '--accent-light': '#60a5fa',
      '--accent-dim': 'rgba(59, 130, 246, 0.15)',
      '--accent-glow': '0 0 8px rgba(59, 130, 246, 0.25)',
    },
  },
  {
    id: 'dark-green',
    name: '矩阵绿',
    mode: 'dark',
    preview: '#22c55e',
    vars: {
      ...darkBase,
      '--accent': '#22c55e',
      '--accent-light': '#4ade80',
      '--accent-dim': 'rgba(34, 197, 94, 0.15)',
      '--accent-glow': '0 0 8px rgba(34, 197, 94, 0.25)',
    },
  },
  {
    id: 'dark-purple',
    name: '极光紫',
    mode: 'dark',
    preview: '#a855f7',
    vars: {
      ...darkBase,
      '--accent': '#a855f7',
      '--accent-light': '#c084fc',
      '--accent-dim': 'rgba(168, 85, 247, 0.15)',
      '--accent-glow': '0 0 8px rgba(168, 85, 247, 0.25)',
    },
  },
  {
    id: 'dark-cyan',
    name: '霓虹青',
    mode: 'dark',
    preview: '#06b6d4',
    vars: {
      ...darkBase,
      '--accent': '#06b6d4',
      '--accent-light': '#22d3ee',
      '--accent-dim': 'rgba(6, 182, 212, 0.15)',
      '--accent-glow': '0 0 8px rgba(6, 182, 212, 0.25)',
    },
  },
  {
    id: 'dark-rose',
    name: '玫瑰红',
    mode: 'dark',
    preview: '#f43f5e',
    vars: {
      ...darkBase,
      '--accent': '#f43f5e',
      '--accent-light': '#fb7185',
      '--accent-dim': 'rgba(244, 63, 94, 0.15)',
      '--accent-glow': '0 0 8px rgba(244, 63, 94, 0.25)',
    },
  },
  // 浅色主题
  {
    id: 'light-orange',
    name: '日光橙',
    mode: 'light',
    preview: '#ea580c',
    vars: {
      ...lightBase,
      '--accent': '#ea580c',
      '--accent-light': '#f97316',
      '--accent-dim': 'rgba(234, 88, 12, 0.1)',
      '--accent-glow': '0 0 8px rgba(234, 88, 12, 0.15)',
    },
  },
  {
    id: 'light-blue',
    name: '晴空蓝',
    mode: 'light',
    preview: '#2563eb',
    vars: {
      ...lightBase,
      '--accent': '#2563eb',
      '--accent-light': '#3b82f6',
      '--accent-dim': 'rgba(37, 99, 235, 0.1)',
      '--accent-glow': '0 0 8px rgba(37, 99, 235, 0.15)',
    },
  },
  {
    id: 'light-green',
    name: '薄荷绿',
    mode: 'light',
    preview: '#16a34a',
    vars: {
      ...lightBase,
      '--accent': '#16a34a',
      '--accent-light': '#22c55e',
      '--accent-dim': 'rgba(22, 163, 74, 0.1)',
      '--accent-glow': '0 0 8px rgba(22, 163, 74, 0.15)',
    },
  },
  {
    id: 'light-purple',
    name: '薰衣草',
    mode: 'light',
    preview: '#7c3aed',
    vars: {
      ...lightBase,
      '--accent': '#7c3aed',
      '--accent-light': '#8b5cf6',
      '--accent-dim': 'rgba(124, 58, 237, 0.1)',
      '--accent-glow': '0 0 8px rgba(124, 58, 237, 0.15)',
    },
  },
]

export function applyTheme(themeId: string) {
  const theme = themes.find(t => t.id === themeId)
  if (!theme) return
  const root = document.documentElement
  for (const [key, value] of Object.entries(theme.vars)) {
    root.style.setProperty(key, value)
  }
  // 持久化到文件
  import('@tauri-apps/api/core').then(({ invoke }) => {
    invoke('write_config', { filename: 'config.json', data: JSON.stringify({ theme: themeId }, null, 2) })
  }).catch(() => {})
}

export async function getStoredThemeId(): Promise<string> {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const raw = await invoke<string>('read_config', { filename: 'config.json' })
    const config = JSON.parse(raw)
    return config.theme || 'dark-orange'
  } catch {
    return 'dark-orange'
  }
}
