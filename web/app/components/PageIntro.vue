<script setup lang="ts">
defineProps<{ eyebrow: string, title: string, description: string }>()
</script>

<template>
  <UPageHero
    :title="title"
    :description="description"
    :ui="{ container: 'py-16 sm:py-20 lg:py-24', title: 'text-4xl sm:text-5xl lg:text-6xl', description: 'max-w-2xl mx-auto text-base sm:text-lg' }"
  >
    <template #headline>
      <p class="eyebrow text-primary">
        {{ eyebrow }}
      </p>
    </template>
    <template
      v-if="$slots.default"
      #links
    >
      <slot />
    </template>
  </UPageHero>
</template>
