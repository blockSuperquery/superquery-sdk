<script setup lang="ts">
useSeoMeta({
  description: 'A high-integrity indexing framework designed for the most demanding blockchain data pipelines. Performance of Rust, familiarity of SubQuery.',
  keywords: 'blockchain indexing, rust, subquery, graphql, ethereum indexer, substrate indexer, blockchain infrastructure, web3 data',
  ogImage: '/superquery-mark.svg',
  ogImageAlt: 'SuperQuery — Rust-native blockchain indexing framework'
})

useSchemaOrg([
  defineSoftwareApp({
    name: 'SuperQuery',
    description: 'High-integrity blockchain indexing framework — Rust-native, SubQuery-compatible.',
    applicationCategory: 'DeveloperApplication',
    operatingSystem: 'Linux, macOS, Windows',
    offers: { '@type': 'Offer', price: '0' }
  }),
  defineWebPage({ '@type': 'WebPage' })
])

const workflowSteps = [
  { n: 1, title: 'Define Schema', copy: 'Standard GraphQL schema definition for your entities.' },
  { n: 2, title: 'Chain Ingest', copy: 'SuperQuery reads raw blocks via optimized Rust fetchers.' },
  { n: 3, title: 'Rust Mappings', copy: 'Blazing fast data transformation compiled to native code.' },
  { n: 4, title: 'Store & Index', copy: 'ACID-compliant storage in PostgreSQL with automatic indexing.' },
  { n: 5, title: 'Query Engine', copy: 'Expose your data via high-concurrency GraphQL endpoints.' }
]

const activeStep = ref(1)

const primaryFeatures = [
  { icon: 'i-lucide-zap', title: 'Native Performance', copy: 'Compile directly to machine code for indexing speeds 5× faster than TypeScript alternatives.' },
  { icon: 'i-lucide-shield-check', title: 'Type Safety', copy: 'End-to-end type safety with Rust ensures your data transformations never fail in production.' },
  { icon: 'i-lucide-layers', title: 'Modular Core', copy: 'Pluggable fetchers and stores. Use PostgreSQL today, migrate to ClickHouse tomorrow.' }
]

const capabilities = [
  'SubQuery Manifest Compatible',
  'Real-time Block Subscription',
  'Deterministic Indexing',
  'Automated API Scaffolding',
  'Multi-chain Aggregate Support',
  'Prometheus Monitoring Export'
]

const currentScope = [
  'Rust SDK for EVM Log handling',
  'PostgreSQL Dynamic Store plugin',
  'CLI code generation tools',
  'Substrate extrinsic support'
]

const plannedScope = [
  'Distributed Ingestion Engine',
  'WASM execution sandbox',
  'GraphQL dynamic filtering v2',
  'Managed Cloud Interface'
]
</script>

<template>
  <div>
    <!-- HERO -->
    <section class="relative overflow-hidden pt-[100px] pb-16">
      <div class="isometric-grid absolute inset-0 -z-10" />
      <div class="container-content text-center">
        <UBadge
          color="primary"
          variant="soft"
          class="mb-8 rounded-full border border-primary-600/20 bg-soft-blue px-4 py-1 font-mono text-[11px] font-semibold uppercase tracking-wider text-primary-600"
        >
          Rust-native · SubQuery compatible
        </UBadge>

        <h1 class="mx-auto mb-4 max-w-4xl font-display text-5xl font-bold leading-[1.05] tracking-tight text-heading md:text-6xl lg:text-[64px]">
          Query at super speed<span class="text-signal-red">.</span>
        </h1>

        <p class="mx-auto mb-8 max-w-2xl text-lg leading-relaxed text-fg-muted">
          A high-integrity indexing framework designed for the most demanding blockchain data pipelines.
          Performance of Rust, familiarity of SubQuery.
        </p>

        <div class="mb-16 flex flex-col justify-center gap-4 md:flex-row">
          <UButton
            to="/sdk"
            size="lg"
            trailing-icon="i-lucide-arrow-right"
            class="bg-primary-600 px-8 py-3 font-mono text-xs uppercase tracking-wider text-white hover:bg-primary-700"
          >
            Start building
          </UButton>
          <UButton
            to="/docs"
            variant="outline"
            color="neutral"
            size="lg"
            class="border-subtle bg-transparent px-8 py-3 font-mono text-xs uppercase tracking-wider text-heading"
          >
            View documentation
          </UButton>
        </div>

        <!-- Illustration: bordered terminal-style card -->
        <div class="mx-auto max-w-5xl border border-subtle bg-surface p-2">
          <div class="relative aspect-video w-full overflow-hidden bg-surface-muted">
            <div class="isometric-grid absolute inset-0 opacity-60" />
            <div class="absolute inset-0 flex items-center justify-center p-8">
              <div class="w-full max-w-2xl overflow-hidden rounded border border-neutral-800 bg-deep-ink text-left shadow-2xl">
                <div class="flex items-center gap-2 border-b border-white/10 px-4 py-2">
                  <span class="h-2 w-2 rounded-full bg-signal-red" />
                  <span class="h-2 w-2 rounded-full bg-rust-orange" />
                  <span class="h-2 w-2 rounded-full bg-primary-500" />
                  <span class="ml-3 font-mono text-[11px] uppercase tracking-wider text-neutral-400">
                    superquery run --chain ethereum
                  </span>
                </div>
                <div class="code-syntax-rust p-5 font-mono text-[13px] leading-relaxed text-neutral-200">
                  <div><span class="comment">$ superquery build --release</span></div>
                  <div class="text-emerald-400">
                    ✓ Compiled 4 handlers in <span class="text-white">1.2s</span>
                  </div>
                  <div class="text-emerald-400">
                    ✓ Indexed block <span class="text-white">18,201,442</span> · 12ms
                  </div>
                  <div class="text-emerald-400">
                    ✓ GraphQL endpoint ready · <span class="text-primary-400">localhost:3000/graphql</span>
                  </div>
                  <div class="mt-2 text-neutral-500">
                    <span class="comment"># tail latency p99: 42ms · throughput 8.4k blocks/s</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- SOCIAL PROOF -->
    <section class="border-y border-subtle bg-surface py-8">
      <div class="container-content flex flex-col items-center justify-between gap-4 md:flex-row">
        <div class="flex min-w-[320px] items-center gap-4 border border-subtle bg-surface-muted p-4">
          <div class="flex h-10 w-10 items-center justify-center rounded bg-primary-600">
            <UIcon name="i-lucide-terminal" class="text-white" />
          </div>
          <div>
            <p class="eyebrow text-fg-muted">
              GitHub Repository
            </p>
            <p class="font-display text-lg font-semibold leading-none text-heading">
              SuperQuery/superquery-sdk
            </p>
          </div>
          <div class="ml-auto flex flex-col items-end gap-1">
            <div class="flex items-center gap-1 text-primary-600">
              <UIcon name="i-lucide-star" class="text-sm" />
              <span class="font-mono text-sm font-bold">1.2k</span>
            </div>
            <div class="flex items-center gap-1">
              <span class="h-2 w-2 rounded-full bg-rust-orange" />
              <span class="eyebrow text-[10px] text-fg-muted">Rust</span>
            </div>
          </div>
        </div>
        <p class="max-w-sm text-center text-sm text-fg-muted md:text-right">
          Open-source and audited core. Trusted by infrastructure providers for Substrate, EVM, and Cosmos.
        </p>
      </div>
    </section>

    <!-- ARCHITECTURE / WORKFLOW -->
    <section class="bg-surface-muted py-[100px]">
      <div class="container-content">
        <div class="mb-16 text-center">
          <span class="eyebrow text-rust-orange">Architecture</span>
          <h2 class="mt-4 font-display text-4xl font-bold text-heading md:text-5xl">
            Technical Workflow
          </h2>
        </div>

        <div class="grid grid-cols-1 items-center gap-16 lg:grid-cols-2">
          <div class="space-y-4">
            <button
              v-for="step in workflowSteps"
              :key="step.n"
              type="button"
              class="group flex w-full items-start gap-4 text-left"
              @mouseenter="activeStep = step.n"
              @focus="activeStep = step.n"
            >
              <div
                class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full border font-mono text-sm transition-colors"
                :class="activeStep === step.n
                  ? 'border-primary-600 bg-primary-600 text-white'
                  : 'border-subtle text-fg-muted group-hover:border-primary-600 group-hover:text-primary-600'"
              >
                {{ step.n }}
              </div>
              <div>
                <h3 class="font-display text-xl font-semibold text-heading">
                  {{ step.title }}
                </h3>
                <p class="mt-1 text-sm text-fg-muted">
                  {{ step.copy }}
                </p>
              </div>
            </button>
          </div>

          <div class="overflow-hidden rounded-lg border border-neutral-800 bg-deep-ink">
            <div class="flex border-b border-white/10">
              <div class="flex items-center gap-2 border-r border-white/10 bg-deep-ink px-4 py-2">
                <span class="h-2 w-2 rounded-full bg-rust-orange" />
                <span class="eyebrow text-[10px] text-white">mapping.rs</span>
              </div>
              <div class="border-r border-white/10 px-4 py-2">
                <span class="eyebrow text-[10px] text-neutral-500">schema.graphql</span>
              </div>
              <div class="border-r border-white/10 px-4 py-2">
                <span class="eyebrow text-[10px] text-neutral-500">project.yaml</span>
              </div>
            </div>
            <div class="code-syntax-rust overflow-x-auto p-6 font-mono text-[13px] leading-relaxed text-neutral-200">
              <pre><code><span class="comment">// Standard Rust Handler for Transfer Events</span>
<span class="keyword">pub async fn</span> <span class="type">handle_transfer</span>(event: <span class="type">TransferEvent</span>) -&gt; <span class="type">Result</span>&lt;(), <span class="type">Error</span>&gt; {
    <span class="keyword">let</span> <span class="keyword">mut</span> account = <span class="type">Account</span>::<span class="type">load</span>(&amp;event.from).<span class="keyword">await</span>?;

    <span class="comment">// High-integrity balance update</span>
    account.balance -= event.value;
    account.<span class="type">save</span>().<span class="keyword">await</span>?;

    <span class="type">info!</span>(<span class="string">"Indexed transfer of {:?} from {:?}"</span>, event.value, event.from);
    <span class="keyword">Ok</span>(())
}</code></pre>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- FEATURES -->
    <section class="bg-surface py-[100px]">
      <div class="container-content">
        <div class="mb-16 grid grid-cols-1 gap-6 md:grid-cols-3">
          <div
            v-for="f in primaryFeatures"
            :key="f.title"
            class="border border-subtle bg-surface p-8 transition-colors hover:border-primary-600"
          >
            <UIcon
              :name="f.icon"
              class="mb-4 text-3xl text-primary-600"
            />
            <h3 class="mb-2 font-display text-xl font-semibold text-heading">
              {{ f.title }}
            </h3>
            <p class="text-sm text-fg-muted">
              {{ f.copy }}
            </p>
          </div>
        </div>

        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
          <div
            v-for="cap in capabilities"
            :key="cap"
            class="flex items-center gap-4"
          >
            <UIcon
              name="i-lucide-check-circle-2"
              class="shrink-0 text-primary-600"
            />
            <span class="text-base text-fg">{{ cap }}</span>
          </div>
        </div>
      </div>
    </section>

    <!-- ROADMAP (intentionally dark in both modes; slightly lighter in dark to stay distinct from page bg) -->
    <section class="bg-deep-ink dark:bg-[var(--sq-surface-muted)] py-[100px] text-white">
      <div class="container-content">
        <div class="mb-16 text-center">
          <span class="eyebrow text-rust-orange">Milestones</span>
          <h2 class="mt-4 font-display text-4xl font-bold !text-white md:text-5xl">
            Development Roadmap
          </h2>
        </div>

        <div class="grid grid-cols-1 gap-8 md:grid-cols-2">
          <div class="relative overflow-hidden border border-white/10 bg-white/5 p-8">
            <div class="absolute right-0 top-0 border-b border-l border-white/10 bg-primary-600/20 px-3 py-2 font-mono text-[10px] font-semibold uppercase tracking-wider text-primary-200">
              Current Scope
            </div>
            <h3 class="mb-8 font-display text-2xl font-semibold !text-white">
              v0.8 Production Alpha
            </h3>
            <ul class="space-y-4">
              <li
                v-for="item in currentScope"
                :key="item"
                class="flex items-center gap-4"
              >
                <UIcon
                  name="i-lucide-check-circle-2"
                  class="text-primary-400"
                />
                <span class="text-base text-neutral-200">{{ item }}</span>
              </li>
            </ul>
          </div>

          <div class="relative overflow-hidden border border-white/10 bg-white/5 p-8">
            <div class="absolute right-0 top-0 border-b border-l border-white/10 bg-rust-orange/20 px-3 py-2 font-mono text-[10px] font-semibold uppercase tracking-wider text-rust-orange">
              Planned Scope
            </div>
            <h3 class="mb-8 font-display text-2xl font-semibold !text-white">
              v1.0 Stability Goal
            </h3>
            <ul class="space-y-4">
              <li
                v-for="item in plannedScope"
                :key="item"
                class="flex items-center gap-4 opacity-70"
              >
                <UIcon
                  name="i-lucide-circle"
                  class="text-signal-red"
                />
                <span class="text-base text-neutral-200">{{ item }}</span>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </section>

    <!-- INSPIRATION NOTE -->
    <section class="bg-surface-muted py-[100px]">
      <div class="mx-auto max-w-3xl px-6 text-center">
        <span class="eyebrow mb-4 block text-rust-orange">A note on inspiration</span>
        <p class="font-display text-2xl italic leading-relaxed text-heading md:text-[28px]">
          &ldquo;Infrastructure should be invisible, boring, and absolutely reliable.
          We build for the engineers who sleep soundly because they know their indexing
          logic is type-safe and compiled to native metal.&rdquo;
        </p>
        <div class="mx-auto mt-8 h-1 w-12 bg-[var(--sq-border-subtle)]" />
      </div>
    </section>

    <!-- CTA (brand-blue panel — same in both modes) -->
    <section class="py-[100px]">
      <div class="container-content">
        <div class="relative overflow-hidden border border-primary-700 bg-primary-600 p-12">
          <div class="relative z-10 flex flex-col items-center justify-between gap-8 md:flex-row">
            <div class="text-left">
              <h2 class="mb-2 font-display text-4xl font-bold !text-white md:text-5xl">
                Ready to build?
              </h2>
              <p class="text-lg text-primary-100">
                Start querying blockchain data with sub-second latency.
              </p>
            </div>
            <div class="flex w-full flex-col gap-4 sm:flex-row md:w-auto">
              <UButton
                to="/sdk"
                size="lg"
                class="bg-white px-8 py-3 font-mono text-xs uppercase tracking-wider text-primary-600 hover:bg-neutral-50"
              >
                Get started
              </UButton>
              <UButton
                to="https://github.com/superquery/superquery-sdk"
                target="_blank"
                size="lg"
                icon="i-simple-icons-github"
                class="border border-white/20 bg-deep-ink px-8 py-3 font-mono text-xs uppercase tracking-wider text-white hover:bg-deep-ink/80"
              >
                View on GitHub
              </UButton>
            </div>
          </div>
          <div class="absolute -bottom-24 -right-24 h-64 w-64 rounded-full border-[32px] border-white/10" />
        </div>
      </div>
    </section>
  </div>
</template>
