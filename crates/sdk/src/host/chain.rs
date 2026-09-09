//! Host calls for chain context: `sq_chain_*`.
//!
//! A handler is told which block it is running for rather than asking an RPC
//! endpoint, so mappings stay deterministic and replayable.

/// Import name for the current-block call.
pub const IMPORT_CURRENT_BLOCK: &str = "sq_chain_current_block";
