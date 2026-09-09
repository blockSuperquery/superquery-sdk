//! EVM support: manifest kinds, mapping payload types and ABI codegen.
//!
//! This is the reference chain integration. Stellar and Solana are expected to
//! mirror its shape, so prefer fixing the seam in `superquery-chain-api` over
//! adding EVM-shaped special cases to generic tooling.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod abi;
pub mod integration;
pub mod kinds;
pub mod types;

pub use integration::EvmIntegration;
pub use kinds::{DATA_SOURCE_KINDS, HANDLER_KINDS};
pub use types::{EvmBlock, EvmLog, EvmTransaction};

/// Types a mapping author writes against.
pub mod prelude {
    pub use crate::types::{EvmBlock, EvmLog, EvmTransaction};
    pub use alloy_primitives::{Address, B256, I256, U256};
}
