# SuperQuery SDK — Implementation Plan

Working plan for `blockSuperquery/superquery-sdk`. Derived from
[`docs/sdk-implementation-guide.md`](docs/sdk-implementation-guide.md), which is
the design authority — if this plan and the guide disagree, the guide wins.

**Status as of the scaffold commit:** workspace compiles, 37 tests pass,
`superquery validate` and `superquery codegen --dry-run` work end-to-end
against `templates/evm`.

---

## Upstream references

SuperQuery is a Rust reimplementation of SubQuery's developer surface. These
are behavioural references, not code to translate line by line (see §25 of the
guide on licensing).

| Concern | Upstream |
|---|---|
| GitHub org | <https://github.com/subquery> |
| Monorepo | <https://github.com/subquery/subql> |
| CLI | <https://github.com/subquery/subql/tree/main/packages/cli> |
| CLI commands | <https://github.com/subquery/subql/tree/main/packages/cli/src/commands> |
| `init` | <https://github.com/subquery/subql/blob/main/packages/cli/src/commands/init.ts> |
| `build` | <https://github.com/subquery/subql/blob/main/packages/cli/src/commands/build.ts> |
| `codegen` | <https://github.com/subquery/subql/tree/main/packages/cli/src/commands/codegen> |
| `import-abi` | <https://github.com/subquery/subql/blob/main/packages/cli/src/commands/codegen/import-abi.ts> |
| Project model | <https://github.com/subquery/subql/tree/main/packages/common/src/project> |
| Project readers | <https://github.com/subquery/subql/tree/main/packages/common/src/project/readers> |
| Versioned projects | <https://github.com/subquery/subql/tree/main/packages/common/src/project/versioned> |
| Public types | <https://github.com/subquery/subql/tree/main/packages/types/src> |
| Core types | <https://github.com/subquery/subql/tree/main/packages/types-core> |
| EVM project config | <https://github.com/subquery/subql-ethereum/tree/main/packages/common-ethereum> |
| EVM mapping types | <https://github.com/subquery/subql-ethereum/tree/main/packages/types> |
| Stellar | <https://github.com/subquery/subql-stellar> |
| Solana | <https://github.com/subquery/subql-solana> |

---

## Crate topology

```text
superquery-types        scalars, entity values, IDs, block ptr, ABI version
      ^
      |-- superquery-manifest      project.yaml parse + semantic validation
      |-- superquery-schema        GraphQL -> canonical IR + hashing
      |-- superquery-macros        #[handler], #[derive(SuperQueryEntity)]
      |
      |-- superquery-chain-api     the ChainIntegration trait seam
      |     ^
      |     |-- superquery-evm         reference integration
      |     |-- superquery-stellar     reserved (Milestone 12)
      |     `-- superquery-solana      reserved (Milestone 12)
      |
      |-- superquery-codegen       IR -> deterministic Rust
      |-- superquery-sdk           the guest mapping API
      `-- superquery-cli           the `superquery` binary
```

Dependency direction is strictly downward. `superquery-types` depends on
nothing in the workspace; the CLI depends on everything. A cycle here means the
layering is wrong.

---

## Milestone status

| # | Milestone | State | Where |
|---|---|---|---|
| 0 | Specification documents | **stub** | `docs/spec/*.md` |
| 1 | `superquery-types` | **done** | `crates/types` |
| 2 | manifest crate | **mostly done** — needs fixture test suite | `crates/manifest` |
| 3 | schema parser + canonical IR | **mostly done** — needs relation/index test coverage | `crates/schema` |
| 4 | entity codegen | **partial** — structs generate; trait impls pending Milestone 7 | `crates/codegen` |
| 5 | EVM ABI codegen | **partial** — emits `sol!`; no round-trip decode test yet | `crates/chains/evm` |
| 6 | guest mapping SDK | **scaffold** — shape settled, host calls stubbed | `crates/sdk` |
| 7 | macros | **scaffold** — both macros are pass-through | `crates/macros` |
| 8 | CLI init/validate/codegen | **partial** — `validate` and `codegen` work; `init` stubbed | `crates/cli` |
| 9 | CLI `build` | **not started** | `crates/cli/src/commands/build.rs` |
| 10 | testing helpers | **scaffold** — `TestStore` only | `crates/sdk/src/testing.rs` |
| 11 | web/docs | **web exists**, docs pending | `web/`, `docs/` |
| 12 | Stellar/Solana | **reserved** | `crates/chains/{stellar,solana}` |

---

## What to build next, in order

### 1. Milestone 6/7 — close the mapping ABI loop

This is the critical path. Everything downstream (`build`, `test`, the node's
runtime) is blocked on the guest/host contract being real rather than stubbed.

- Write `docs/spec/mapping-abi-v1.md` first — the node team implements against
  it, so it has to exist before the code hardens.
- Replace the stubs in `crates/sdk/src/host/store.rs` with the actual
  `extern "C"` block, plus the pointer/length encoding for entity payloads.
- Make `#[handler]` generate the export symbol, the payload decode and the
  error conversion. Test the expansion, not just that it compiles.
- Make `#[derive(SuperQueryEntity)]` generate the `Entity` impl so the
  `.save()` in `templates/evm/src/lib.rs` actually resolves.

**Done when:** `templates/evm` compiles to `wasm32-wasip1` and exports
`sq_mapping_abi_version` plus one `sq_handle_*` symbol.

### 2. Milestone 8 — `superquery init`

Cheap once templates exist, and it unblocks anyone trying the SDK.

- Copy `templates/evm`, substituting the project name into `Cargo.toml` and
  `project.yaml`.
- Refuse to overwrite a non-empty directory.
- Print the next three commands.

### 3. Milestone 9 — deterministic `superquery build`

- Write `docs/spec/build-artifact-v1.md` first.
- Run codegen, then `cargo build --release --target wasm32-wasip1`.
- Assemble `dist/` and write `build.json` with the three hashes.
- **Test determinism explicitly**: build twice, assert byte-identical output.
  A build that is 99% reproducible is not reproducible.

### 4. Fixture-driven test suites (Milestones 2, 3, 5)

The validators are written but under-tested. Add `tests/fixtures/{valid,invalid}`
to the manifest and schema crates, one invalid fixture per validation branch,
and assert on the diagnostic that comes back — not just that it failed.

### 5. Milestone 10 — the test harness

`run_handler(handler, event, &store)` against `TestStore`, so a developer can
unit test a mapping with no RPC and no Postgres.

---

## Invariants

Things that are easy to break and expensive to unbreak.

**One manifest type.** `superquery-manifest` is the only definition of
`project.yaml`. The node consumes this crate; it does not define its own.

**One schema parser.** `graphql_parser` appears in `crates/schema` and nowhere
else. Codegen reads the IR. If codegen needs something the IR lacks, extend the
IR.

**Deterministic codegen.** No `HashMap` iteration, no timestamps, no absolute
paths in generated text. The build hashes depend on it, and reproducible builds
depend on the hashes. There is a test for this in
`crates/codegen/src/entities.rs`; keep it passing.

**The CLI does not index.** It prepares and builds projects. The node executes
them. The node consumes `dist/`, never the developer's source tree.

**Chain knowledge lives in chain crates.** Generic tooling must not grow
`match family {}` arms — that is what `ChainIntegration` is for. Adding Stellar
should mean writing one impl and touching nothing else. If it doesn't, the seam
is wrong.

**The ABI version is a promise.** Changing anything in `crates/sdk/src/host/`
means bumping `MAPPING_ABI_VERSION` in `crates/types`.

---

## Deliberate deviations from the guide

Recorded so they are choices rather than drift.

- **`serde_yaml_ng` instead of `serde_yaml`.** Upstream `serde_yaml` is
  archived and unmaintained; `serde_yaml_ng` is the maintained fork with the
  same API.
- **No `rust-toolchain.toml`.** Pinning a channel plus the `wasm32-wasip1`
  target triggers a full rustup component sync on contributors' machines.
  `superquery doctor` reports what is missing instead, which is the friendlier
  failure.
- **`web/` was moved, not recreated.** The Nuxt portal predates this scaffold
  and moved into `web/` with `git mv`, so its history is intact.
- **The types crate is `std`-only.** Mappings target `wasm32-wasip1`, which has
  `std`. A `no_std` feature nobody builds is a claim, not a capability.

---

## Cross-repo contract

The SDK is the source of truth for four things:

```text
Manifest specification   -> docs/spec/manifest-v1.md
Schema IR                -> docs/spec/schema-v1.md
Mapping ABI              -> docs/spec/mapping-abi-v1.md
Build artifact format    -> docs/spec/build-artifact-v1.md
```

`superquery-node` consumes all four to execute a project.
`superquery-query` consumes the schema IR to expose read APIs.

| SDK spec | Node | Query |
|---|---|---|
| manifest 1 / ABI 1 | >= 0.1 | >= 0.1 |
