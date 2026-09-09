# SuperQuery SDK

Rust workspace for the developer-facing half of the SuperQuery platform:
manifest, schema IR, codegen, mapping SDK, macros and the `superquery` CLI.
The Nuxt web portal lives in `web/`, outside the Cargo workspace.

## Read first

- [`.claude/IMPLEMENTATION_PLAN.md`](.claude/IMPLEMENTATION_PLAN.md) — milestone
  status, what to build next, and the invariants that are expensive to break.
- [`.claude/docs/sdk-implementation-guide.md`](.claude/docs/sdk-implementation-guide.md)
  — the design authority. If the plan and the guide disagree, the guide wins.

## Commands

```bash
cargo check --workspace --all-targets   # fast feedback
cargo test --workspace                  # everything
cargo clippy --workspace --all-targets  # lints
cargo fmt --all                         # 100-col, edition 2024

cargo run -p superquery-cli -- validate -m templates/evm/project.yaml
cargo run -p superquery-cli -- codegen --dry-run -m templates/evm/project.yaml

cd web && pnpm install && pnpm dev      # the portal
```

## Layering

Dependencies run strictly downward: `types` <- everything, `cli` -> everything.
A cycle means the layering is wrong, not that a `pub use` is missing.

## Conventions

- Comments explain *why*, not *what*. The code says what it does.
- Test names are sentences describing the behaviour being pinned
  (`rejects_shapes_that_break_downstream_identifiers`), not
  `test_project_id_2`.
- Unimplemented work returns a real error naming its milestone, never
  `todo!()`. A scaffold should fail legibly, not panic.
- New validation gets a fixture proving the diagnostic, not just the failure.

## Invariants

One manifest type, one schema parser, deterministic codegen, no chain-specific
`match` arms in generic tooling, and a mapping-ABI version bump whenever
`crates/sdk/src/host/` changes. The plan explains each; break them knowingly.
