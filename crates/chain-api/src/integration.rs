//! What a chain crate must provide.

use camino::Utf8Path;
use superquery_manifest::{DataSource, ProjectManifest};
use superquery_types::ChainFamily;

use crate::codegen::GeneratedModule;

/// One chain family's developer-facing knowledge.
pub trait ChainIntegration: Send + Sync {
    /// Which family this integration serves.
    fn family(&self) -> ChainFamily;

    /// The datasource and handler kinds this family accepts.
    fn kinds(&self) -> &[KindSpec];

    /// Family-specific manifest checks, run after the generic ones.
    ///
    /// Generic validation already checked that kinds carry the right family
    /// prefix; this is where an EVM integration checks that `options.address`
    /// is a real address and that a log handler's `event:` signature parses.
    fn validate(&self, manifest: &ProjectManifest) -> ChainValidation;

    /// Generate typed bindings for a datasource's assets, e.g. ABI events.
    ///
    /// `base_dir` is the project root, for resolving asset paths.
    fn codegen(
        &self,
        manifest: &ProjectManifest,
        data_source: &DataSource,
        base_dir: &Utf8Path,
    ) -> Result<Vec<GeneratedModule>, CodegenError>;
}

/// One legal `<family>/<Kind>` value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KindSpec {
    /// The full kind string, e.g. `evm/LogHandler`.
    pub kind: &'static str,
    /// Whether this names a datasource or a handler.
    pub role: KindRole,
    /// One-line description, surfaced by `superquery validate` when a kind is
    /// misspelled.
    pub summary: &'static str,
}

/// Whether a kind names a datasource or a handler.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KindRole {
    /// A `dataSources[].kind` value.
    DataSource,
    /// A `handlers[].kind` value.
    Handler,
}

/// Findings from a family-specific validation pass.
///
/// Deliberately a plain list of `(field, message)` so this crate does not have
/// to depend on the manifest crate's diagnostic type in both directions.
#[derive(Debug, Clone, Default)]
pub struct ChainValidation {
    /// Blocking findings, as `(field path, message)`.
    pub errors: Vec<(String, String)>,
    /// Non-blocking findings.
    pub warnings: Vec<(String, String)>,
}

impl ChainValidation {
    /// Record a blocking finding.
    pub fn error(&mut self, field: impl Into<String>, message: impl Into<String>) {
        self.errors.push((field.into(), message.into()));
    }

    /// Record a non-blocking finding.
    pub fn warn(&mut self, field: impl Into<String>, message: impl Into<String>) {
        self.warnings.push((field.into(), message.into()));
    }
}

/// A chain integration could not generate bindings.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CodegenError {
    /// An asset file was missing or unreadable.
    #[error("could not read asset `{path}`")]
    Asset {
        /// The path that failed.
        path: String,
        /// The underlying error.
        #[source]
        source: std::io::Error,
    },

    /// An asset parsed but was not usable, e.g. a malformed ABI.
    #[error("{0}")]
    Invalid(String),
}
