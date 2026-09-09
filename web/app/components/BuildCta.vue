<script setup lang="ts">
import { links } from '~/utils/project'
</script>

<template>
  <UContainer class="py-16 sm:py-24">
    <UPageCTA title="Your next idea starts with a schema." description="Explore the EVM starter, validate your project, and help shape what comes next." variant="solid" :links="[{ label: 'Read the quickstart', to: '/docs', trailingIcon: 'i-lucide-arrow-right' }, { label: 'Explore GitHub', to: links.organization, target: '_blank', icon: 'i-lucide-github', color: 'neutral', variant: 'outline' }]" :ui="{ container: 'py-12 sm:py-16', title: 'text-3xl sm:text-4xl' }" />
  </UContainer>
</template>
