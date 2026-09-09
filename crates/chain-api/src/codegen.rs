//! The shape of generated output.
//!
//! Chain integrations return files rather than writing them, so the CLI stays
//! in charge of where output lands and can diff before overwriting.

/// A module of generated code, e.g. one contract's bindings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedModule {
    /// Module name, used as the file stem, e.g. `erc20`.
    pub name: String,
    /// Files this module contributes.
    pub files: Vec<GeneratedFile>,
}

/// One generated file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedFile {
    /// Path relative to the codegen output root, e.g. `contracts/erc20.rs`.
    pub path: String,
    /// Complete file contents.
    ///
    /// Must be byte-for-byte identical for identical inputs — `superquery
    /// build` hashes generated output, and a nondeterministic generator would
    /// break reproducible builds.
    pub contents: String,
}
