<script setup lang="ts">
import { links, repositories, transferSchema, transferQuery } from '~/utils/project'

useSeoMeta({
  title: 'Blockchain data, built with Rust',
  description: 'Define your schema, generate Rust types, and explore a modular blockchain indexing stack. Open-source SDK, indexing engine, and GraphQL query service.',
  ogTitle: 'SuperQuery | Blockchain data, built with Rust'
})
useSchemaOrg([defineSoftwareApp({ name: 'SuperQuery', applicationCategory: 'DeveloperApplication', operatingSystem: 'Linux, macOS, Windows', description: 'An open-source Rust toolkit for blockchain indexing.', isAccessibleForFree: true })])

const files = [
  { label: 'schema.graphql', icon: 'i-lucide-file-code', value: 'schema', code: transferSchema },
  { label: 'query.graphql', icon: 'i-lucide-search-code', value: 'query', code: transferQuery }
]
const features = [
  { icon: 'i-lucide-braces', title: 'A schema you can build on.', description: 'Define entities in GraphQL. Generate deterministic Rust types, metadata, and EVM contract bindings from the same project.' },
  { icon: 'i-lucide-shield-check', title: 'Correctness at the boundaries.', description: 'Catch invalid manifests and event filters before execution. Shared, versioned contracts keep the SDK, node, and query service aligned.' },
  { icon: 'i-lucide-git-branch', title: 'A stack you can inspect.', description: 'Follow the code from developer tooling to indexed state to read APIs. Each repository has a clear role and a public implementation plan.' }
]
const questions = [
  { label: 'What can I use today?', content: 'You can validate an EVM project, generate Rust entity types and contract bindings, and inspect your toolchain with the SDK. The query service supports GraphQL reads over a prepared PostgreSQL schema. The complete indexing pipeline is still in development.' },
  { label: 'How does SuperQuery relate to SubQuery?', content: 'SuperQuery follows SubQuery’s developer model and architecture through an independent Rust implementation. GraphQL schemas and manifests will feel familiar, but this is not a promise of drop-in compatibility. TypeScript mappings need to be rewritten in Rust.' },
  { label: 'Which chains are supported?', content: 'EVM is the reference integration for manifest validation, ABI parsing, and generated bindings. Stellar and Solana have reserved SDK crates. Their integrations and additional chain support are planned.' },
  { label: 'Is there a hosted service?', content: 'There is no managed hosting service available from this site. Explore the self-hosted repositories, follow the roadmap, or contribute to the platform as it develops.' }
]
</script>

<template>
  <div>
    <section class="relative overflow-hidden border-b border-default">
      <div class="dot-grid pointer-events-none absolute inset-0 opacity-45 [mask-image:linear-gradient(to_bottom,black,transparent)]" aria-hidden="true" />
      <UPageHero orientation="horizontal" description="Turn chain events into data you can build with. A Rust toolkit for defining, indexing, and querying blockchain data." :ui="{ container: 'relative py-16 sm:py-24 lg:py-28 gap-12 lg:gap-10', title: 'text-5xl sm:text-6xl lg:text-[4.5rem] leading-[1.04] tracking-[-0.05em]', description: 'text-base sm:text-lg leading-8 max-w-lg', links: 'gap-3' }">
        <template #headline>
          <NuxtLink to="/roadmap" class="inline-flex items-center gap-2 rounded-full border border-default bg-default px-3 py-1.5 text-xs text-muted">
            <span class="size-1.5 rounded-full bg-blue-500" />
            Built in public. Built with Rust.
            <UIcon name="i-lucide-arrow-right" class="size-3" />
          </NuxtLink>
        </template>
        <template #title>Blockchain data.<br><span class="text-primary">Supercharged</span><span class="text-red-500">.</span></template>
        <template #links>
          <UButton to="/docs" size="xl" trailing-icon="i-lucide-arrow-up-right">Start building</UButton>
          <UButton :to="links.sdk" target="_blank" size="xl" icon="i-lucide-github" color="neutral" variant="outline">Explore the code</UButton>
        </template>
        <template #footer><p class="mt-6 font-mono text-[11px] text-muted">OPEN SOURCE <span class="mx-2 text-dimmed">/</span> EVM FIRST <span class="mx-2 text-dimmed">/</span> PRE-ALPHA</p></template>
        <div class="relative min-w-0">
          <div class="absolute -inset-4 rounded-3xl bg-blue-500/5 sm:-inset-6" aria-hidden="true" />
          <div class="relative overflow-hidden rounded-xl border border-slate-700 bg-ink shadow-2xl shadow-blue-950/15">
            <div class="flex items-center justify-between border-b border-white/10 px-5 py-4">
              <div class="flex gap-1.5" aria-hidden="true"><span class="size-2 rounded-full bg-red-400" /><span class="size-2 rounded-full bg-amber-400" /><span class="size-2 rounded-full bg-emerald-400" /></div>
              <span class="font-mono text-[10px] tracking-wider text-slate-400">ERC-20 TRANSFERS</span>
              <UIcon name="i-lucide-code-2" class="size-4 text-slate-400" />
            </div>
            <div class="px-5 pt-5 sm:px-7">
              <p class="mb-5 font-mono text-xs text-slate-400">schema.graphql</p>
              <pre class="overflow-x-auto font-mono text-[13px] leading-7 text-slate-200"><code><span class="text-blue-400">type</span> <span class="text-white">Transfer</span> <span class="text-orange-300">@entity</span> {
  id: <span class="text-blue-300">ID!</span>
  from: <span class="text-blue-300">String!</span>
  to: <span class="text-blue-300">String!</span>
  value: <span class="text-blue-300">BigInt!</span>
}</code></pre>
              <p class="mb-5 mt-4 text-[11px] text-slate-400">An excerpt from the EVM starter schema.</p>
            </div>
            <div class="flex items-center gap-3 border-t border-white/10 bg-white/[0.03] px-5 py-4 sm:px-7">
              <span class="flex size-8 items-center justify-center rounded-lg bg-blue-400/10"><UIcon name="i-lucide-arrow-down-to-line" class="size-4 text-blue-400" /></span>
              <div><p class="font-mono text-xs text-slate-200">schema → Rust types</p><p class="mt-1 text-[11px] text-slate-400">One project. A shared contract.</p></div>
            </div>
          </div>
          <div class="relative mx-4 -mt-0.5 flex items-center justify-between rounded-b-xl border border-t-0 border-default bg-default px-4 py-3 text-[10px] text-muted sm:mx-6 sm:text-xs">
            <span class="flex items-center gap-1.5"><UIcon name="i-lucide-file-code" class="size-3.5 text-primary" />Define</span><UIcon name="i-lucide-arrow-right" class="size-3" />
            <span class="flex items-center gap-1.5"><UIcon name="i-lucide-box" class="size-3.5 text-primary" />Generate</span><UIcon name="i-lucide-arrow-right" class="size-3" />
            <span class="flex items-center gap-1.5"><UIcon name="i-lucide-search" class="size-3.5 text-primary" />Query</span>
          </div>
        </div>
      </UPageHero>
    </section>

    <UContainer class="flex flex-wrap items-center justify-between gap-x-8 gap-y-4 py-7">
      <p class="eyebrow text-muted">Familiar primitives. A new foundation.</p>
      <div class="flex flex-wrap gap-6 text-sm font-medium text-muted sm:gap-10">
        <span class="flex items-center gap-2"><UIcon name="i-lucide-settings-2" class="size-4" />Rust</span>
        <span class="flex items-center gap-2"><UIcon name="i-lucide-box" class="size-4" />WebAssembly</span>
        <span class="flex items-center gap-2"><UIcon name="i-lucide-database" class="size-4" />PostgreSQL</span>
        <span class="flex items-center gap-2"><UIcon name="i-lucide-network" class="size-4" />GraphQL</span>
      </div>
    </UContainer>

    <UPageSection title="Three parts. One clear path." description="Developer tooling, indexed state, and read APIs. Separate responsibilities, connected by shared specifications." :ui="{ root: 'border-y border-default bg-elevated/40', container: 'py-16 sm:py-24', title: 'text-3xl sm:text-4xl', description: 'text-base max-w-2xl mx-auto' }">
      <template #headline><span class="eyebrow text-primary">The architecture</span></template>
      <UPageGrid>
        <UPageCard v-for="repo in repositories" :key="repo.title" :title="repo.title" :description="repo.description" :icon="repo.icon" :to="repo.to" :target="repo.target" :ui="{ root: 'bg-default', title: 'font-display text-xl', container: 'p-7 sm:p-8' }">
          <template #leading><span class="eyebrow text-muted">{{ repo.label }}</span></template>
          <template #footer><span class="inline-flex items-center gap-2 text-sm font-medium text-primary">Explore <UIcon name="i-lucide-arrow-up-right" class="size-4" /></span></template>
        </UPageCard>
      </UPageGrid>
      <p class="text-center text-sm text-muted">The full indexing pipeline is in development. <NuxtLink to="/status" class="font-medium text-primary underline underline-offset-4">See what works today</NuxtLink></p>
    </UPageSection>

    <UPageSection title="Start with your data. Keep control of the stack." description="A familiar GraphQL schema is the starting point. The SDK generates Rust types; the query service exposes indexed entities through a typed API." orientation="horizontal" :ui="{ container: 'py-16 sm:py-24 gap-12', title: 'text-3xl sm:text-4xl', description: 'text-base leading-7' }">
      <template #headline><span class="eyebrow text-primary">Designed for developers</span></template>
      <template #links><UButton to="/examples" variant="outline" color="neutral" trailing-icon="i-lucide-arrow-right">Explore the EVM starter</UButton></template>
      <div class="min-w-0">
        <UTabs :items="files" :unmount-on-hide="false" aria-label="Schema and query examples" :ui="{ trigger: 'font-mono text-xs' }">
          <template #content="{ item }"><CodeBlock :code="item.code" :filename="item.label" /></template>
        </UTabs>
        <p class="mt-4 text-xs leading-6 text-muted">Query example requires the query service and a populated PostgreSQL schema.</p>
      </div>
    </UPageSection>

    <UPageSection title="Less guesswork. More explicit contracts." :features="features" :ui="{ root: 'border-y border-default bg-elevated/40', container: 'py-16 sm:py-24', title: 'text-3xl sm:text-4xl' }">
      <template #headline><span class="eyebrow text-primary">The engineering choices</span></template>
    </UPageSection>

    <UPageSection title="Build with us, from the beginning." description="The foundations are in place. The next step is bringing the pipeline together." :ui="{ container: 'py-16 sm:py-24', title: 'text-3xl sm:text-4xl', description: 'text-base' }">
      <div class="grid gap-6 md:grid-cols-3">
        <UPageCard title="Available today" description="Manifest validation, schema parsing, deterministic codegen, EVM bindings, and GraphQL reads." icon="i-lucide-check" variant="soft" />
        <UPageCard title="Up next" description="Project scaffolding, WASM build artifacts, live EVM ingestion, and mapping execution." icon="i-lucide-hammer" variant="soft" />
        <UPageCard title="Further ahead" description="Historical queries, subscriptions, additional chain integrations, and managed tooling." icon="i-lucide-telescope" variant="soft" />
      </div>
      <div class="text-center"><UButton to="/roadmap" color="neutral" variant="outline" trailing-icon="i-lucide-arrow-right">Follow the roadmap</UButton></div>
    </UPageSection>

    <UPageSection title="A few things to know." :ui="{ root: 'border-y border-default', container: 'py-16 sm:py-20 max-w-3xl', title: 'text-3xl sm:text-4xl' }">
      <UAccordion :items="questions" :ui="{ trigger: 'text-base py-5', body: 'text-muted leading-7 pb-5' }" />
    </UPageSection>
    <BuildCta />
  </div>
</template>
