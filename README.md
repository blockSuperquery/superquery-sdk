<div align="center">

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./web/public/superquery-wordmark-dark.svg">
  <img alt="SuperQuery" src="./web/public/superquery-wordmark.svg" width="440">
</picture>

### Query at super speed.

##### The SDK · CLI · Web Portal

**`RUST-NATIVE` · `SUBQUERY COMPATIBLE`**

Everything an indexer developer uses to define, generate, build and validate a
SuperQuery project — plus the developer portal.

[Live site](https://superquery.vercel.app/) ·
[SuperQuery core](https://github.com/blockSuperquery)

</div>

---

## What this repository is

`superquery-sdk` is the developer-facing contract of the SuperQuery platform:
the project manifest, the schema IR, the mapping SDK, the code generator and
the `superquery` CLI. It is one of exactly three repositories.

| Repository | Owns |
|---|---|
| **superquery-sdk** | manifest, schema IR, mapping ABI, build format, CLI, web |
| superquery-node | the RPC indexing loop, reorg handling, the entity store |
| superquery-query | the PostgreSQL-backed GraphQL read API |

The SDK is the source of truth for the manifest specification, the schema IR,
the mapping ABI and the build artifact format. The other two consume them.

## Layout

```text
crates/
  types/          scalars, entity values, IDs, block pointers, ABI version
  manifest/       project.yaml parsing + semantic validation
  schema/         GraphQL -> canonical IR + deterministic hashing
  codegen/        IR -> deterministic Rust
  macros/         #[handler], #[derive(SuperQueryEntity)]
  chain-api/      the ChainIntegration trait seam
  chains/evm/     the reference chain integration
  chains/stellar/ reserved
  chains/solana/  reserved
  sdk/            the guest mapping API
  cli/            the `superquery` binary

templates/evm/    the ERC-20 Transfers starter project
docs/spec/        the four cross-repo specifications
web/              the Nuxt developer portal (its own pnpm workspace)
```

## Quick start

```bash
cargo test --workspace

# Check the example project — offline, no RPC, no database.
cargo run -p superquery-cli -- validate -m templates/evm/project.yaml

# See what codegen would write.
cargo run -p superquery-cli -- codegen --dry-run -m templates/evm/project.yaml
```

Building a mapping needs the WASM target:

```bash
rustup target add wasm32-wasip1
```

`cargo run -p superquery-cli -- doctor` reports what is missing.

## The CLI

| Command | State |
|---|---|
| `superquery validate` | works — manifest, schema, assets, filters |
| `superquery codegen` | works — entities, metadata, contract bindings |
| `superquery doctor` | works — toolchain and project checks |
| `superquery init` | scaffold (Milestone 8) |
| `superquery build` | scaffold (Milestone 9) |
| `superquery test` | scaffold (Milestone 10) |

## A mapping

```rust
use superquery_sdk::prelude::*;

#[handler]
pub async fn handle_transfer(event: EvmLog<Transfer>) -> Result<()> {
    TransferEntity {
        id: event.id(),
        from: event.params.from.to_string(),
        to: event.params.to.to_string(),
        value: BigInt::new(event.params.value.to_string()),
    }
    .save()
    .await
}
```

A mapping compiles to WASM. It never opens a socket, a file or a database:
every effect goes through a host call the node implements.

## Web portal

The Nuxt site is a separate pnpm workspace, excluded from the Cargo workspace.

```bash
cd web
pnpm install
pnpm dev
```

## Contributing

Read [`.claude/IMPLEMENTATION_PLAN.md`](.claude/IMPLEMENTATION_PLAN.md) for
milestone status and what to build next, and
[`.claude/docs/sdk-implementation-guide.md`](.claude/docs/sdk-implementation-guide.md)
for the design authority.

Dependency direction runs strictly downward, from `types` to `cli`. Unfinished
work returns an error naming its milestone rather than panicking.

## License

See [LICENSE](LICENSE).
