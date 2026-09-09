<script setup lang="ts">
useSeoMeta({
  title: 'Status',
  description: 'Live network status for SuperQuery — indexer uptime, tail latency, throughput, and per-chain synchronization health.'
})
</script>

<template>
  <section class="py-[100px]">
    <div class="container-content max-w-3xl text-center">
      <span class="eyebrow text-rust-orange">In progress</span>
      <h1 class="mt-6 font-display text-5xl font-bold tracking-tight text-heading">
        Status<span class="text-signal-red">.</span>
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
