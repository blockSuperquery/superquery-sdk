<script setup lang="ts">
import { links, repositories } from '~/utils/project'
useSeoMeta({ title: 'Developer portal', description: 'Your entry point to SuperQuery documentation, repositories, examples, and self-hosted tooling.', robots: 'noindex, follow' })
</script>

<template>
  <div>
    <PageIntro eyebrow="Developer portal" title="Everything you need to find your way." description="Jump into the documentation, inspect the code, or follow the next milestone." />
    <UContainer class="space-y-10 pb-20">
      <UAlert title="Managed deployments are not available yet" description="This portal is a resource hub. It does not connect a wallet, deploy indexers, or manage a hosted account." color="info" variant="soft" icon="i-lucide-info" />
      <UPageGrid><UPageCard v-for="repo in repositories" :key="repo.title" :title="repo.title" :description="repo.description" :to="repo.to" :target="repo.target" :icon="repo.icon" /></UPageGrid>
      <UPageGrid class="lg:grid-cols-2">
        <UPageCard title="Run the quickstart" description="Validate the EVM template and preview generated Rust files locally." to="/docs" icon="i-lucide-book-open" variant="soft" />
        <UPageCard title="Browse the starter" description="Explore the schema, event ABI, and manifest for ERC-20 transfers." to="/examples" icon="i-lucide-code-2" variant="soft" />
        <UPageCard title="Follow a milestone" description="See what works today and where contributions can help." to="/roadmap" icon="i-lucide-route" variant="soft" />
        <UPageCard title="Discuss a use case" description="Describe what you want to index and how you plan to query it." :to="`${links.sdk}/issues/new`" target="_blank" icon="i-lucide-messages-square" variant="soft" />
      </UPageGrid>
    </UContainer>
  </div>
</template>
