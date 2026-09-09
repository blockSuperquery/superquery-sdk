<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui'
import { links } from '~/utils/project'

const navigation: NavigationMenuItem[] = [
  { label: 'SDK', to: '/sdk' },
  { label: 'Documentation', to: '/docs' },
  { label: 'Examples', to: '/examples' },
  { label: 'Roadmap', to: '/roadmap' }
]
</script>

<template>
  <UHeader
    title="SuperQuery"
    :ui="{ root: 'bg-default/95', container: 'gap-4' }"
  >
    <template #title>
      <AppLogo
        variant="mark"
        class="size-8 shrink-0"
      />
      <span class="font-display text-xl font-bold tracking-tight">SuperQuery</span>
    </template>
    <UNavigationMenu
      :items="navigation"
      aria-label="Main navigation"
    />
    <template #right>
      <UColorModeButton />
      <UButton
        :to="links.sdk"
        target="_blank"
        icon="i-lucide-github"
        variant="ghost"
        color="neutral"
        aria-label="SuperQuery on GitHub"
        class="hidden sm:inline-flex"
      />
      <UButton
        to="/docs"
        trailing-icon="i-lucide-arrow-up-right"
        class="hidden sm:inline-flex"
      >
        Start building
      </UButton>
    </template>
    <template #body>
      <UNavigationMenu
        :items="navigation"
        orientation="vertical"
        aria-label="Mobile navigation"
      />
      <USeparator class="my-6" />
      <UButton
        to="/docs"
        block
        trailing-icon="i-lucide-arrow-right"
      >
        Start building
      </UButton>
    </template>
  </UHeader>
</template>
