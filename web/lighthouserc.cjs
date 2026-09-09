const { chromium } = require('@playwright/test')

module.exports = {
  ci: {
    collect: {
      startServerCommand: 'node scripts/preview.mjs',
      startServerReadyPattern: 'Static preview ready',
      url: ['http://127.0.0.1:4173/', 'http://127.0.0.1:4173/docs', 'http://127.0.0.1:4173/examples'],
      numberOfRuns: 3,
      chromePath: process.env.CHROME_PATH || chromium.executablePath(),
      settings: { chromeFlags: '--no-sandbox --disable-dev-shm-usage', onlyCategories: ['performance', 'accessibility', 'best-practices', 'seo'] }
    },
    assert: {
      assertions: {
        'categories:performance': ['error', { minScore: 0.9, aggregationMethod: 'median' }],
        'categories:accessibility': ['error', { minScore: 1, aggregationMethod: 'median' }],
        'categories:best-practices': ['error', { minScore: 0.95, aggregationMethod: 'median' }],
        'categories:seo': ['error', { minScore: 1, aggregationMethod: 'median' }],
        'largest-contentful-paint': ['error', { maxNumericValue: 2500, aggregationMethod: 'median' }],
        'cumulative-layout-shift': ['error', { maxNumericValue: 0.1, aggregationMethod: 'median' }],
        'total-blocking-time': ['error', { maxNumericValue: 200, aggregationMethod: 'median' }]
      }
    },
    upload: { target: 'filesystem', outputDir: '.lighthouseci/reports' }
  }
}
