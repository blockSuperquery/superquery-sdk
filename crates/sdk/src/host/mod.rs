//! The guest side of the mapping ABI.
//!
//! Every function here corresponds to a WASM import the node provides. The
//! guest declares them; `superquery-node` supplies them. Keeping the guest
//! declarations in this repo — not in the node — is what makes the ABI a
//! published contract rather than an implementation detail.
//!
//! Two layers. `raw` moves bytes: on `wasm32` through the `superquery` import
//! module, elsewhere through a thread-local [`native::HostBackend`]. The typed
//! modules above it ([`store`], [`log`], [`chain`]) own the JSON encoding and
//! the status codes, so both targets share them.
//!
//! See `docs/spec/mapping-abi-v1.md`. Changing anything here means bumping
//! [`superquery_types::MAPPING_ABI_VERSION`].

pub mod abi;
pub mod chain;
pub mod log;
pub mod store;

#[cfg(target_arch = "wasm32")]
#[allow(unsafe_code)]
pub mod wasm;
#[cfg(target_arch = "wasm32")]
pub(crate) use wasm as raw;

#[cfg(not(target_arch = "wasm32"))]
pub mod native;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use native as raw;
