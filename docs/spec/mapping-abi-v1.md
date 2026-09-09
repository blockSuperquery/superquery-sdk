# Mapping ABI v1

The contract between a compiled `mapping.wasm` and the node runtime hosting it.

Status: **stub** — write this before implementing Milestone 6. The node team
implements against this document, so it needs to exist before the code hardens.

`MAPPING_ABI_VERSION = 1` (`crates/types/src/abi_version.rs`).

## What must be specified

**Guest exports**

- `sq_mapping_abi_version() -> u32` — read before anything else, so an
  incompatible bundle is refused with a message rather than a trap.
- `sq_handle_<name>(ptr, len) -> i32` — one per `#[handler]`.
- Allocator exports the host needs to pass payloads in.

**Host imports** (declared in `crates/sdk/src/host/`)

| Import | Purpose |
|---|---|
| `sq_store_set` | Insert or replace an entity |
| `sq_store_get` | Read an entity by key |
| `sq_store_remove` | Delete an entity |
| `sq_log` | Emit a log line carrying block and handler context |
| `sq_chain_current_block` | The block this handler is running for |

**Still to decide**

- Payload encoding across the boundary. JSON is the obvious first answer and
  the obvious thing to regret; measure before committing.
- Memory ownership: who allocates, who frees, and what happens on a trap.
- Error propagation: how a handler's `Err` reaches the node with its message
  intact.
- Panic behaviour: a panicking mapping must abort its block, not the process.

## Compatibility

A host implementing version *N* runs guests built against *<= N*. A guest from
the future is always rejected. Anything that changes the imports, the exports
or the encoding is a version bump.

## Transactional guarantee

Store writes are transactional per block: either every mapping in a block
commits or none does. That is the node's guarantee, but it is why `save()`
cannot fail independently of the block it runs in.
