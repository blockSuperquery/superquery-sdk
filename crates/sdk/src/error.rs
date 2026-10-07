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

    /// The handler payload did not decode into the handler's input type.
    ///
    /// The host routed something this handler cannot accept; reported as
    /// handler status `2` rather than as a mapping failure.
    #[error("could not decode handler payload: {0}")]
    Payload(String),

    /// The mapping itself rejected the input.
    #[error("{0}")]
    Mapping(String),
}

// A value that will not encode is a bug in the mapping's input handling, so it
// surfaces as a mapping error carrying the offending text.
impl From<superquery_types::InvalidDecimal> for Error {
    fn from(err: superquery_types::InvalidDecimal) -> Self {
        Self::Mapping(err.to_string())
    }
}

impl Error {
    /// Build a mapping-authored error.
    pub fn mapping(message: impl Into<String>) -> Self {
        Self::Mapping(message.into())
    }
}
