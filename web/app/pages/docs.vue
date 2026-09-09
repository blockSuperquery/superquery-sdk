<script setup lang="ts">
useSeoMeta({
  title: 'Docs',
  description: 'SuperQuery documentation — architecture, Rust handlers, GraphQL schema definitions, and deployment guides for the high-integrity blockchain indexing framework.'
})
</script>

<template>
  <section class="py-[100px]">
    <div class="container-content max-w-3xl text-center">
      <span class="eyebrow text-rust-orange">In progress</span>
      <h1 class="mt-6 font-display text-5xl font-bold tracking-tight text-heading">
        Docs<span class="text-signal-red">.</span>
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
