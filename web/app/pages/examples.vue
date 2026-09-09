<script setup lang="ts">
import { links, quickstart, transferSchema, transferQuery } from '~/utils/project'
useSeoMeta({ title: 'Examples', description: 'Explore the SuperQuery EVM starter for ERC-20 transfers with real validation commands, GraphQL entity definitions, and a query example.' })
const tabs = [
  { label: 'Quickstart', value: 'quickstart', code: quickstart, filename: 'terminal' },
  { label: 'Schema', value: 'schema', code: transferSchema, filename: 'schema.graphql' },
  { label: 'GraphQL query', value: 'query', code: transferQuery, filename: 'query.graphql' }
]
</script>

<template>
  <div>
    <PageIntro eyebrow="Examples" title="Start with something concrete." description="A real EVM starter to explore the project format, validate a manifest, and generate your first Rust types." />
    <UContainer class="space-y-12 pb-20">
      <UPageCard variant="soft" :ui="{ container: 'p-6 sm:p-10' }">
        <div class="flex flex-wrap items-center gap-3"><UBadge variant="subtle">EVM</UBadge><UBadge color="neutral" variant="subtle">Reference template</UBadge></div>
        <h2 class="mt-3 text-3xl font-semibold text-highlighted">ERC-20 transfers</h2>
        <p class="page-copy max-w-2xl">Follow USDC Transfer events on Ethereum. The template includes a project manifest, an ERC-20 ABI, and Transfer and Holder entities. Validation and code generation work today; end-to-end indexing awaits the build and runtime milestones.</p>
        <div class="mt-4"><UButton :to="`${links.sdk}/tree/main/templates/evm`" target="_blank" trailing-icon="i-lucide-arrow-up-right">Open the template</UButton></div>
      </UPageCard>
      <UTabs :items="tabs" :unmount-on-hide="false" aria-label="Example files">
        <template #content="{ item }"><CodeBlock :code="item.code" :filename="item.filename" /></template>
      </UTabs>
      <UAlert title="About the query example" description="Run it against a configured superquery-query instance with a populated database. The SDK quickstart validates and generates code; it does not start an indexer or a GraphQL endpoint." color="info" variant="soft" icon="i-lucide-info" />
      <section class="space-y-6">
        <h2 class="text-2xl font-semibold text-highlighted">Help grow the examples.</h2>
        <UPageGrid class="lg:grid-cols-2">
          <UPageCard title="Uniswap V3" description="A planned example for swap events, multiple entities, and relations. Follow its progress in the examples directory." :to="`${links.sdk}/tree/main/examples`" target="_blank" icon="i-lucide-arrow-left-right"><UBadge color="neutral" variant="soft">Planned</UBadge></UPageCard>
          <UPageCard title="Have another use case?" description="Propose an example with a chain, contract, event signature, and the entities you want to query." :to="`${links.sdk}/issues/new`" target="_blank" icon="i-lucide-lightbulb"><span class="text-sm text-primary">Propose an example ↗</span></UPageCard>
        </UPageGrid>
      </section>
    </UContainer>
  </div>
</template>
