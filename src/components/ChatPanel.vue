<template>
  <div class="chat-panel">
    <div class="chat-messages" ref="messagesEl">
      <div
        v-for="(msg, index) in messages"
        :key="index"
        :class="['chat-message', msg.type]"
      >
        <span class="msg-tag">{{ msg.type === 'user' ? 'YOU' : 'SYS' }}</span>
        <span class="msg-content">{{ msg.content }}</span>
        <span class="msg-time">{{ formatTime(msg.timestamp) }}</span>
      </div>
      <div v-if="messages.length === 0" class="chat-empty">
        输入指令发送到终端
      </div>
    </div>
    <div class="chat-input-area">
      <textarea
        ref="inputEl"
        v-model="inputText"
        class="chat-input"
        placeholder="输入指令..."
        rows="1"
        @keydown.enter.exact.prevent="send"
        @input="autoResize"
      ></textarea>
      <button class="chat-send" @click="send" :disabled="!inputText.trim()">
        &#x25B6;
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Message {
  type: 'user' | 'system'
  content: string
  timestamp: Date
}

const props = defineProps<{
  ptyId: string | null
}>()

const inputText = ref('')
const messages = ref<Message[]>([])
const messagesEl = ref<HTMLElement>()
const inputEl = ref<HTMLTextAreaElement>()

function formatTime(date: Date) {
  return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' })
}

async function send() {
  const text = inputText.value.trim()
  if (!text || !props.ptyId) return

  messages.value.push({
    type: 'user',
    content: text,
    timestamp: new Date(),
  })

  // 发送到终端
  await invoke('write_pty', { id: props.ptyId, data: text + '\n' })

  inputText.value = ''
  // 重置输入框高度
  if (inputEl.value) inputEl.value.style.height = 'auto'

  nextTick(() => {
    messagesEl.value?.scrollTo({ top: messagesEl.value.scrollHeight, behavior: 'smooth' })
  })
}

function autoResize() {
  if (inputEl.value) {
    inputEl.value.style.height = 'auto'
    inputEl.value.style.height = Math.min(inputEl.value.scrollHeight, 120) + 'px'
  }
}

// 当切换标签时清空消息
watch(() => props.ptyId, () => {
  messages.value = []
})
</script>

<style scoped>
.chat-panel {
  height: 200px;
  min-height: 120px;
  background: var(--bg-panel);
  border-top: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
}

.chat-messages {
  flex: 1;
  overflow-y: auto;
  padding: 8px 12px;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.chat-message {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  padding: 6px 10px;
  border-radius: 4px;
  font-size: 12px;
  max-width: 90%;
}

.chat-message.user {
  align-self: flex-end;
  background: var(--accent-dim);
  border: 1px solid rgba(255, 107, 43, 0.2);
}

.chat-message.system {
  align-self: flex-start;
  background: rgba(106, 106, 138, 0.1);
  border: 1px solid rgba(106, 106, 138, 0.2);
}

.msg-tag {
  font-size: 9px;
  font-weight: 600;
  letter-spacing: 0.5px;
  flex-shrink: 0;
  padding: 1px 4px;
  border-radius: 2px;
}

.chat-message.user .msg-tag {
  color: var(--accent);
  background: var(--accent-dim);
}

.chat-message.system .msg-tag {
  color: var(--text-secondary);
  background: rgba(106, 106, 138, 0.1);
}

.msg-content {
  color: var(--text-primary);
  flex: 1;
  word-break: break-all;
  white-space: pre-wrap;
}

.msg-time {
  font-size: 9px;
  color: var(--text-secondary);
  flex-shrink: 0;
  align-self: flex-end;
}

.chat-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-secondary);
  font-size: 12px;
}

.chat-input-area {
  display: flex;
  align-items: flex-end;
  gap: 8px;
  padding: 8px 12px;
  border-top: 1px solid var(--border);
}

.chat-input {
  flex: 1;
  background: var(--bg-primary);
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-primary);
  font-family: 'Fira Code', monospace;
  font-size: 12px;
  padding: 8px 10px;
  resize: none;
  outline: none;
  transition: border-color 0.2s;
  line-height: 1.4;
}

.chat-input::placeholder {
  color: var(--text-secondary);
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
  border-radius: 4px;
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
</style>
