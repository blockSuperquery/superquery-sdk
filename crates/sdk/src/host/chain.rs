//! Host calls for chain context: `sq_chain_*`.
//!
//! A handler is told which block it is running for rather than asking an RPC
//! endpoint, so mappings stay deterministic and replayable.

use superquery_types::BlockPtr;

use crate::error::{Error, Result};
use crate::host::raw;

/// Import name for the current-block call.
pub const IMPORT_CURRENT_BLOCK: &str = "sq_chain_current_block";

/// The block the running handler was invoked for.
pub fn current_block() -> Result<BlockPtr> {
    let host_error = |message: String| Error::Host {
        call: IMPORT_CURRENT_BLOCK,
        message,
    };
    let bytes =
        raw::current_block()?.ok_or_else(|| host_error("no block is being processed".into()))?;
    serde_json::from_slice(&bytes).map_err(|e| host_error(format!("malformed block pointer: {e}")))
}
