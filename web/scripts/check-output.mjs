import assert from 'node:assert/strict'
import { readFile, stat } from 'node:fs/promises'
import { join } from 'node:path'

const root = '.output/public'
const origin = (process.env.NUXT_PUBLIC_SITE_URL || 'https://superquery.vercel.app').replace(/\/$/, '')
const routes = ['/', '/sdk', '/docs', '/examples', '/roadmap', '/status', '/changelog', '/grants', '/portal']
const titles = new Set()
for (const route of routes) {
  const html = await readFile(join(root, route, 'index.html'), 'utf8')
  const title = html.match(/<title>([^<]+)<\/title>/)?.[1]
  assert(title && !titles.has(title), `${route}: missing or duplicate title`)
  titles.add(title)
  assert.equal((html.match(/<h1\b/g) || []).length, 1, `${route}: expected one h1`)
  assert.match(html, /<meta[^>]+name="description"[^>]+content="[^"]+"/, `${route}: missing description`)
  assert(html.includes(`href="${origin}${route === '/' ? '/' : route}"`) || (route === '/' && html.includes(`href="${origin}"`)), `${route}: missing canonical URL`)
  assert(!html.includes('\u2014'), `${route}: contains an em dash`)
  assert(!/being built from|github\.com\/superquery\/|discord\.gg\/superquery|5×|1\.2k/.test(html), `${route}: stale placeholder or unsupported claim`)
  assert((await stat(join(root, route, 'index.html'))).size < 180_000, `${route}: HTML exceeds 180 KB`)
}
const robots = await readFile(join(root, 'robots.txt'), 'utf8')
assert.match(robots, /Disallow: \/portal/)
const sitemap = await readFile(join(root, 'sitemap.xml'), 'utf8')
for (const route of routes.filter(route => route !== '/portal')) {
  assert(sitemap.includes(`<loc>${origin}${route === '/' ? '/' : route}</loc>`) || (route === '/' && sitemap.includes(`<loc>${origin}</loc>`)), `Sitemap missing ${route}`)
}
assert(!sitemap.includes(`${origin}/portal`), 'Portal must not be in sitemap')
const social = await readFile(join(root, 'og-image.png'))
assert.equal(social.readUInt32BE(16), 1200, 'Social image width must be 1200')
assert.equal(social.readUInt32BE(20), 630, 'Social image height must be 630')
assert((await stat(join(root, '404.html'))).size > 0, 'Missing 404 page')
console.log(`Verified ${routes.length} prerendered pages, metadata, sitemap, robots, social image, and HTML budget.`)
