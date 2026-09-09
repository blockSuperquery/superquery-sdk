# SuperQuery SDK — Rust Implementation Guide

> Source document for this repository's design. Reproduced here as the
> reference an agent or contributor should read before changing the crate
> topology, the manifest format, the schema IR or the mapping ABI.
>
> Repository: `blockSuperquery/superquery-sdk`
> Upstream reference: SubQuery CLI + common project model + public types +
> chain-specific manifest/types packages.

---

## 1. What this repository owns

`superquery-sdk` is the **developer-facing contract** of the whole platform.

It contains:

- project manifest model
- manifest parsing and validation
- schema parsing
- shared schema/entity intermediate representation
- mapping SDK
- mapping host-call types
- procedural macros
- code generation
- chain-specific developer types
- CLI
- templates/examples
- project build/bundle format
- docs
- website/dashboard while the organization remains exactly three repositories

It does **not** contain:

- RPC indexing loop → `superquery-node`
- PostgreSQL GraphQL server → `superquery-query`
- node reorg/checkpoint implementation
- query execution implementation

Mental model:

```text
Developer
   |
   |-- project.yaml
   |-- schema.graphql
   |-- ABI/assets
   |-- src/mappings.rs
          |
          v
    superquery-sdk
          |
          |-- validate
          |-- codegen
          |-- compile
          |-- bundle
          |
          v
       dist/
       |-- manifest.json
       |-- schema.graphql
       |-- schema.ir.json
       |-- mapping.wasm
       |-- assets/
       |-- build.json
```

---

## 2. Closest SubQuery equivalents

| SuperQuery SDK responsibility | SubQuery reference |
|---|---|
| CLI | [`subquery/subql/packages/cli`](https://github.com/subquery/subql/tree/main/packages/cli) |
| generic project model | [`subquery/subql/packages/common`](https://github.com/subquery/subql/tree/main/packages/common) |
| core/shared types | [`subquery/subql/packages/types-core`](https://github.com/subquery/subql/tree/main/packages/types-core) |
| project/mapping types | [`subquery/subql/packages/types`](https://github.com/subquery/subql/tree/main/packages/types) |
| EVM common/project config | [`subquery/subql-ethereum/packages/common-ethereum`](https://github.com/subquery/subql-ethereum/tree/main/packages/common-ethereum) |
| EVM mapping types | [`subquery/subql-ethereum/packages/types`](https://github.com/subquery/subql-ethereum/tree/main/packages/types) |
| Stellar reference | [`subquery/subql-stellar`](https://github.com/subquery/subql-stellar) |
| Solana reference | [`subquery/subql-solana`](https://github.com/subquery/subql-solana) |

This repo intentionally combines what SubQuery spreads across several packages,
because the organization is exactly three repositories.

---

## 3. Upstream CLI source map

CLI package: [`packages/cli`](https://github.com/subquery/subql/tree/main/packages/cli)
Commands: [`packages/cli/src/commands`](https://github.com/subquery/subql/tree/main/packages/cli/src/commands)

Important commands:

- [`init.ts`](https://github.com/subquery/subql/blob/main/packages/cli/src/commands/init.ts)
- [`build.ts`](https://github.com/subquery/subql/blob/main/packages/cli/src/commands/build.ts)
- [`build-manifest.ts`](https://github.com/subquery/subql/blob/main/packages/cli/src/commands/build-manifest.ts)
- [`codegen/`](https://github.com/subquery/subql/tree/main/packages/cli/src/commands/codegen)
- [`codegen/index.ts`](https://github.com/subquery/subql/blob/main/packages/cli/src/commands/codegen/index.ts)
- [`codegen/import-abi.ts`](https://github.com/subquery/subql/blob/main/packages/cli/src/commands/codegen/import-abi.ts)
- [`migrate.ts`](https://github.com/subquery/subql/blob/main/packages/cli/src/commands/migrate.ts)
- [`multi-chain/`](https://github.com/subquery/subql/tree/main/packages/cli/src/commands/multi-chain)
- [`publish.ts`](https://github.com/subquery/subql/blob/main/packages/cli/src/commands/publish.ts)

Do not implement every current SubQuery command in v0.1. The correct SuperQuery
first set is:

```text
superquery init
superquery validate
superquery codegen
superquery build
superquery test
superquery doctor
```

Deployment/network publishing comes later.

---

## 4. Upstream common/project model

Study [`packages/common/src`](https://github.com/subquery/subql/tree/main/packages/common/src),
especially [`packages/common/src/project`](https://github.com/subquery/subql/tree/main/packages/common/src/project):

- [`project/readers`](https://github.com/subquery/subql/tree/main/packages/common/src/project/readers)
- [`project/versioned`](https://github.com/subquery/subql/tree/main/packages/common/src/project/versioned)
- [`multichain`](https://github.com/subquery/subql/tree/main/packages/common/src/multichain)

### SuperQuery equivalent

```text
crates/manifest/
`-- src/
    |-- lib.rs
    |-- version.rs
    |-- project.rs
    |-- network.rs
    |-- datasource.rs
    |-- handler.rs
    |-- filter.rs
    |-- reader.rs
    `-- validate.rs
```

Use `serde` structs, explicit schema versions and strong validation.

---

## 5. Upstream public types

Study [`packages/types/src`](https://github.com/subquery/subql/tree/main/packages/types/src):

- [`global.ts`](https://github.com/subquery/subql/blob/main/packages/types/src/global.ts)
- [`interfaces.ts`](https://github.com/subquery/subql/blob/main/packages/types/src/interfaces.ts)
- [`project.ts`](https://github.com/subquery/subql/blob/main/packages/types/src/project.ts)
- [`index.ts`](https://github.com/subquery/subql/blob/main/packages/types/src/index.ts)

Also [`packages/types-core`](https://github.com/subquery/subql/tree/main/packages/types-core).

### SuperQuery equivalent

Publish stable Rust crates:

```text
superquery-sdk
superquery-types
superquery-manifest
superquery-schema
superquery-macros
```

Node and query consume these types rather than define duplicate versions.

---

## 6. Recommended repository layout

```text
superquery-sdk/
|-- Cargo.toml
|-- README.md
|-- crates/
|   |-- sdk/
|   |-- types/
|   |-- manifest/
|   |-- schema/
|   |-- codegen/
|   |-- macros/
|   |-- chain-api/
|   |-- chains/
|   |   |-- evm/
|   |   |-- stellar/
|   |   `-- solana/
|   `-- cli/
|
|-- templates/
|   |-- evm/
|   |-- stellar/
|   `-- solana/
|
|-- examples/
|   |-- erc20-transfers/
|   `-- uniswap-v3/
|
|-- docs/
`-- web/
```

`web/` is a Node/Nuxt/Next frontend workspace inside this repository, kept out
of the Rust Cargo workspace through workspace exclusions.

---

## 7. Define the public project format first

Do this before CLI polish.

```yaml
specVersion: "1.0"

name: erc20-transfers
version: "0.1.0"

network:
  family: evm
  chainId: "1"
  endpoint:
    - "https://..."

startBlock: 21000000

schema:
  file: "./schema.graphql"

dataSources:
  - kind: evm/Runtime
    options:
      address: "0x..."
    handlers:
      - handler: handle_transfer
        kind: evm/LogHandler
        filter:
          event: "Transfer(address,address,uint256)"
```

Rust model:

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProjectManifest {
    pub spec_version: SpecVersion,
    pub name: String,
    pub version: String,
    pub network: NetworkConfig,
    pub schema: SchemaRef,
    pub data_sources: Vec<DataSource>,
}
```

Make versioning explicit from day one.

---

## 8. Manifest implementation

### Step 1 — Parse

Dependencies: `serde`, `serde_yaml`, `serde_json`, `semver`, `url`.

### Step 2 — Semantic validation

Parsing is not enough. Validate:

- supported spec version
- known chain family
- non-empty endpoint
- valid chain ID
- schema path exists
- unique datasource names if named
- handler exists
- handler/filter combination is valid for chain
- start block is sensible
- referenced ABI/assets exist

Return structured diagnostics, not strings. Use `miette` for good CLI errors.

Acceptance: `superquery validate` must point to the exact bad field with a
human-readable reason.

---

## 9. GraphQL entity schema parser

Parse the GraphQL schema into a chain-independent IR.

```graphql
type Transfer @entity {
  id: ID!
  from: String!
  to: String!
  value: BigInt!
  blockNumber: BigInt!
}
```

```rust
pub struct SchemaIr {
    pub entities: Vec<EntityDefinition>,
}

pub struct EntityDefinition {
    pub name: String,
    pub fields: Vec<FieldDefinition>,
    pub indexes: Vec<IndexDefinition>,
}
```

```text
crates/schema/src/
|-- parser.rs
|-- ir.rs
|-- scalar.rs
|-- relation.rs
|-- index.rs
|-- validate.rs
`-- canonical.rs
```

The **same IR** feeds:

1. SDK Rust entity codegen
2. node DB table/migration planning
3. query GraphQL schema generation

That eliminates three incompatible schema implementations.

---

## 10. Entity code generation

`superquery codegen` turns schema entities into Rust.

```rust
#[derive(Clone, Debug, SuperQueryEntity)]
pub struct Transfer {
    pub id: String,
    pub from: String,
    pub value: BigInt,
}
```

Generate: entity structs, field metadata, serialization, store save/get/remove
methods or trait implementations, relation metadata.

Study the *intent* of
[`packages/cli/src/commands/codegen`](https://github.com/subquery/subql/tree/main/packages/cli/src/commands/codegen).
Do not mechanically translate TypeScript templates.

---

## 11. ABI code generation

Study [`codegen/import-abi.ts`](https://github.com/subquery/subql/blob/main/packages/cli/src/commands/codegen/import-abi.ts).

For EVM, prefer Alloy's ABI/types ecosystem: `alloy-json-abi`,
`alloy-sol-types`, `alloy-primitives`.

Do not build your own Ethereum ABI decoder unless necessary.

---

## 12. Mapping SDK

Developer mappings should feel small:

```rust
use superquery_sdk::prelude::*;

#[handler]
pub async fn handle_transfer(event: EvmLog<Transfer>) -> Result<()> {
    let entity = TransferEntity {
        id: format!("{}-{}", event.tx_hash, event.log_index),
        from: event.params.from.to_string(),
        to: event.params.to.to_string(),
        value: event.params.value.into(),
    };

    entity.save().await?;
    Ok(())
}
```

The SDK does not open PostgreSQL. Generated code invokes mapping host calls
supplied by `superquery-node`'s WASM runtime:

```text
mapping SDK `entity.save()`
          |
          v
WASM import `sq_store_set`
          |
          v
node runtime
          |
          v
transactional EntityStore
```

---

## 13. Mapping ABI contract

The public guest side lives here even though the runtime implementation is in
the node.

```text
crates/sdk/src/host/
|-- store.rs
|-- log.rs
|-- chain.rs
`-- abi.rs
```

```rust
pub const MAPPING_ABI_VERSION: u32 = 1;
```

SDK build metadata includes this version. The node rejects unsupported
versions cleanly.

---

## 14. Procedural macros

Create only after the project/schema/ABI contracts are stable.

```rust
#[superquery::handler]
#[derive(SuperQueryEntity)]
```

Use `proc-macro2`, `syn`, `quote`.

The handler macro generates: the WASM export symbol, payload deserialization,
error conversion, ABI version assertion.

Keep business logic transparent. Avoid macro magic that makes mappings
impossible to debug.

---

## 15. CLI implementation sequence

Use `clap`.

**`superquery init`** — reference [`init.ts`](https://github.com/subquery/subql/blob/main/packages/cli/src/commands/init.ts).

```bash
superquery init my-indexer --chain evm
```

```text
my-indexer/
|-- Cargo.toml
|-- project.yaml
|-- schema.graphql
|-- abis/
`-- src/
    `-- lib.rs
```

**`superquery validate`** — manifest, schema, assets/ABIs, handler references,
compatibility versions. Fast; no PostgreSQL or RPC unless optional network
validation is explicitly requested.

**`superquery codegen`** — reference [`codegen/index.ts`](https://github.com/subquery/subql/blob/main/packages/cli/src/commands/codegen/index.ts).

```text
src/generated/
|-- entities.rs
|-- schema_metadata.rs
`-- contracts/
    `-- erc20.rs
```

Output must be deterministic. Same inputs = byte-for-byte same generated files.

**`superquery build`** — reference [`build.ts`](https://github.com/subquery/subql/blob/main/packages/cli/src/commands/build.ts).

```bash
cargo build --release --target wasm32-wasip1
```

```text
dist/
|-- manifest.json
|-- schema.graphql
|-- schema.ir.json
|-- mapping.wasm
|-- assets/
`-- build.json
```

```json
{
  "formatVersion": 1,
  "mappingAbiVersion": 1,
  "manifestHash": "...",
  "schemaHash": "...",
  "mappingHash": "..."
}
```

**`superquery test`** — start with mapping unit-test helpers, not an enormous
custom framework.

```rust
let event = fixture::evm_log(...);
let store = TestStore::new();

run_handler(handle_transfer, event, &store).await?;

assert_eq!(store.entity::<Transfer>("id"), ...);
```

Later add an integration mode against Anvil/Postgres.

**`superquery doctor`** — Rust version, WASM target installed, Docker
availability, manifest/schema parse, node/query compatibility, database/RPC
connectivity when requested. This saves users significant setup pain.

---

## 16. Templates

Start with one high-quality template.

```text
templates/evm/
|-- Cargo.toml
|-- project.yaml
|-- schema.graphql
|-- abis/
`-- src/lib.rs
```

First: **ERC-20 Transfers**. Second: **Uniswap V3 Swap events**. Only then add
Stellar/Solana templates.

---

## 17. Build bundle contract

The output from the SDK is the input to the node. That boundary must be boring
and stable.

```text
superquery-sdk
   |
   | `superquery build`
   v
dist/
   |
   | read-only bundle
   v
superquery-node
```

The node should never need the developer's source tree to run a compiled
project. This makes Docker deployment simpler, artifact hashing possible,
reproducible builds possible, and future decentralized deployment easier.

---

## 18. Web location in the three-repo model

Because the organization is exactly three repos, the website/dashboard lives
here:

```text
superquery-sdk/
|-- crates/
|-- templates/
|-- docs/
`-- web/
```

Why the SDK? The web surface is developer-facing: docs, project setup,
explorer/dashboard later, deployment UX later, SDK/CLI documentation.

Keep `web` separate from the Rust Cargo workspace through workspace exclusions.
If it becomes a huge independent hosted platform later, reconsider. Do not add
a fourth repo now.

---

## 19. Recommended first public API

Keep v0.1 small:

```rust
use superquery_sdk::prelude::*;

#[handler]
pub async fn handle_transfer(event: EvmLog<Transfer>) -> Result<()> {
    TransferEntity {
        id: event.id(),
        from: event.params.from.to_string(),
        to: event.params.to.to_string(),
        value: event.params.value.into(),
    }
    .save()
    .await
}
```

The hardest part is not the syntax. It is keeping SDK, mapping ABI, node store
and query schema synchronized.

---

## 20. Step-by-step milestones

| # | Milestone | Acceptance |
|---|---|---|
| 0 | Specification document | Node/query developers can implement consumers using only `docs/spec/*` |
| 1 | `superquery-types` | Scalars, entity values, project IDs, block height/hash, chain-family enum, mapping ABI version types. No CLI yet |
| 2 | manifest crate | Valid EVM fixture passes; invalid fixtures test every validation branch |
| 3 | schema parser + canonical IR | Parses entity/scalars/nullability/list/relation/index metadata; canonical serialization produces deterministic hash |
| 4 | entity codegen | Schema fixture generates compileable Rust entities; output stable across runs |
| 5 | EVM ABI codegen | ERC-20 ABI produces typed Transfer event; known log decodes to expected fields |
| 6 | guest mapping SDK | Mapping compiles to WASM without opening sockets/files/DB itself |
| 7 | macros | `#[handler]` exports exactly the symbol the runtime expects; panic/error returns a structured runtime error |
| 8 | CLI `init`, `validate`, `codegen` | `init demo --chain evm` → `validate` → `codegen` → `cargo check` works end-to-end |
| 9 | CLI `build` | Produces deterministic `dist/`; hashes all required components; compiled mapping is loadable by node |
| 10 | testing helpers | Developers can unit test mappings without live RPC/Postgres |
| 11 | web/docs | Quickstart, manifest reference, schema reference, mapping API, EVM events, local dev, troubleshooting. Do this once APIs stop changing daily |
| 12 | Stellar/Solana developer APIs | Only after the EVM project's full lifecycle is stable |

---

## 21. Rust dependency suggestions

```toml
serde
serde_json
serde_yaml
semver
url
clap
miette
thiserror
anyhow
graphql-parser
proc-macro2
syn
quote
alloy-json-abi
alloy-sol-types
alloy-primitives
sha2
hex
camino
```

Use `cargo_metadata` if the CLI needs structured Cargo workspace inspection.

---

## 22. Grant-friendly issue sequence

1. Define manifest v1 Rust types
2. Add manifest semantic validator
3. Define schema IR
4. Implement GraphQL entity schema parser
5. Implement deterministic schema canonicalization
6. Generate Rust entity structs
7. Add EVM ABI importer
8. Generate typed EVM events
9. Define Mapping ABI v1
10. Implement guest store host wrappers
11. Implement `#[handler]`
12. Add `superquery init`
13. Add `superquery validate`
14. Add `superquery codegen`
15. Add deterministic `superquery build`
16. Add mapping test harness
17. Add ERC-20 example
18. Add Uniswap V3 example
19. Add Stellar template
20. Add Solana template

---

## 23. Avoid these mistakes

**Do not duplicate manifest types in node.**

```text
Bad:                        Good:
sdk::ProjectManifest        superquery-manifest crate
node::ProjectManifest2            ^
                                  |-- CLI
                                  `-- Node
```

**Do not let codegen become a second schema parser.** Parse once into canonical
IR. Generate from IR.

**Do not make the CLI responsible for indexing.** CLI prepares/builds projects.
Node executes them.

**Do not compile mappings inside the node in production.** The node consumes
built artifacts. Development tooling may orchestrate a build, but runtime and
build concerns stay separable.

---

## 24. Cross-repository contract

SDK is the source of truth for:

```text
Manifest specification
Schema IR
Mapping ABI
Build artifact format
```

Node consumes those to execute. Query consumes schema metadata to expose read
APIs.

| SDK spec | Node | Query |
|---|---|---|
| manifest 1 / ABI 1 | >= 0.1 | >= 0.1 |

---

## 25. Source-license note

Use the linked SubQuery sources as architectural/behavioral references. Before
copying source text, generated templates or substantial code, verify the
license/notice requirements for the exact package/repository/version. Prefer
implementing Rust interfaces from documented behavior and tests rather than
line-by-line translation.
