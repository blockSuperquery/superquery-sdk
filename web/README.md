# SuperQuery web

The public SuperQuery site, built with Nuxt 4, Nuxt UI 4, and Tailwind CSS 4.
It lives in `web/`, outside the Rust workspace.

## Development

Use Bun 1.3.14 (see `.bun-version`) for dependencies and scripts. Node 24
(see `.node-version`) is also needed by Nuxt and the Lighthouse tooling.

```bash
bun install --frozen-lockfile
bun run dev
```

## Production and verification

```bash
bun run lint
bun run typecheck
bun run generate
bun run check:output
bunx --no-install playwright install --with-deps chromium
bun run test:e2e
bun run test:perf
```

`generate` prerenders all nine routes into `.output/public/`.
`preview:static` serves this output on port 4173 with gzip when available.
`build` and `preview` remain available for Nuxt's server deployment workflow.

The browser suite checks desktop and mobile layouts, accessible names and contrast,
navigation, keyboard controls, copy buttons, dark mode, documentation anchors, and
404 responses. Lighthouse checks the homepage, docs, and examples with three mobile
runs each. Reports stay local under `playwright-report/` and `.lighthouseci/`.

To use an existing browser, set `CHROME_PATH` to its executable path.
Close the static preview before running Lighthouse, which starts its own server.

## CI and deployment

GitHub Actions reads `web/.bun-version` and installs from `web/bun.lock` with
`--frozen-lockfile`. Rust and web checks run separately. Web CI generates the site,
checks metadata and output budgets, runs browser and Lighthouse checks, and saves
the static output and reports as artifacts.

Vercel project settings are managed separately. The repository's web root is
`web/`; this change does not modify the Vercel project or deployment settings.
Use Bun locally so local builds and CI use the same lockfile.

Copy `.env.example` for configuration. Set `NUXT_PUBLIC_SITE_URL` to the
production origin before building to keep canonical URLs, the sitemap, and
structured data aligned. The default is `https://superquery.vercel.app`.
Set `NUXT_SITE_INDEXABLE=false` for non-production previews. The portal is
excluded from indexing and the sitemap independently.

## Performance choices

- Prerender every public route and compress the output.
- Bundle Latin variable fonts and only the icons used by the site.
- Keep font and icon requests local.
- Prefetch links on interaction instead of fetching all visible destinations.
- Keep navigation immediately interactive and hydrate below-the-fold content on visibility.
- Generate Nuxt UI themes only for components the site uses.
- Use CSS and an HTML code illustration for the hero.
- Serve a checked-in 1200 × 630 social card without a runtime image renderer.
- Respect reduced motion and use Nuxt UI's accessible navigation and controls.

## Content and project scope

`app/utils/project.ts` contains repository links and reusable examples.
The site reflects the implementation, not assumed production capabilities:

| Repository | Responsibility | Current scope |
| --- | --- | --- |
| [SDK](https://github.com/blockSuperquery/superquery-sdk) | Developer contract and tooling | Validation, schema parsing, codegen, EVM ABI bindings, doctor |
| [Node](https://github.com/blockSuperquery/superquery-node) | Ingestion and indexed-state writes | Pre-alpha components; full pipeline still being connected |
| [Query](https://github.com/blockSuperquery/superquery-query) | PostgreSQL-backed GraphQL reads | Filtering, sorting, pagination, relations, health and query limits |

Source review: 9 September 2026. SDK init/build/test, full mapping execution,
additional chain integrations, and managed hosting must not be presented as
available. Do not add invented benchmarks, adoption numbers, live health data,
funding offers, or nonfunctional signup forms. Site copy uses no em dashes.

For shared contracts, see [the specifications](../docs/spec/README.md).
For developer milestones, see [the implementation plan](../.claude/IMPLEMENTATION_PLAN.md).

## References

- [Nuxt rendering modes](https://nuxt.com/docs/4.x/guide/concepts/rendering)
- [Nuxt performance](https://nuxt.com/docs/4.x/guide/best-practices/performance)
- [Nuxt UI Header](https://ui.nuxt.com/docs/components/header)
- [Nuxt UI components](https://ui.nuxt.com/docs/components)
- [Lighthouse scoring](https://developer.chrome.com/docs/lighthouse/performance/performance-scoring)
