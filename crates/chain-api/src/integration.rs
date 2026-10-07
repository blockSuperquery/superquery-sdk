//! What a chain crate must provide.

use camino::Utf8Path;
use superquery_manifest::{DataSource, ProjectManifest};
use superquery_types::ChainFamily;

use crate::codegen::GeneratedModule;
use crate::template::ProjectTemplate;

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

    /// The starter project `superquery init` writes, if this family has one.
    fn template(&self) -> Option<&'static ProjectTemplate> {
        None
    }
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
/// Deliberately its own type rather than the manifest crate's `Diagnostic`:
/// the manifest crate sits below this one, and the CLI is the only place the
/// two kinds of finding need to meet.
#[derive(Debug, Clone, Default)]
pub struct ChainValidation {
    /// Blocking findings.
    pub errors: Vec<ChainFinding>,
    /// Non-blocking findings.
    pub warnings: Vec<ChainFinding>,
}

impl ChainValidation {
    /// Record a blocking finding. Chain `.with_help(..)` to add a fix.
    pub fn error(
        &mut self,
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> &mut ChainFinding {
        self.errors.push(ChainFinding::new(field, message));
        self.errors.last_mut().expect("just pushed")
    }

    /// Record a non-blocking finding. Chain `.with_help(..)` to add a fix.
    pub fn warn(
        &mut self,
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> &mut ChainFinding {
        self.warnings.push(ChainFinding::new(field, message));
        self.warnings.last_mut().expect("just pushed")
    }

    /// Whether any blocking finding was recorded.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

/// One family-specific finding, tied to a manifest field path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainFinding {
    /// Dotted path to the offending field, e.g. `dataSources[0].options.address`.
    pub field: String,
    /// What is wrong.
    pub message: String,
    /// How to fix it, when there is a concrete suggestion.
    pub help: Option<String>,
}

impl ChainFinding {
    /// A finding with no fix suggestion yet.
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            help: None,
        }
    }

    /// Attach a fix suggestion.
    pub fn with_help(&mut self, help: impl Into<String>) -> &mut Self {
        self.help = Some(help.into());
        self
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
