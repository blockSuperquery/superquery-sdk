const siteUrl = process.env.NUXT_PUBLIC_SITE_URL || 'https://superquery.vercel.app'

export default defineNuxtConfig({
  modules: ['@nuxt/eslint', '@nuxt/ui', '@nuxtjs/seo'],
  devtools: { enabled: false },
  css: ['~/assets/css/main.css'],
  site: {
    url: siteUrl,
    name: 'SuperQuery',
    description: 'An open-source Rust toolkit for defining, indexing, and querying blockchain data.',
    defaultLocale: 'en',
    indexable: process.env.NUXT_SITE_INDEXABLE !== 'false'
  },
  colorMode: { preference: 'light', fallback: 'light', classSuffix: '' },
  ui: { fonts: false, colorMode: true },
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
  experimental: {
    defaults: { nuxtLink: { prefetchOn: { interaction: true } } }
  },
  compatibilityDate: '2026-06-30',
  nitro: {
    compressPublicAssets: true,
    prerender: { crawlLinks: true, failOnError: true }
  },
  eslint: {
    config: { stylistic: { commaDangle: 'never', braceStyle: '1tbs' } }
  },
  // A checked-in social card avoids a native image renderer during deployments.
  ogImage: { enabled: false },
  robots: { disallow: ['/portal'] },
  schemaOrg: {
    identity: {
      type: 'Organization',
      name: 'SuperQuery',
      url: siteUrl,
      logo: `${siteUrl}/superquery-mark.svg`,
      sameAs: ['https://github.com/blockSuperquery']
    }
  },
  sitemap: { exclude: ['/portal'] }
})
