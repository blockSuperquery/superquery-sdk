//! Schema errors.

use camino::Utf8PathBuf;
use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

/// Result alias for schema operations.
pub type SchemaResult<T> = Result<T, SchemaError>;

/// Something went wrong reading or resolving a schema.
#[derive(Debug, Error, Diagnostic)]
#[non_exhaustive]
pub enum SchemaError {
    /// The schema file could not be read.
    #[error("could not read schema at `{path}`")]
    #[diagnostic(code(superquery::schema::unreadable))]
    Unreadable {
        /// The path that failed.
        path: Utf8PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// The GraphQL document did not parse.
    #[error("{message}")]
    #[diagnostic(code(superquery::schema::syntax))]
    Syntax {
        /// The parser's message.
        message: String,
        /// The schema source, for span rendering.
        #[source_code]
        src: NamedSource<String>,
        /// Where the parser stopped.
        #[label("here")]
        span: Option<SourceSpan>,
    },

    /// The document parsed but is not a valid entity schema.
    #[error("{message}")]
    #[diagnostic(code(superquery::schema::invalid))]
    Invalid {
        /// What is wrong.
        message: String,
        /// Where in the schema, e.g. `Transfer.value`.
        location: String,
        /// How to fix it.
        #[help]
        help: Option<String>,
    },
}

impl SchemaError {
    /// Build an [`SchemaError::Invalid`] for a named location.
    pub fn invalid(location: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Invalid {
            message: message.into(),
            location: location.into(),
            help: None,
        }
    }

    /// Attach a fix suggestion to an [`SchemaError::Invalid`].
    pub fn with_help(mut self, text: impl Into<String>) -> Self {
        if let SchemaError::Invalid { help, .. } = &mut self {
            *help = Some(text.into());
        }
        self
    }
}
