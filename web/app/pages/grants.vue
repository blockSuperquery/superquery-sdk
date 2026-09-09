<script setup lang="ts">
import { links } from '~/utils/project'
useSeoMeta({ title: 'Grants & ecosystem', description: 'Contribute to the SuperQuery ecosystem through Rust tooling, EVM examples, documentation, and integration work. See the current grants status.' })
const areas = [
  { title: 'Developer experience', description: 'Improve CLI diagnostics, project scaffolding, and examples that make the first run easier.', icon: 'i-lucide-terminal' },
  { title: 'Chain integrations', description: 'Help advance EVM support or propose an adapter through the shared chain integration interface.', icon: 'i-lucide-network' },
  { title: 'Data applications', description: 'Propose useful schemas for token transfers, protocol activity, and on-chain analytics.', icon: 'i-lucide-chart-no-axes-combined' },
  { title: 'Documentation & tests', description: 'Clarify a specification, add a focused regression fixture, or document a reproducible setup.', icon: 'i-lucide-book-open-check' }
]
</script>

<template>
  <div>
    <PageIntro eyebrow="Grants & ecosystem" title="Build something useful, together." description="SuperQuery is an open-source project. Help shape the tools, integrations, and examples that developers will use." />
    <UContainer class="space-y-12 pb-20">
      <UAlert title="No active grant application round" description="There is no published funding program or application process at this time. Contributions and project proposals are welcome through GitHub." color="info" variant="soft" icon="i-lucide-info" />
      <UPageGrid class="lg:grid-cols-2"><UPageCard v-for="area in areas" :key="area.title" v-bind="area" /></UPageGrid>
      <section class="grid gap-10 rounded-xl border border-default bg-elevated/40 p-6 sm:p-10 lg:grid-cols-2">
        <div><h2 class="text-3xl font-semibold text-highlighted">Start with a focused proposal.</h2><p class="page-copy mt-4">Open an issue in the relevant repository before a large change so maintainers can discuss scope and dependencies.</p></div>
        <div class="space-y-5">
          <ol class="list-decimal space-y-3 pl-5 text-muted"><li>Describe the user problem and intended outcome.</li><li>Point to the relevant repository and milestone.</li><li>Outline a small, testable first contribution.</li></ol>
          <UButton :to="`${links.sdk}/issues/new`" target="_blank" trailing-icon="i-lucide-arrow-up-right">Propose a contribution</UButton>
        </div>
      </section>
    </UContainer>
  </div>
</template>
