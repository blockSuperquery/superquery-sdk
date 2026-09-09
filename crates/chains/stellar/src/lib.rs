//! Stellar support — reserved, not yet implemented.
//!
//! The crate exists so the workspace topology is settled and so the manifest
//! can name `stellar` as a family that this build knowingly rejects, rather than
//! failing with an unrecognised-value error.
//!
//! Implementing it means writing one `ChainIntegration` against
//! `superquery-chain-api` and nothing else: no generic tooling should need to
//! change. That is the test of whether the seam is right.
//!
//! Milestone 12 of `docs/IMPLEMENTATION_PLAN.md`. Upstream reference:
//! <https://github.com/subquery/subql-stellar>.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use superquery_types::ChainFamily;

/// The family this crate will serve.
pub const FAMILY: ChainFamily = ChainFamily::Stellar;

/// Whether this build has a working Stellar integration.
///
/// Kept as a constant so `superquery doctor` can report the answer without
/// pretending the integration exists.
pub const IMPLEMENTED: bool = false;
