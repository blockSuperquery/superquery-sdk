//! The seam between chain-agnostic tooling and per-family knowledge.
//!
//! The CLI, codegen and manifest validation must not grow `match family {}`
//! arms. Instead each chain crate implements [`ChainIntegration`] and the
//! generic tooling asks it questions: is this datasource kind legal, what
//! assets does it need, what Rust does its ABI generate.
//!
//! Adding Stellar or Solana (Milestone 12) should mean writing one impl, not
//! editing the CLI.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod codegen;
pub mod integration;
pub mod registry;

pub use codegen::{GeneratedFile, GeneratedModule};
pub use integration::{ChainIntegration, ChainValidation, CodegenError, KindRole, KindSpec};
pub use registry::{Registry, UnsupportedFamily};
