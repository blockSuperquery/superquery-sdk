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

// `deny` rather than `forbid`: the WASM boundary in `host::wasm` is the one
// place that must handle raw pointers, and it opts back in explicitly.
#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod error;
mod executor;
pub mod host;
pub mod store;
pub mod testing;

pub use error::{Error, Result};
pub use superquery_types::{MAPPING_ABI_VERSION, MappingAbiVersion};

/// The shared value model. Generated code names these paths, so they are part
/// of the SDK's public surface.
pub use superquery_types as types;

#[cfg(feature = "evm")]
pub use alloy_sol_types::sol;

/// Paths the macros expand to. Not a public API: names here change whenever
/// the macros need them to.
#[doc(hidden)]
pub mod __private {
    pub use crate::error::{Error, Result};
    pub use superquery_types::{
        Entity as UntypedEntity, EntityId, FromValue, FromValueError, ToValue, Value,
    };

    /// Build an entity's id, naming the entity if it is empty.
    pub fn entity_id(entity: &str, id: &str) -> Result<EntityId> {
        EntityId::new(id).map_err(|err| Error::Entity {
            operation: "encode",
            entity: entity.to_owned(),
            message: err.to_string(),
        })
    }
}

/// What a mapping's `use superquery_sdk::prelude::*;` brings in.
pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::store::{Entity as EntityTrait, Store};
    pub use superquery_macros::{SuperQueryEntity, handler};
    pub use superquery_types::{BigDecimal, BigInt, Bytes, Json, Timestamp};

    #[cfg(feature = "evm")]
    pub use superquery_evm::prelude::*;
}
