# Manifest v1

`specVersion: "1.0"`. Implemented by `crates/manifest`.

Status: **v1** — this document and the Rust types agree. Every validation rule
below has a fixture under `crates/manifest/tests/fixtures/` that proves its
diagnostic; a rule without a fixture is not part of the spec.

## Document

```yaml
specVersion: "1.0"          # required; "1.0" or "1" (an unquoted 1.0 is tolerated)
name: erc20-transfers       # required, [a-z][a-z0-9-]*, <= 48 chars, no trailing `-`
version: "0.1.0"            # required, semver recommended
description: "..."          # optional
license: "Apache-2.0"       # optional, SPDX

network:                    # required
  family: evm               # evm (alias: ethereum) | stellar | solana
  chainId: "1"              # required, string; per-family meaning
  endpoint:                 # optional; a string or a list of strings (URLs)
    - "https://..."
  dictionary: "https://..." # optional URL
  chaintypes: "./types.json" # optional path, for families that need one

startBlock: 21000000        # optional, default for all datasources (else 0)

schema:                     # required
  file: "./schema.graphql"  # relative to the manifest

dataSources:                # required, non-empty
  - kind: evm/Runtime       # <family>/<Kind>
    name: usdc              # optional; unique if set
    startBlock: 21000000    # optional; overrides the project default
    endBlock: 21100000      # optional; must be >= the effective startBlock
    options:                # family-specific, see below
      address: "0x..."
    assets:                 # optional; name -> file, relative to the manifest
      erc20:
        file: "./abis/ERC20.json"
    handlers:               # required, non-empty
      - handler: handle_transfer
        kind: evm/LogHandler
        filter:             # optional; shape must match the handler kind
          event: "Transfer(address,address,uint256)"
```

**Unknown keys are errors**, at every level. A misspelled `startblock` fails
parsing with `unknown field` rather than being silently ignored.

The effective start block of a datasource is its own `startBlock`, else the
project `startBlock`, else `0`.

## Filters

A filter is recognised by its fields; there is no explicit discriminator.

| Shape | Fields | Valid on handler kinds ending in |
|---|---|---|
| log | `event` (required), `topics` (list, positional after topic0) | `LogHandler`, `EventHandler` |
| transaction | `function` (required), `from`, `to` | `TransactionHandler`, `CallHandler` |
| block | `modulo`, `timestamp` (both optional) | `BlockHandler` |

A filter of the wrong shape for its handler kind would never match, so it is an
error rather than a no-op.

## Two passes

Parsing and validation are separate. A manifest that parses may still be
rejected, which is what lets each error name an exact field path such as
`dataSources[0].handlers[1].kind`. Validation reports every finding in one pass
rather than stopping at the first.

### Parse errors

| Problem | Message contains |
|---|---|
| unsupported `specVersion` | ``unsupported specVersion `2.0` (supported: 1.0)`` |
| unknown `network.family` | ``unknown variant `bitcoin` `` |
| unknown key | ``unknown field `...` `` |
| missing required key | ``missing field `...` `` |

Parse errors carry a source span covering the offending line.

### Generic validation rules

| Rule | Field | Severity |
|---|---|---|
| `name` is a valid project id | `name` | error |
| `version` is semver | `version` | warning |
| `network.family` is implemented in this build | `network.family` | error |
| `network.chainId` is non-empty | `network.chainId` | error |
| EVM `network.chainId` is a decimal number | `network.chainId` | error |
| `network.endpoint` is non-empty | `network.endpoint` | warning |
| `schema.file` exists | `schema.file` | error |
| `dataSources` is non-empty | `dataSources` | error |
| datasource names are unique | `dataSources[i].name` | error |
| datasource `kind` carries the family's prefix | `dataSources[i].kind` | error |
| `endBlock` >= effective start block | `dataSources[i].endBlock` | error |
| declared assets exist on disk | `dataSources[i].assets.<name>.file` | error |
| each datasource has a handler | `dataSources[i].handlers` | error |
| handler name is non-empty | `dataSources[i].handlers[j].handler` | error |
| the same handler name appears once | `dataSources[i].handlers[j].handler` | warning |
| handler `kind` carries the family's prefix | `dataSources[i].handlers[j].kind` | error |
| filter shape matches handler kind | `dataSources[i].handlers[j].filter` | error |

Filesystem rules (`schema.file`, assets) are skipped when validating a manifest
that was not read from disk.

### EVM rules

Run after the generic rules, by `ChainIntegration::validate` in
`crates/chains/evm`.

| Rule | Field | Severity |
|---|---|---|
| datasource kind is `evm/Runtime` | `dataSources[i].kind` | error |
| `options.address` parses as a 20-byte address | `dataSources[i].options.address` | error |
| `options.address` is set | `dataSources[i].options.address` | warning |
| handler kind is `evm/LogHandler`, `evm/TransactionHandler` or `evm/BlockHandler` | `dataSources[i].handlers[j].kind` | error |
| a log filter's `event` is a full signature, not a bare name | `dataSources[i].handlers[j].filter.event` | error |

## Diagnostics

Every finding carries a field path, a message, an optional `help` line with a
concrete fix, and a severity. `superquery validate` exits non-zero if and only if
at least one error-severity finding exists.

## Out of scope for v1

- Multi-chain projects (upstream `packages/common/src/multichain`). Whether
  they extend this format or sit beside it is undecided.
- Templated datasources (dynamic contract discovery).
- Network validation (`superquery validate --network`), which probes endpoints
  and compares `chainId`. It is optional and does not change what a valid
  manifest is.
