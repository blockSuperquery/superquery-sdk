//! Codegen errors.

use camino::Utf8PathBuf;
use thiserror::Error;

/// Result alias for codegen operations.
pub type CodegenResult<T> = Result<T, CodegenError>;

/// Something went wrong producing or writing generated code.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CodegenError {
    /// A generated file could not be written.
    #[error("could not write generated file `{path}`")]
    Write {
        /// The path that failed.
        path: Utf8PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// The IR contained something codegen cannot express in Rust.
    #[error("cannot generate code for {location}: {message}")]
    Unsupported {
        /// Where in the schema, e.g. `Transfer.value`.
        location: String,
        /// What could not be generated.
        message: String,
    },

    /// A chain integration failed while generating its bindings.
    #[error(transparent)]
    Chain(#[from] superquery_chain_api::CodegenError),
}
