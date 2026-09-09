<script setup lang="ts">
useSeoMeta({
  title: 'Roadmap',
  description: 'SuperQuery development roadmap — current v0.8 production alpha scope, planned v1.0 stability milestones, distributed ingestion, and WASM sandboxing.'
})
</script>

<template>
  <section class="py-[100px]">
    <div class="container-content max-w-3xl text-center">
      <span class="eyebrow text-rust-orange">In progress</span>
      <h1 class="mt-6 font-display text-5xl font-bold tracking-tight text-heading">
        Roadmap<span class="text-signal-red">.</span>
      </h1>
      <p class="mt-6 text-lg text-fg-muted">
        This page is being built from the SuperQuery design handoff.
      </p>
      <UButton
        to="/"
        icon="i-lucide-arrow-left"
        variant="outline"
        color="neutral"
        class="mt-10 border-subtle font-mono text-xs uppercase tracking-wider text-heading"
      >
        Back to home
      </UButton>
    </div>
  </section>
</template>
