const siteUrl = process.env.NUXT_PUBLIC_SITE_URL || 'https://superquery.vercel.app'

export default defineNuxtConfig({
  modules: ['@nuxt/eslint', '@nuxt/ui', '@nuxtjs/seo'],
  devtools: { enabled: false },
  css: ['~/assets/css/main.css'],
  compatibilityDate: '2026-06-30',
  site: {
    url: siteUrl,
    name: 'SuperQuery',
    description: 'An open-source Rust toolkit for defining, indexing, and querying blockchain data.',
    defaultLocale: 'en',
    indexable: process.env.NUXT_SITE_INDEXABLE !== 'false'
  },
  routeRules: {
    '/': { prerender: true },
    '/sdk': { prerender: true },
    '/docs': { prerender: true },
    '/examples': { prerender: true },
    '/roadmap': { prerender: true },
    '/status': { prerender: true },
    '/changelog': { prerender: true },
    '/grants': { prerender: true },
    '/portal': { prerender: true, robots: false }
  },
  nitro: {
    compressPublicAssets: true,
    prerender: { crawlLinks: true, failOnError: true }
  },
  experimental: {
    defaults: { nuxtLink: { prefetchOn: { interaction: true } } }
  },
  sitemap: { exclude: ['/portal'] },
  robots: { disallow: ['/portal'] },
  // A checked-in social card avoids a native image renderer during deployments.
  ogImage: { enabled: false },
  schemaOrg: {
    identity: {
      type: 'Organization',
      name: 'SuperQuery',
      url: siteUrl,
      logo: `${siteUrl}/superquery-mark.svg`,
      sameAs: ['https://github.com/blockSuperquery']
    }
  },
  ui: { fonts: false, colorMode: true },
  colorMode: { preference: 'light', fallback: 'light', classSuffix: '' },
  eslint: {
    config: { stylistic: { commaDangle: 'never', braceStyle: '1tbs' } }
  }
})
