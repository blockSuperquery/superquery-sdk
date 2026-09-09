<script setup lang="ts">
import { links } from '~/utils/project'

useSeoMeta({ title: 'Roadmap', description: 'Track SuperQuery development across the Rust SDK, indexing node, and GraphQL query service. See what is available, next, and planned.' })
const tracks = [
  { title: 'Developer tooling', repo: links.sdk, milestones: [
    { title: 'Foundations available', state: 'Available', description: 'Shared types, manifest validation, schema IR, deterministic codegen, EVM ABI bindings, and toolchain checks.' },
    { title: 'Complete the project lifecycle', state: 'Next', description: 'Template scaffolding, WASM builds, versioned artifact bundles, and mapping test commands.' },
    { title: 'Expand integrations', state: 'Planned', description: 'Additional chain integrations and complete examples that validate, generate, and build in CI.' }
  ] },
  { title: 'Indexing engine', repo: links.node, milestones: [
    { title: 'Core logic in place', state: 'Available', description: 'Configuration, storage foundations, fetch planning, ordered commit logic, reorg detection, and EVM filters.' },
    { title: 'Connect the pipeline', state: 'Next', description: 'Live EVM RPC ingestion, scheduler loops, worker execution, and Wasmtime host functions.' },
    { title: 'Extend indexed state', state: 'Planned', description: 'Rewind execution, dynamic data sources, and the node support needed for historical reads and subscriptions.' }
  ] },
  { title: 'Query service', repo: links.query, milestones: [
    { title: 'Typed reads available', state: 'Available', description: 'Dynamic GraphQL schemas, filtering, ordering, cursor pagination, relations, query limits, and health endpoints.' },
    { title: 'Batch relation reads', state: 'Next', description: 'DataLoader batching for related entities.' },
    { title: 'History and subscriptions', state: 'Planned', description: 'Historical queries need entity versioning. Subscriptions need commit notifications from the node.' }
  ] }
]
</script>

<template>
  <div>
    <PageIntro
      eyebrow="Built in public"
      title="A clear view of what comes next."
      description="Progress across the three repositories, with working foundations and unfinished milestones made explicit."
    />
    <UContainer class="space-y-12 pb-20">
      <UAlert
        title="A development roadmap, without release-date promises"
        description="This snapshot reflects the repository documentation reviewed on 9 September 2026. Follow each repository for the latest implementation details."
        icon="i-lucide-git-branch"
        color="info"
        variant="soft"
      />
      <section
        v-for="track in tracks"
        :key="track.title"
        class="space-y-5"
      >
        <div class="flex flex-wrap items-center justify-between gap-3">
          <h2 class="text-2xl font-semibold text-highlighted">
            {{ track.title }}
          </h2>
          <UButton
            :to="track.repo"
            target="_blank"
            color="neutral"
            variant="link"
            trailing-icon="i-lucide-arrow-up-right"
          >
            Follow development
          </UButton>
        </div>
        <UPageGrid>
          <UPageCard
            v-for="milestone in track.milestones"
            :key="milestone.title"
            :title="milestone.title"
            :description="milestone.description"
            variant="soft"
          >
            <template #leading>
              <UBadge
                :color="milestone.state === 'Available' ? 'success' : milestone.state === 'Next' ? 'info' : 'neutral'"
                variant="subtle"
              >
                {{ milestone.state }}
              </UBadge>
            </template>
          </UPageCard>
        </UPageGrid>
      </section>
      <UPageCTA
        title="Help move a milestone forward."
        description="Start with a repository, read its implementation plan, and propose a focused contribution."
        :links="[{ label: 'Find a place to contribute', to: '/grants', trailingIcon: 'i-lucide-arrow-right' }]"
        variant="soft"
        :ui="{ title: 'text-3xl', container: 'py-10 sm:py-12' }"
      />
    </UContainer>
  </div>
</template>
