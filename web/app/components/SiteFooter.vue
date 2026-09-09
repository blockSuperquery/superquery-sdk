<script setup lang="ts">
import { links } from '~/utils/project'

const columns = [
  { label: 'Build', children: [{ label: 'Get started', to: '/docs' }, { label: 'SDK reference', to: '/sdk' }, { label: 'Examples', to: '/examples' }, { label: 'Developer portal', to: '/portal' }] },
  { label: 'Project', children: [{ label: 'Roadmap', to: '/roadmap' }, { label: 'Changelog', to: '/changelog' }, { label: 'Project status', to: '/status' }, { label: 'Grants & ecosystem', to: '/grants' }] },
  { label: 'Open source', children: [{ label: 'SDK repository', to: links.sdk, target: '_blank' }, { label: 'Indexing engine', to: links.node, target: '_blank' }, { label: 'Query service', to: links.query, target: '_blank' }] }
]
</script>

<template>
  <UFooter class="border-t border-default">
    <template #top>
      <UContainer class="py-12 sm:py-16">
        <UFooterColumns :columns="columns">
          <template #left>
            <NuxtLink
              to="/"
              class="inline-flex items-center gap-2 font-display text-xl font-bold text-highlighted"
              aria-label="SuperQuery home"
            >
              <AppLogo
                variant="mark"
                class="size-8"
              />SuperQuery
            </NuxtLink>
            <p class="mt-4 max-w-xs text-sm leading-6 text-muted">
              From chain events to useful data.<br>Open source. Written in Rust. Built in public.
            </p>
          </template>
          <template #right>
            <p class="font-display font-semibold text-highlighted">
              Follow the build.
            </p>
            <p class="my-3 max-w-56 text-sm leading-6 text-muted">
              Track new capabilities and releases directly on GitHub.
            </p>
            <UButton
              :to="`${links.sdk}/subscription`"
              target="_blank"
              color="neutral"
              variant="outline"
              trailing-icon="i-lucide-arrow-up-right"
            >
              Watch the project
            </UButton>
          </template>
        </UFooterColumns>
      </UContainer>
    </template>
    <template #left>
      <p class="text-xs text-muted">
        © {{ new Date().getFullYear() }} SuperQuery
      </p>
    </template>
    <template #right>
      <NuxtLink
        to="/status"
        class="inline-flex items-center gap-2 text-xs text-muted"
      ><span class="size-1.5 rounded-full bg-amber-500" />Platform in development</NuxtLink>
    </template>
  </UFooter>
</template>
