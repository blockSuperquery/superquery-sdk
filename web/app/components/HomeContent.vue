<script setup lang="ts">
import { repositories, transferSchema, transferQuery } from '~/utils/project'

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
    <UPageSection
      title="Three parts. One clear path."
      description="Developer tooling, indexed state, and read APIs. Separate responsibilities, connected by shared specifications."
      :ui="{ root: 'border-y border-default bg-elevated/40', container: 'py-16 sm:py-24', title: 'text-3xl sm:text-4xl', description: 'text-base max-w-2xl mx-auto' }"
    >
      <template #headline>
        <span class="eyebrow text-primary">The architecture</span>
      </template>
      <UPageGrid>
        <UPageCard
          v-for="repo in repositories"
          :key="repo.title"
          :title="repo.title"
          :description="repo.description"
          :icon="repo.icon"
          :to="repo.to"
          :target="repo.target"
          :ui="{ root: 'bg-default', title: 'font-display text-xl', container: 'p-7 sm:p-8' }"
        >
          <template #leading>
            <span class="eyebrow text-muted">{{ repo.label }}</span>
          </template>
          <template #footer>
            <span class="inline-flex items-center gap-2 text-sm font-medium text-primary">Explore <UIcon
              name="i-lucide-arrow-up-right"
              class="size-4"
            /></span>
          </template>
        </UPageCard>
      </UPageGrid>
      <p class="text-center text-sm text-muted">
        The full indexing pipeline is in development. <NuxtLink
          to="/status"
          class="font-medium text-primary underline underline-offset-4"
        >See what works today</NuxtLink>
      </p>
    </UPageSection>

    <UPageSection
      title="Start with your data. Keep control of the stack."
      description="A familiar GraphQL schema is the starting point. The SDK generates Rust types; the query service exposes indexed entities through a typed API."
      orientation="horizontal"
      :ui="{ container: 'py-16 sm:py-24 gap-12', title: 'text-3xl sm:text-4xl', description: 'text-base leading-7' }"
    >
      <template #headline>
        <span class="eyebrow text-primary">Designed for developers</span>
      </template>
      <template #links>
        <UButton
          to="/examples"
          variant="outline"
          color="neutral"
          trailing-icon="i-lucide-arrow-right"
        >
          Explore the EVM starter
        </UButton>
      </template>
      <div class="min-w-0">
        <UTabs
          default-value="schema"
          :items="files"
          :unmount-on-hide="false"
          aria-label="Schema and query examples"
          :ui="{ trigger: 'font-mono text-xs' }"
        >
          <template #content="{ item }">
            <CodeBlock
              :code="item.code"
              :filename="item.label"
            />
          </template>
        </UTabs>
        <p class="mt-4 text-xs leading-6 text-muted">
          Query example requires the query service and a populated PostgreSQL schema.
        </p>
      </div>
    </UPageSection>

    <UPageSection
      title="Less guesswork. More explicit contracts."
      :features="features"
      :ui="{ root: 'border-y border-default bg-elevated/40', container: 'py-16 sm:py-24', title: 'text-3xl sm:text-4xl' }"
    >
      <template #headline>
        <span class="eyebrow text-primary">The engineering choices</span>
      </template>
    </UPageSection>

    <UPageSection
      title="Build with us, from the beginning."
      description="The foundations are in place. The next step is bringing the pipeline together."
      :ui="{ container: 'py-16 sm:py-24', title: 'text-3xl sm:text-4xl', description: 'text-base' }"
    >
      <div class="grid gap-6 md:grid-cols-3">
        <UPageCard
          title="Available today"
          description="Manifest validation, schema parsing, deterministic codegen, EVM bindings, and GraphQL reads."
          icon="i-lucide-check"
          variant="soft"
        />
        <UPageCard
          title="Up next"
          description="Project scaffolding, WASM build artifacts, live EVM ingestion, and mapping execution."
          icon="i-lucide-hammer"
          variant="soft"
        />
        <UPageCard
          title="Further ahead"
          description="Historical queries, subscriptions, additional chain integrations, and managed tooling."
          icon="i-lucide-telescope"
          variant="soft"
        />
      </div>
      <div class="text-center">
        <UButton
          to="/roadmap"
          color="neutral"
          variant="outline"
          trailing-icon="i-lucide-arrow-right"
        >
          Follow the roadmap
        </UButton>
      </div>
    </UPageSection>

    <UPageSection
      title="A few things to know."
      :ui="{ root: 'border-y border-default', container: 'py-16 sm:py-20 max-w-3xl', title: 'text-3xl sm:text-4xl' }"
    >
      <UAccordion
        :items="questions"
        :ui="{ trigger: 'text-base py-5', body: 'text-muted leading-7 pb-5' }"
      />
    </UPageSection>
    <BuildCta />
  </div>
</template>
