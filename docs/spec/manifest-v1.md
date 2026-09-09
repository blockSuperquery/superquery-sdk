# Manifest v1

`specVersion: "1.0"`. Implemented by `crates/manifest`.

Status: **draft** — the Rust types are authoritative; this document trails them.

## Document

```yaml
specVersion: "1.0"          # required, string
name: erc20-transfers       # required, [a-z][a-z0-9-]*, <= 48 chars
version: "0.1.0"            # required, semver recommended
description: "..."          # optional
license: "Apache-2.0"       # optional, SPDX

network:                    # required
  family: evm               # evm | stellar | solana
  chainId: "1"              # required, string; per-family meaning
  endpoint:                 # optional; string or list of strings
    - "https://..."
  dictionary: "https://..." # optional
  chaintypes:               # optional, path
    file: "./types.json"

startBlock: 21000000        # optional, default for all datasources

schema:                     # required
  file: "./schema.graphql"

dataSources:                # required, non-empty
  - kind: evm/Runtime       # <family>/<Kind>
    name: usdc              # optional; unique if set
    startBlock: 21000000    # optional; overrides project default
    endBlock: 21100000      # optional
    options:                # family-specific
      address: "0x..."
    assets:                 # optional
      erc20:
        file: "./abis/ERC20.json"
    handlers:               # required, non-empty
      - handler: handle_transfer
        kind: evm/LogHandler
        filter:
          event: "Transfer(address,address,uint256)"
```

## Validation rules

Parsing and validation are separate passes: a manifest that parses may still be
rejected, which is what lets errors name an exact field.

| Rule | Severity |
|---|---|
| `specVersion` is a supported version | error (parse) |
| `name` is a valid project id | error |
| `version` is semver | warning |
| `network.family` is implemented in this build | error |
| `network.chainId` is non-empty, and numeric for EVM | error |
| `network.endpoint` is non-empty | warning |
| `schema.file` exists | error |
| `dataSources` is non-empty | error |
| datasource `kind` carries the declared family's prefix | error |
| datasource names are unique | error |
| `endBlock >= startBlock` | error |
| declared assets exist on disk | error |
| each datasource has at least one handler | error |
| handler `kind` carries the family's prefix | error |
| filter shape matches handler kind | error |
| the same handler name appears once | warning |

Family-specific rules run after these — see `ChainIntegration::validate`. For
EVM: `options.address` parses as an address, and a log filter's `event:` is a
signature rather than a bare name.

## Open questions

- Multi-chain projects (upstream `packages/common/src/multichain`) are out of
  scope for v1. Decide whether they extend this format or sit beside it.
- Templated datasources (dynamic contract discovery) are not in v1.
