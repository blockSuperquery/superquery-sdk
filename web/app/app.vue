<script setup lang="ts">
import displayFont from '@fontsource-variable/space-grotesk/files/space-grotesk-latin-wght-normal.woff2'
import bodyFont from '@fontsource-variable/inter/files/inter-latin-wght-normal.woff2'

useHead({
  titleTemplate: title => title ? `${title} | SuperQuery` : 'SuperQuery | Blockchain data, built with Rust',
  htmlAttrs: { lang: 'en' },
  meta: [{ name: 'theme-color', content: '#2563eb' }],
  link: [
    { rel: 'icon', href: '/superquery-app-icon.svg', type: 'image/svg+xml' },
    { rel: 'preload', as: 'font', type: 'font/woff2', href: displayFont, crossorigin: '' },
    { rel: 'preload', as: 'font', type: 'font/woff2', href: bodyFont, crossorigin: '' }
  ]
})
useSeoMeta({ ogType: 'website', ogSiteName: 'SuperQuery', twitterCard: 'summary_large_image', ogImage: '/og-image.png', twitterImage: '/og-image.png', ogImageAlt: 'SuperQuery. Blockchain data, built with Rust.' })
</script>

<template>
  <UApp>
    <a
      href="#main-content"
      class="skip-link"
    >Skip to content</a>
    <SiteHeader />
    <UMain
      id="main-content"
      tabindex="-1"
    >
      <NuxtPage />
    </UMain>
    <LazySiteFooter hydrate-on-visible />
  </UApp>
</template>
