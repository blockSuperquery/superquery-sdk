<script setup lang="ts">
withDefaults(
  defineProps<{
    variant?: 'wordmark' | 'mark'
  }>(),
  { variant: 'wordmark' }
)
</script>

<template>
  <svg
    v-if="variant === 'wordmark'"
    viewBox="0 0 980 240"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
    role="img"
    aria-label="SuperQuery"
  >
    <defs>
      <linearGradient id="sqRed" x1="34" y1="35" x2="144" y2="88" gradientUnits="userSpaceOnUse">
        <stop offset="0" stop-color="#FF4A3D" />
        <stop offset="1" stop-color="#DC2626" />
      </linearGradient>
      <linearGradient id="sqBlue" x1="142" y1="90" x2="34" y2="158" gradientUnits="userSpaceOnUse">
        <stop offset="0" stop-color="#2563EB" />
        <stop offset="1" stop-color="#1D4ED8" />
      </linearGradient>
      <linearGradient id="sqBolt" x1="92" y1="100" x2="161" y2="177" gradientUnits="userSpaceOnUse">
        <stop offset="0" stop-color="#2563EB" />
        <stop offset="1" stop-color="#1D4ED8" />
      </linearGradient>
    </defs>
    <g transform="translate(24 28)">
      <path
        d="M30 92C30 61.072 55.072 36 86 36C116.928 36 142 61.072 142 92"
        stroke="url(#sqRed)"
        stroke-width="22"
        stroke-linecap="round"
      />
      <path
        d="M142 92C142 122.928 116.928 148 86 148C55.072 148 30 122.928 30 92"
        stroke="url(#sqBlue)"
        stroke-width="22"
        stroke-linecap="round"
      />
      <path
        d="M88 95L158 165L121 128H144L91 84L104 112H76L88 95Z"
        fill="url(#sqBolt)"
      />
    </g>
    <text
      x="210"
      y="145"
      font-family="Space Grotesk, Inter, sans-serif"
      font-size="86"
      font-weight="800"
      letter-spacing="-4"
      fill="currentColor"
    >SuperQuery</text>
  </svg>

  <svg
    v-else
    viewBox="0 0 120 120"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
    role="img"
    aria-label="SuperQuery"
  >
    <defs>
      <linearGradient id="sqMarkRed" x1="26" y1="18" x2="96" y2="52" gradientUnits="userSpaceOnUse">
        <stop offset="0" stop-color="#FF4A3D" />
        <stop offset="1" stop-color="#DC2626" />
      </linearGradient>
      <linearGradient id="sqMarkBlue" x1="92" y1="55" x2="26" y2="98" gradientUnits="userSpaceOnUse">
        <stop offset="0" stop-color="#2563EB" />
        <stop offset="1" stop-color="#1D4ED8" />
      </linearGradient>
      <linearGradient id="sqMarkBolt" x1="66" y1="58" x2="105" y2="108" gradientUnits="userSpaceOnUse">
        <stop offset="0" stop-color="#2563EB" />
        <stop offset="1" stop-color="#1D4ED8" />
      </linearGradient>
    </defs>
    <path
      d="M28 58C28 39.225 43.225 24 62 24C80.775 24 96 39.225 96 58"
      stroke="url(#sqMarkRed)"
      stroke-width="14"
      stroke-linecap="round"
    />
    <path
      d="M96 58C96 76.775 80.775 92 62 92C43.225 92 28 76.775 28 58"
      stroke="url(#sqMarkBlue)"
      stroke-width="14"
      stroke-linecap="round"
    />
    <path
      d="M64 61L104 103L82 82H96L66 55L74 72H57L64 61Z"
      fill="url(#sqMarkBolt)"
    />
  </svg>
</template>
