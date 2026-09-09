<script setup lang="ts">
import { links } from '~/utils/project'
useSeoMeta({ title: 'Project status', description: 'Current SuperQuery development status, repository CI links, and guidance for monitoring a self-hosted query service.' })
const projects = [
  { title: 'SDK', description: 'Validation and code generation available. Project builds and scaffolding are planned.', to: links.sdk, icon: 'i-lucide-terminal' },
  { title: 'Indexing node', description: 'Pre-alpha. Core components are tested; the end-to-end indexing pipeline is not yet connected.', to: links.node, icon: 'i-lucide-blocks' },
  { title: 'Query service', description: 'GraphQL reads available against a prepared database. History and subscriptions await node support.', to: links.query, icon: 'i-lucide-database' }
]
const endpoints = [
  { endpoint: 'GET /health', purpose: 'Liveness', behavior: 'Checks the query process without touching the database.' },
  { endpoint: 'GET /ready', purpose: 'Readiness', behavior: 'Returns 503 when PostgreSQL is unreachable.' },
  { endpoint: 'GET /meta', purpose: 'Index state', behavior: 'Reports indexed heights, chain, and entity metadata.' }
]
</script>

<template>
  <div>
    <PageIntro eyebrow="Project status" title="Know where the platform stands." description="Development status and links to the checks that back it up." />
    <UContainer class="space-y-12 pb-20">
      <UAlert title="Development snapshot, not live service monitoring" description="Reviewed on 9 September 2026. This site is not connected to production indexers and does not report uptime, throughput, or network health." icon="i-lucide-info" color="info" variant="soft" />
      <UPageGrid>
        <UPageCard v-for="project in projects" :key="project.title" :title="project.title" :description="project.description" :icon="project.icon">
          <template #footer><UButton :to="`${project.to}/actions`" target="_blank" variant="outline" color="neutral" trailing-icon="i-lucide-arrow-up-right">View CI runs</UButton></template>
        </UPageCard>
      </UPageGrid>
      <section class="space-y-6">
        <h2 class="text-2xl font-semibold text-highlighted">Monitoring your own query service</h2>
        <p class="page-copy max-w-3xl">Use separate checks for process health and database availability. A database outage should remove an instance from traffic without forcing a process restart.</p>
        <UTable :data="endpoints" :ui="{ td: 'whitespace-normal min-w-32' }" />
        <UButton :to="links.query" target="_blank" variant="outline" color="neutral" trailing-icon="i-lucide-arrow-up-right">Read the operations reference</UButton>
      </section>
      <UPageCard title="Found a problem?" description="Open an issue in the affected repository. Include the commit, command, expected behavior, and a minimal reproduction. Remove credentials from logs before sharing them." :to="`${links.sdk}/issues`" target="_blank" icon="i-lucide-bug" variant="soft" />
    </UContainer>
  </div>
</template>
