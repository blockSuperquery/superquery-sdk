//! Starter projects for `superquery init`.
//!
//! Each chain integration ships its own template, so `init` needs no
//! knowledge of any family: it asks the registry and writes what comes back.

/// A project template, embedded in the binary.
#[derive(Debug, Clone, Copy)]
pub struct ProjectTemplate {
    /// The project name the template's files are written with. `init`
    /// replaces it in the `name` entries of `Cargo.toml` and `project.yaml`.
    pub placeholder_name: &'static str,
    /// Every file, path relative to the project root.
    pub files: &'static [TemplateFile],
}

/// One file of a [`ProjectTemplate`].
#[derive(Debug, Clone, Copy)]
pub struct TemplateFile {
    /// Path relative to the project root, `/`-separated.
    pub path: &'static str,
    /// Contents, written verbatim apart from the name substitution.
    pub contents: &'static str,
}
