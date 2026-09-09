//! The mapping SDK — the guest half of the SuperQuery runtime.
//!
//! A mapping is a WASM module. It never opens a socket, a file or a database;
//! everything it does to the outside world goes through a host call that
//! `superquery-node` implements. This crate is the safe wrapper around those
//! calls, plus the traits generated entity code implements.
//!
//! ```ignore
//! use superquery_sdk::prelude::*;
//!
//! #[handler]
//! pub async fn handle_transfer(event: EvmLog<Transfer>) -> Result<()> {
//!     Transfer {
//!         id: event.id(),
//!         from: event.params.from.to_string(),
//!         to: event.params.to.to_string(),
//!         value: event.params.value.into(),
//!     }
//!     .save()
//!     .await
//! }
//! ```
//!
//! The store writes a handler performs are transactional per block: either the
//! whole block's mappings commit or none of them do. That is the node's
//! guarantee, but it is why `save()` cannot fail independently of the block.
//!
//! Milestone 6 of `docs/IMPLEMENTATION_PLAN.md`.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod error;
pub mod host;
pub mod store;
pub mod testing;

pub use error::{Error, Result};
pub use superquery_types::{MAPPING_ABI_VERSION, MappingAbiVersion};

#[cfg(feature = "evm")]
pub use alloy_sol_types::sol;

/// What a mapping's `use superquery_sdk::prelude::*;` brings in.
pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::store::{Entity as EntityTrait, Store};
    pub use superquery_macros::{SuperQueryEntity, handler};
    pub use superquery_types::{BigDecimal, BigInt, Bytes, Json, Timestamp};

    #[cfg(feature = "evm")]
    pub use superquery_evm::prelude::*;
}
