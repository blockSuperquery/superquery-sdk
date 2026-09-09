// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: [
    '@nuxt/eslint',
    '@nuxt/ui',
    '@nuxtjs/seo'
  ],

  devtools: {
    enabled: true
  },

  css: ['~/assets/css/main.css'],

  routeRules: {
    '/': { prerender: true }
  },

  compatibilityDate: '2026-06-30',

  site: {
    url: 'https://superquery.io',
    name: 'SuperQuery',
    description: 'A high-integrity indexing framework designed for the most demanding blockchain data pipelines. Performance of Rust, familiarity of SubQuery.',
    defaultLocale: 'en',
    indexable: true
  },

  sitemap: {
    exclude: ['/portal/**']
  },

  robots: {
    disallow: ['/portal']
  },

  ogImage: {
    fonts: ['Space+Grotesk:700', 'Inter:400']
  },

  schemaOrg: {
    identity: {
      type: 'Organization',
      name: 'SuperQuery',
      url: 'https://superquery.io',
      logo: 'https://superquery.io/superquery-mark.svg',
      sameAs: [
        'https://github.com/superquery/superquery-sdk',
        'https://discord.gg/superquery'
      ]
    }
  },

  ui: {
    fonts: true,
    colorMode: true
  },

  colorMode: {
    preference: 'light',
    fallback: 'light',
    classSuffix: ''
  },

  fonts: {
    families: [
      { name: 'Inter', provider: 'google', weights: [400, 500, 600, 700] },
      { name: 'Space Grotesk', provider: 'google', weights: [500, 600, 700] },
      { name: 'JetBrains Mono', provider: 'google', weights: [400, 500, 600] }
    ]
  },

  eslint: {
    config: {
      stylistic: {
        commaDangle: 'never',
        braceStyle: '1tbs'
      }
    }
  }
})
