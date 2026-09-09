<div align="center">

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./public/superquery-wordmark-dark.svg">
  <img alt="SuperQuery" src="./public/superquery-wordmark.svg" width="440">
</picture>

### Query at super speed.

##### The Web / Landing Portal

**`RUST-NATIVE` · `SUBQUERY COMPATIBLE`**

The official website and documentation portal for **SuperQuery** — a
high-integrity indexing framework designed for the most demanding blockchain
data pipelines. The performance of Rust, the familiarity of SubQuery.

[Live site](https://superquery.vercel.app/) ·
[SuperQuery core](https://github.com/blockSuperquery)

</div>

---

## About

This repository contains the SuperQuery marketing site and documentation portal,
built with [Nuxt](https://nuxt.com) and [Nuxt UI](https://ui.nuxt.com). It
presents the SuperQuery platform:

- **Native performance** — mappings compile to native Rust code: no interpreter,
  no GC pauses, predictable tail latency under load.
- **High integrity** — deterministic indexing with ACID-compliant PostgreSQL
  storage and reorg-safe historical state.
- **SubQuery compatible** — reuse your GraphQL schema and project manifest; a
  migration path, not a rewrite.
- **Observable** — first-class Prometheus metrics, health endpoints, and
  structured tracing for production fleets.

For the indexing engine itself, see the SuperQuery core (Rust) repository.

## Tech Stack

- [Nuxt](https://nuxt.com) — Vue meta-framework
- [Nuxt UI](https://ui.nuxt.com) — component library
- [pnpm](https://pnpm.io) — package manager
- Deployed on [Vercel](https://vercel.com)

## Platform Roadmap & Progress

SuperQuery is built test-first, with every engine milestone gated by
**differential tests against the reference SubQuery implementation** (schema and
data asserted byte-identical before a gate passes).

**Delivered**

- [x] Rust workspace + crate topology (engine · store · config · node/query/cli)
- [x] Node & database configuration ported 1:1
- [x] Core engine types and trait seams (`BlockchainService`, `ProjectService`, `Store`, `BlockDispatcher`)
- [x] **Gate 1** — live connectivity verified against real RPC + real Postgres
- [x] PostgreSQL storage layer: schema-introspection differ + golden-fixture pipeline
- [x] GraphQL schema → DDL generation — **Gate 2 (schema)** byte-identical to reference
- [x] Direct-DB model (upsert · delete · get · filtered queries) — **Gate 2 (data)** byte-identical
- [x] Marketing + docs web portal (this repo)

**In progress**

- [ ] Metadata model, write-behind cache, historical `_block_range`
- [ ] Enums, embedded JSON types, relations / foreign keys

**Planned**

- [ ] Fetch + dispatch pipeline (**Gate 3**)
- [ ] Chain integration (Substrate + EVM) — first end-to-end index (**Gate 4 · MVP**)
- [ ] Proof-of-index merkle cross-verification (**Gate 5**)
- [ ] Rust / WASM mapping execution (**Gate 6**)
- [ ] Dictionary, reorg/rewind, multi-worker & multi-chain
- [ ] GraphQL query service, admin / health endpoints, Prometheus metrics

## Setup

Install dependencies:

```bash
pnpm install
```

## Development Server

Start the development server on `http://localhost:3000`:

```bash
pnpm dev
```

## Production

Build the application for production:

```bash
pnpm build
```

Locally preview the production build:

```bash
pnpm preview
```

See the [Nuxt deployment documentation](https://nuxt.com/docs/getting-started/deployment)
for more information. Production builds deploy automatically to Vercel on push to
`main`.

## Renovate integration

Dependency updates are managed by [Renovate](https://github.com/apps/renovate/installations/select_target).

## License

See [LICENSE](LICENSE).
