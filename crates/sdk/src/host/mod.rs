//! The guest side of the mapping ABI.
//!
//! Every function here corresponds to a WASM import the node provides. The
//! guest declares them; `superquery-node` supplies them. Keeping the guest
//! declarations in this repo — not in the node — is what makes the ABI a
//! published contract rather than an implementation detail.
//!
//! See `docs/spec/mapping-abi-v1.md`. Bumping anything here means bumping
//! [`superquery_types::MAPPING_ABI_VERSION`].
//!
//! Scaffold: the module layout and signatures are settled; the extern
//! declarations and their encoding land in Milestone 6.

pub mod abi;
pub mod chain;
pub mod log;
pub mod store;
