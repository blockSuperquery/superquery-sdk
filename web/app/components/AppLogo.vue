<script setup lang="ts">
withDefaults(defineProps<{ variant?: 'wordmark' | 'mark' }>(), { variant: 'wordmark' })
</script>

<template>
  <img
    v-if="variant === 'mark'"
    src="/superquery-mark.svg"
    alt=""
    width="120"
    height="120"
  >
  <picture v-else>
    <source
      media="(prefers-color-scheme: dark)"
      srcset="/superquery-wordmark-dark.svg"
    >
    <img
      src="/superquery-wordmark.svg"
      alt="SuperQuery"
      width="980"
      height="240"
    >
  </picture>
</template>
