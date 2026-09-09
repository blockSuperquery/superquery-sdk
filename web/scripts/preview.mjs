import { createServer } from 'node:http'
import { readFile, stat } from 'node:fs/promises'
import { resolve, sep, extname } from 'node:path'

const root = resolve('.output/public')
const mime = { '.html': 'text/html; charset=utf-8', '.js': 'text/javascript; charset=utf-8', '.css': 'text/css; charset=utf-8', '.json': 'application/json', '.svg': 'image/svg+xml', '.png': 'image/png', '.ico': 'image/x-icon', '.woff2': 'font/woff2', '.xml': 'application/xml', '.txt': 'text/plain; charset=utf-8' }

createServer(async (request, response) => {
  try {
    const pathname = decodeURIComponent(new URL(request.url, 'http://localhost').pathname)
    let file = resolve(root, '.' + pathname)
    if (file !== root && !file.startsWith(root + sep)) {
      response.writeHead(403).end()
      return
    }
    let status = 200
    try {
      if ((await stat(file)).isDirectory()) file = resolve(file, 'index.html')
      await stat(file)
    } catch {
      file = resolve(root, '404.html')
      status = 404
    }
    const headers = { 'Content-Type': mime[extname(file)] || 'application/octet-stream' }
    // Match CDN transfer behavior when measuring the generated assets.
    if (status === 200 && /\bgzip\b/.test(request.headers['accept-encoding'] || '')) {
      try {
        await stat(file + '.gz')
        file += '.gz'
        headers['Content-Encoding'] = 'gzip'
        headers.Vary = 'Accept-Encoding'
      } catch { /* Uncompressed files are valid too. */ }
    }
    const body = await readFile(file)
    response.writeHead(status, { ...headers, 'Content-Length': body.length })
    response.end(request.method === 'HEAD' ? undefined : body)
  } catch {
    response.writeHead(400).end('Bad request')
  }
}).listen(4173, '127.0.0.1', () => {
  console.log('Static preview ready at http://127.0.0.1:4173')
})
