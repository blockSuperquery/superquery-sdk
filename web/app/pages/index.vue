<script setup lang="ts">
import { links } from '~/utils/project'

useSeoMeta({
  title: 'Blockchain data, built with Rust',
  description: 'Define your schema, generate Rust types, and explore a modular blockchain indexing stack. Open-source SDK, indexing engine, and GraphQL query service.',
  ogTitle: 'SuperQuery | Blockchain data, built with Rust'
})
useSchemaOrg([defineSoftwareApp({ name: 'SuperQuery', applicationCategory: 'DeveloperApplication', operatingSystem: 'Linux, macOS, Windows', description: 'An open-source Rust toolkit for blockchain indexing.', isAccessibleForFree: true })])
</script>

<template>
  <div>
    <section class="relative overflow-hidden border-b border-default">
      <div
        class="dot-grid pointer-events-none absolute inset-0 opacity-45 [mask-image:linear-gradient(to_bottom,black,transparent)]"
        aria-hidden="true"
      />
      <UPageHero
        orientation="horizontal"
        description="Turn chain events into data you can build with. A Rust toolkit for defining, indexing, and querying blockchain data."
        :ui="{ container: 'relative py-16 sm:py-24 lg:py-28 gap-12 lg:gap-10', title: 'text-5xl sm:text-6xl lg:text-[4.5rem] leading-[1.04] tracking-[-0.05em]', description: 'text-base sm:text-lg leading-8 max-w-lg', links: 'gap-3' }"
      >
        <template #headline>
          <NuxtLink
            to="/roadmap"
            class="inline-flex items-center gap-2 rounded-full border border-default bg-default px-3 py-1.5 text-xs text-muted"
          >
            <span class="size-1.5 rounded-full bg-blue-500" />
            Built in public. Built with Rust.
            <UIcon
              name="i-lucide-arrow-right"
              class="size-3"
            />
          </NuxtLink>
        </template>
        <template #title>
          Blockchain data.<br><span class="text-primary">Supercharged</span><span class="text-red-500">.</span>
        </template>
        <template #footer>
          <div class="mt-8 flex flex-wrap gap-3">
            <UButton
              to="/docs"
              size="xl"
              trailing-icon="i-lucide-arrow-up-right"
            >
              Start building
            </UButton>
            <UButton
              :to="links.sdk"
              target="_blank"
              size="xl"
              icon="i-lucide-github"
              color="neutral"
              variant="outline"
            >
              Explore the code
            </UButton>
          </div>
          <p class="mt-6 font-mono text-[11px] text-muted">
            OPEN SOURCE <span class="mx-2 text-dimmed">/</span> EVM FIRST <span class="mx-2 text-dimmed">/</span> PRE-ALPHA
          </p>
        </template>
        <div class="relative min-w-0">
          <div
            class="absolute -inset-4 rounded-3xl bg-blue-500/5 sm:-inset-6"
            aria-hidden="true"
          />
          <div class="relative overflow-hidden rounded-xl border border-slate-700 bg-ink shadow-2xl shadow-blue-950/15">
            <div class="flex items-center justify-between border-b border-white/10 px-5 py-4">
              <div
                class="flex gap-1.5"
                aria-hidden="true"
              >
                <span class="size-2 rounded-full bg-red-400" /><span class="size-2 rounded-full bg-amber-400" /><span class="size-2 rounded-full bg-emerald-400" />
              </div>
              <span class="font-mono text-[10px] tracking-wider text-slate-400">ERC-20 TRANSFERS</span>
              <UIcon
                name="i-lucide-code-2"
                class="size-4 text-slate-400"
              />
            </div>
            <div class="px-5 pt-5 sm:px-7">
              <p class="mb-5 font-mono text-xs text-slate-400">
                schema.graphql
              </p>
              <pre class="overflow-x-auto font-mono text-[13px] leading-7 text-slate-200"><code><span class="text-blue-400">type</span> <span class="text-white">Transfer</span> <span class="text-orange-300">@entity</span> {
  id: <span class="text-blue-300">ID!</span>
  from: <span class="text-blue-300">String!</span>
  to: <span class="text-blue-300">String!</span>
  value: <span class="text-blue-300">BigInt!</span>
}</code></pre>
              <p class="mb-5 mt-4 text-[11px] text-slate-400">
                An excerpt from the EVM starter schema.
              </p>
            </div>
            <div class="flex items-center gap-3 border-t border-white/10 bg-white/[0.03] px-5 py-4 sm:px-7">
              <span class="flex size-8 items-center justify-center rounded-lg bg-blue-400/10"><UIcon
                name="i-lucide-arrow-down-to-line"
                class="size-4 text-blue-400"
              /></span>
              <div>
                <p class="font-mono text-xs text-slate-200">
                  schema → Rust types
                </p><p class="mt-1 text-[11px] text-slate-400">
                  One project. A shared contract.
                </p>
              </div>
            </div>
          </div>
          <div class="relative mx-4 -mt-0.5 flex items-center justify-between rounded-b-xl border border-t-0 border-default bg-default px-4 py-3 text-[10px] text-muted sm:mx-6 sm:text-xs">
            <span class="flex items-center gap-1.5"><UIcon
              name="i-lucide-file-code"
              class="size-3.5 text-primary"
            />Define</span><UIcon
              name="i-lucide-arrow-right"
              class="size-3"
            />
            <span class="flex items-center gap-1.5"><UIcon
              name="i-lucide-box"
              class="size-3.5 text-primary"
            />Generate</span><UIcon
              name="i-lucide-arrow-right"
              class="size-3"
            />
            <span class="flex items-center gap-1.5"><UIcon
              name="i-lucide-search"
              class="size-3.5 text-primary"
            />Query</span>
          </div>
        </div>
      </UPageHero>
    </section>

    <UContainer class="flex flex-wrap items-center justify-between gap-x-8 gap-y-4 py-7">
      <p class="eyebrow text-muted">
        Familiar primitives. A new foundation.
      </p>
      <div class="flex flex-wrap gap-6 text-sm font-medium text-muted sm:gap-10">
        <span class="flex items-center gap-2"><UIcon
          name="i-lucide-settings-2"
          class="size-4"
        />Rust</span>
        <span class="flex items-center gap-2"><UIcon
          name="i-lucide-box"
          class="size-4"
        />WebAssembly</span>
        <span class="flex items-center gap-2"><UIcon
          name="i-lucide-database"
          class="size-4"
        />PostgreSQL</span>
        <span class="flex items-center gap-2"><UIcon
          name="i-lucide-network"
          class="size-4"
        />GraphQL</span>
      </div>
    </UContainer>

    <LazyHomeContent hydrate-on-visible />
  </div>
</template>
