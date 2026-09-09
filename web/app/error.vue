<script setup lang="ts">
import type { NuxtError } from '#app'
defineProps<{ error: NuxtError }>()
useSeoMeta({ title: 'Page unavailable | SuperQuery', robots: 'noindex, nofollow' })
useHead({ htmlAttrs: { lang: 'en' } })
</script>

<template>
  <UApp>
    <UMain class="flex min-h-screen items-center">
      <UContainer class="max-w-2xl py-20 text-center">
        <AppLogo variant="mark" class="mx-auto mb-8 size-14" />
        <p class="eyebrow text-primary">{{ error.statusCode }}</p>
        <h1 class="mt-5 text-4xl font-bold text-highlighted">{{ error.statusCode === 404 ? 'This page is off the map.' : 'Something interrupted the request.' }}</h1>
        <p class="mt-5 leading-7 text-muted">{{ error.statusCode === 404 ? 'The address may have changed. Head back to the homepage or explore the documentation.' : 'Please try again. You can also return to the homepage.' }}</p>
        <div class="mt-8 flex flex-wrap justify-center gap-3">
          <UButton to="/" external icon="i-lucide-arrow-left">Back to home</UButton>
          <UButton to="/docs" external color="neutral" variant="outline">Documentation</UButton>
        </div>
      </UContainer>
    </UMain>
  </UApp>
</template>
