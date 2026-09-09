<script setup lang="ts">
import { links, quickstart, transferSchema } from '~/utils/project'

useSeoMeta({ title: 'Documentation', description: 'Get started with SuperQuery: validate an EVM project, generate Rust types, and understand the SDK, indexing node, and GraphQL service.' })
const sections = [
  { label: 'Quickstart', to: '#quickstart' }, { label: 'Project files', to: '#project-files' },
  { label: 'Rust handlers', to: '#handlers' }, { label: 'Query your data', to: '#query' }, { label: 'Specifications', to: '#specifications' }
]
const specs = [
  { title: 'Manifest v1', description: 'Networks, data sources, assets, and handler filters.', file: 'manifest-v1.md' },
  { title: 'Schema IR v1', description: 'Canonical entity definitions and deterministic schema hashes.', file: 'schema-v1.md' },
  { title: 'Mapping ABI v1', description: 'The boundary between a WASM mapping and its host.', file: 'mapping-abi-v1.md' },
  { title: 'Build artifact v1', description: 'The versioned bundle consumed by the indexing node.', file: 'build-artifact-v1.md' }
]
</script>

<template>
  <div>
    <PageIntro
      eyebrow="Documentation"
      title="From a schema to your first project."
      description="Start with the EVM template. Validate locally, inspect the generated code, and get to know the stack."
    />
    <UContainer class="pb-20">
      <UPage>
        <template #left>
          <UPageAside>
            <UNavigationMenu
              :items="sections"
              orientation="vertical"
              aria-label="Documentation sections"
            />
          </UPageAside>
        </template>
        <UPageBody class="mt-0 space-y-14">
          <section
            id="quickstart"
            class="space-y-5"
          >
            <h2 class="text-2xl font-semibold text-highlighted">
              01. Validate the starter
            </h2>
            <p class="page-copy">
              Install Git and a current stable Rust toolchain, including Cargo. These commands run from the SDK checkout and need no RPC endpoint or database. The first Cargo run downloads and compiles dependencies.
            </p>
            <CodeBlock :code="quickstart" />
            <UAlert
              title="What success looks like"
              description="Validation reports the project is valid. The codegen dry run lists the entity, schema metadata, and ERC-20 contract files it would generate."
              icon="i-lucide-check-circle"
              color="success"
              variant="soft"
            />
            <p class="page-copy">
              Remove <code>--dry-run</code> to write the generated files under <code>templates/evm/src/generated/</code>. The <code>init</code>, <code>build</code>, and <code>test</code> CLI commands are planned and currently return a milestone error.
            </p>
          </section>
          <section
            id="project-files"
            class="space-y-5"
          >
            <h2 class="text-2xl font-semibold text-highlighted">
              02. Understand the project files
            </h2>
            <UPageGrid class="lg:grid-cols-2">
              <UPageCard
                title="project.yaml"
                description="Selects the chain, start block, contract address, ABI assets, and event handlers."
                icon="i-lucide-file-cog"
                variant="soft"
              />
              <UPageCard
                title="schema.graphql"
                description="Defines the entities and scalar types used by generated code and the query API."
                icon="i-lucide-file-code"
                variant="soft"
              />
            </UPageGrid>
            <CodeBlock
              :code="transferSchema"
              filename="schema.graphql"
            />
            <p class="page-copy">
              The EVM template targets USDC Transfer events on Ethereum. Its ABI lives in <code>abis/ERC20.json</code>. Validation checks referenced assets and the full event signature before code generation.
            </p>
          </section>
          <section
            id="handlers"
            class="space-y-5"
          >
            <h2 class="text-2xl font-semibold text-highlighted">
              03. The mapping boundary
            </h2>
            <p class="page-copy">
              Mappings are Rust code designed to compile to WebAssembly using <code>wasm32-wasip1</code>. They use the SDK’s host API for effects. The node owns execution, storage, and chain access. Mapping builds and node host functions are still being implemented.
            </p>
            <CodeBlock
              code="rustup target add wasm32-wasip1
cargo run --locked -p superquery-cli -- doctor"
            />
            <UButton
              :to="`${links.sdk}/tree/main/crates/sdk`"
              target="_blank"
              variant="outline"
              color="neutral"
              trailing-icon="i-lucide-arrow-up-right"
            >
              Read the mapping SDK source
            </UButton>
          </section>
          <section
            id="query"
            class="space-y-5"
          >
            <h2 class="text-2xl font-semibold text-highlighted">
              04. Read indexed data
            </h2>
            <p class="page-copy">
              The separate query service reads a PostgreSQL schema and the project’s GraphQL schema. It supports collection and by-ID queries, filters, ordering, cursor pagination, and relations. It requires an existing database with matching entity tables.
            </p>
            <p class="page-copy">
              Use <code>POST /graphql</code> for queries, <code>GET /health</code> for process liveness, and <code>GET /ready</code> for database readiness. Historical queries and subscriptions depend on future node support.
            </p>
            <UButton
              :to="links.query"
              target="_blank"
              trailing-icon="i-lucide-arrow-up-right"
            >
              Query service setup
            </UButton>
          </section>
          <section
            id="specifications"
            class="space-y-5"
          >
            <h2 class="text-2xl font-semibold text-highlighted">
              The shared specifications
            </h2>
            <p class="page-copy">
              The SDK is the source of truth for the contracts used across all three repositories.
            </p>
            <UPageGrid class="lg:grid-cols-2">
              <UPageCard
                v-for="spec in specs"
                :key="spec.file"
                :title="spec.title"
                :description="spec.description"
                :to="`${links.sdk}/blob/main/docs/spec/${spec.file}`"
                target="_blank"
                icon="i-lucide-book-open"
              />
            </UPageGrid>
          </section>
        </UPageBody>
      </UPage>
    </UContainer>
  </div>
</template>
