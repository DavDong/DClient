<template>
  <div class="chat-panel">
    <!-- 已选附件 -->
    <div v-if="attachments.length > 0" class="attachments">
      <div v-for="(att, i) in attachments" :key="i" class="attachment-tag">
        <span class="att-icon">{{ att.type === 'folder' ? '📁' : '📄' }}</span>
        <span class="att-name">{{ att.name }}</span>
        <button class="att-remove" @click="attachments.splice(i, 1)">✕</button>
      </div>
    </div>
    <!-- 输入区域 -->
    <div class="chat-input-area">
      <textarea
        ref="inputEl"
        v-model="inputText"
        class="chat-input"
        placeholder="输入指令发送到终端..."
        rows="3"
        @keydown.enter.exact.prevent="send"
      ></textarea>
      <button class="chat-send" @click="send" :disabled="!canSend">
        ▶
      </button>
    </div>
    <!-- 底部工具栏 -->
    <div class="chat-toolbar">
      <button class="tool-btn" @click="pickFolder" title="选择文件夹">📁 文件夹</button>
      <button class="tool-btn" @click="pickFile" title="选择文件">📄 文件</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'

interface Attachment {
  type: 'folder' | 'file'
  path: string
  name: string
}

const props = defineProps<{
  ptyId: string | null
}>()

const inputText = ref('')
const attachments = ref<Attachment[]>([])
const inputEl = ref<HTMLTextAreaElement>()

const canSend = computed(() => inputText.value.trim() || attachments.value.length > 0)

async function pickFolder() {
  const selected = await open({ directory: true, multiple: false, title: '选择文件夹' })
  if (selected) {
    const path = selected as string
    attachments.value.push({
      type: 'folder',
      path,
      name: path.split('/').pop() || path.split('\\').pop() || path,
    })
  }
}

async function pickFile() {
  const selected = await open({ multiple: true, title: '选择文件' })
  if (selected) {
    const paths = Array.isArray(selected) ? selected : [selected]
    for (const path of paths) {
      attachments.value.push({
        type: 'file',
        path,
        name: path.split('/').pop() || path.split('\\').pop() || path,
      })
    }
  }
}

async function send() {
  if (!canSend.value || !props.ptyId) return

  // 组合附件路径和输入文字
  let command = ''
  if (attachments.value.length > 0) {
    const paths = attachments.value.map(a => `"${a.path}"`).join(' ')
    command = inputText.value.trim()
      ? `${inputText.value.trim()} ${paths}`
      : paths
  } else {
    command = inputText.value.trim()
  }

  await invoke('write_pty', { id: props.ptyId, data: command + '\n' })

  inputText.value = ''
  attachments.value = []
}
</script>

<style scoped>
.chat-panel {
  background: var(--bg-panel);
  border-top: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
}

/* 附件区域 */
.attachments {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  padding: 8px 12px 0;
}

.attachment-tag {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 3px 8px;
  background: var(--accent-dim);
  border: 1px solid rgba(255, 107, 43, 0.2);
  border-radius: 4px;
  font-size: 11px;
}

.att-icon {
  font-size: 12px;
}

.att-name {
  color: var(--text-primary);
  max-width: 150px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.att-remove {
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  font-size: 10px;
  padding: 0 2px;
  transition: color 0.15s;
}

.att-remove:hover {
  color: var(--color-error);
}

/* 输入区域 */
.chat-input-area {
  display: flex;
  align-items: flex-end;
  gap: 8px;
  padding: 8px 12px;
}

.chat-input {
  flex: 1;
  background: var(--bg-primary);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text-primary);
  font-family: var(--font-mono);
  font-size: 12px;
  padding: 8px 10px;
  resize: none;
  outline: none;
  transition: border-color 0.2s;
  line-height: 1.5;
}

.chat-input::placeholder {
  color: var(--text-muted);
}

.chat-input:focus {
  border-color: var(--accent);
}

.chat-send {
  width: 36px;
  height: 36px;
  flex-shrink: 0;
  background: transparent;
  border: 1px solid var(--accent);
  border-radius: 6px;
  color: var(--accent);
  font-size: 14px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.chat-send:hover:not(:disabled) {
  background: var(--accent-dim);
  color: var(--accent-light);
  border-color: var(--accent-light);
}

.chat-send:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

/* 底部工具栏 */
.chat-toolbar {
  display: flex;
  gap: 4px;
  padding: 0 12px 8px;
}

.tool-btn {
  background: none;
  border: none;
  color: var(--text-muted);
  font-size: 11px;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 4px;
  transition: all 0.15s;
}

.tool-btn:hover {
  color: var(--text-secondary);
  background: var(--bg-sidebar-hover);
}
</style>
