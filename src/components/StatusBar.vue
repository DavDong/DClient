<template>
  <div class="status-bar">
    <div class="status-left">
      <span class="status-indicator" :class="{ connected }">&#x25C9;</span>
      <span>{{ connected ? 'CONNECTED' : 'DISCONNECTED' }}</span>
    </div>
    <div class="status-right">
      <span class="status-item">{{ shell }}</span>
      <span class="status-separator">&#x2502;</span>
      <span class="status-item">{{ cols }}&times;{{ rows }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  connected: boolean
  shell: string
  cols: number
  rows: number
}>()
</script>

<style scoped>
.status-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 28px;
  padding: 0 12px;
  background: var(--bg-tab);
  border-top: 1px solid var(--border);
  font-size: 11px;
  color: var(--text-secondary);
}

.status-left {
  display: flex;
  align-items: center;
  gap: 6px;
}

.status-indicator {
  font-size: 8px;
  color: var(--neon-red);
}

.status-indicator.connected {
  color: var(--neon-green);
  text-shadow: var(--glow-green);
}

.status-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-separator {
  color: var(--border);
}

.status-item {
  color: var(--text-secondary);
}
</style>
