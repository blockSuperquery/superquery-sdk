<script setup lang="ts">
import { links } from '~/utils/project'

useSeoMeta({ title: 'Changelog', description: 'SuperQuery development notes and links to the SDK, node, and query service commit histories and releases.' })
const changes = [
  { title: 'Developer workspace established', description: 'The SDK workspace brings shared types, manifest validation, schema parsing, deterministic code generation, EVM bindings, and the CLI together. The web app lives separately in web/.' },
  { title: 'Cross-repository contracts documented', description: 'Manifest v1, schema IR v1, mapping ABI v1, and build artifact v1 define the interfaces consumed by the node and query service.' },
  { title: 'An EVM starting point', description: 'The ERC-20 transfer template can be validated and used for a codegen dry run. Full example builds await the build milestone.' }
]
</script>

<template>
  <div>
    <PageIntro
      eyebrow="Changelog"
      title="Follow the work as it lands."
      description="Development notes from the SDK workspace, with direct links to the source history across the platform."
    />
    <UContainer class="max-w-4xl space-y-10 pb-20">
      <UAlert
        title="Development snapshot: 9 September 2026"
        description="These are workspace notes, not a tagged production release. Use the release pages and commit histories for the latest changes."
        icon="i-lucide-git-commit-horizontal"
        color="info"
        variant="soft"
      />
      <section class="space-y-6">
        <div class="flex flex-wrap items-center gap-3">
          <UBadge variant="subtle">
            v0.1.0 workspace
          </UBadge><span class="text-sm text-muted">SDK foundations</span>
        </div>
        <UPageCard
          v-for="change in changes"
          :key="change.title"
          v-bind="change"
          variant="soft"
        />
        <UButton
          :to="`${links.sdk}/commits/main`"
          target="_blank"
          color="neutral"
          variant="outline"
          trailing-icon="i-lucide-arrow-up-right"
        >
          Read the SDK commit history
        </UButton>
      </section>
      <USeparator />
      <section class="space-y-5">
        <h2 class="text-2xl font-semibold text-highlighted">
          Track each part of the platform.
        </h2>
        <UPageGrid>
          <UPageCard
            title="SDK releases"
            description="Developer tooling and shared contracts."
            :to="`${links.sdk}/releases`"
            target="_blank"
            icon="i-lucide-terminal"
          />
          <UPageCard
            title="Node history"
            description="Ingestion, execution, and indexed state."
            :to="`${links.node}/commits/main`"
            target="_blank"
            icon="i-lucide-blocks"
          />
          <UPageCard
            title="Query history"
            description="GraphQL, filtering, and data access."
            :to="`${links.query}/commits/main`"
            target="_blank"
            icon="i-lucide-database"
          />
        </UPageGrid>
      </section>
    </UContainer>
  </div>
</template>
