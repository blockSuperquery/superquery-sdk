//! Mapping errors.
//!
//! A handler returning `Err` aborts the block, so these are deliberately
//! coarse: the node needs to know *that* the mapping failed and be able to
//! report *why* to an operator, not to recover.

use thiserror::Error;

/// Result alias used by every handler.
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

/// Something went wrong inside a mapping.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// A host call was rejected.
    #[error("host call `{call}` failed: {message}")]
    Host {
        /// Which host function.
        call: &'static str,
        /// The host's message.
        message: String,
    },

    /// An entity could not be encoded or decoded.
    #[error("could not {operation} entity `{entity}`: {message}")]
    Entity {
        /// `encode` or `decode`.
        operation: &'static str,
        /// The entity type name.
        entity: String,
        /// What went wrong.
        message: String,
    },

    /// The mapping itself rejected the input.
    #[error("{0}")]
    Mapping(String),
}

impl Error {
    /// Build a mapping-authored error.
    pub fn mapping(message: impl Into<String>) -> Self {
        Self::Mapping(message.into())
    }
}
