<script setup lang="ts">
import { useClipboard } from '@vueuse/core'

const props = withDefaults(defineProps<{ code: string, filename?: string }>(), { filename: 'terminal' })
const { copy, copied, isSupported } = useClipboard({ source: () => props.code, copiedDuring: 2000 })
const toast = useToast()
async function copyCode() {
  try {
    await copy()
  } catch {
    toast.add({ title: 'Copy unavailable', description: 'Select the code and copy it using your browser.', color: 'error' })
  }
}
</script>

<template>
  <div class="min-w-0 overflow-hidden rounded-xl border border-slate-700 bg-ink text-slate-200 shadow-sm">
    <div class="flex min-h-12 items-center justify-between gap-3 border-b border-white/10 px-4">
      <span class="font-mono text-xs text-slate-400">{{ filename }}</span>
      <ClientOnly>
        <UButton v-if="isSupported" :icon="copied ? 'i-lucide-check' : 'i-lucide-copy'" variant="link" color="neutral" size="xs" :aria-label="`Copy ${filename}`" class="text-slate-300 hover:text-white" @click="copyCode">{{ copied ? 'Copied' : 'Copy' }}</UButton>
      </ClientOnly>
      <span role="status" class="sr-only">{{ copied ? 'Code copied to clipboard' : '' }}</span>
    </div>
    <pre class="overflow-x-auto p-5 text-[12px] leading-7 sm:p-6 sm:text-[13px]" tabindex="0" :aria-label="filename"><code>{{ code }}</code></pre>
  </div>
</template>
