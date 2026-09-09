<script setup lang="ts">
import { links, commands } from '~/utils/project'

useSeoMeta({ title: 'Rust SDK & CLI', description: 'Explore the SuperQuery Rust SDK, CLI commands, schema code generation, EVM integration, and versioned mapping contracts.' })
const crates = [
  { title: 'Types & manifest', description: 'Scalar values, entity IDs, block pointers, and a validated project manifest.', icon: 'i-lucide-file-check' },
  { title: 'Schema & codegen', description: 'GraphQL parsing, canonical IR, deterministic hashes, and generated Rust.', icon: 'i-lucide-braces' },
  { title: 'Mapping SDK & macros', description: 'The guest API, handler attributes, entity derives, and versioned host calls.', icon: 'i-lucide-box' },
  { title: 'Chain integrations', description: 'A generic integration seam with EVM ABI parsing and binding generation.', icon: 'i-lucide-network' }
]
</script>

<template>
  <div>
    <PageIntro
      eyebrow="The developer toolkit"
      title="Your schema. Your types. Your Rust."
      description="The SDK owns the developer contract: project manifests, schema definitions, generated code, and the mapping interface."
    >
      <UButton
        to="/docs"
        size="lg"
        trailing-icon="i-lucide-arrow-right"
      >
        Get started locally
      </UButton>
      <UButton
        :to="links.sdk"
        target="_blank"
        size="lg"
        variant="outline"
        color="neutral"
        icon="i-lucide-github"
      >
        View source
      </UButton>
    </PageIntro>
    <UContainer class="space-y-16 pb-20">
      <UAlert
        title="v0.1.0 development workspace"
        description="Validation, code generation, and toolchain checks work today. Build and scaffolding commands are still in development."
        color="info"
        variant="soft"
        icon="i-lucide-info"
      />
      <section class="space-y-6">
        <h2 class="text-2xl font-semibold text-highlighted">
          A small CLI with explicit responsibilities.
        </h2>
        <UTable
          :data="commands"
          :columns="[{ accessorKey: 'command', header: 'Command' }, { accessorKey: 'state', header: 'Status' }, { accessorKey: 'description', header: 'Purpose' }]"
          :ui="{ td: 'whitespace-normal min-w-28', th: 'text-highlighted' }"
        >
          <template #command-cell="{ row }">
            <code class="font-mono text-primary">{{ row.original.command }}</code>
          </template>
          <template #state-cell="{ row }">
            <UBadge
              :color="row.original.state === 'Available' ? 'success' : 'neutral'"
              variant="subtle"
            >
              {{ row.original.state }}
            </UBadge>
          </template>
        </UTable>
      </section>
      <section class="space-y-6">
        <h2 class="text-2xl font-semibold text-highlighted">
          Built as a Rust workspace.
        </h2>
        <UPageGrid class="lg:grid-cols-2">
          <UPageCard
            v-for="crate in crates"
            :key="crate.title"
            v-bind="crate"
          />
        </UPageGrid>
      </section>
      <section class="grid items-center gap-8 lg:grid-cols-2">
        <div class="space-y-4">
          <h2 class="text-2xl font-semibold text-highlighted">
            Inspect before you generate.
          </h2><p class="page-copy">
            Preview the generated file paths without changing your project. When you are ready, remove the dry-run flag to write entities, metadata, and contract bindings.
          </p>
        </div>
        <CodeBlock
          code="cargo run --locked -p superquery-cli -- codegen \
  --dry-run \
  --manifest templates/evm/project.yaml"
        />
      </section>
      <UPageCard
        title="SubQuery-inspired. Rust by design."
        description="SuperQuery follows the familiar manifest and GraphQL entity model. It is an independent Rust implementation: check supported schema features and rewrite TypeScript handlers for the Rust mapping interface."
        icon="i-lucide-git-compare-arrows"
        variant="soft"
        to="/docs#specifications"
      />
    </UContainer>
  </div>
</template>
