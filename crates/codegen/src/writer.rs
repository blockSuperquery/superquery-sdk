//! Collecting generated files and putting them on disk.
//!
//! Writing is separated from generating so codegen can be tested without a
//! filesystem, and so the CLI can diff or dry-run before touching a project.

use camino::{Utf8Path, Utf8PathBuf};

use crate::error::{CodegenError, CodegenResult};

/// One generated file, path relative to the output root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedFile {
    /// Relative path, e.g. `contracts/erc20.rs`.
    pub path: Utf8PathBuf,
    /// Complete contents.
    pub contents: String,
}

/// Everything one `superquery codegen` run produces.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OutputSet {
    /// The files, sorted by path so writes happen in a stable order.
    pub files: Vec<GeneratedFile>,
}

impl OutputSet {
    /// An empty set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a file.
    pub fn push(&mut self, file: GeneratedFile) -> &mut Self {
        self.files.push(file);
        self
    }

    /// Sort by path, making the set canonical.
    pub fn finish(mut self) -> Self {
        self.files.sort_by(|a, b| a.path.cmp(&b.path));
        self
    }

    /// Emit the `mod` declarations tying the generated files together.
    pub fn module_root(&self) -> GeneratedFile {
        let mut out = String::from(crate::GENERATED_HEADER);
        out.push_str("//! Generated code. Re-run `superquery codegen` after changing\n");
        out.push_str("//! `schema.graphql` or any ABI.\n\n");

        // Files sitting directly in the output root become `pub mod` lines;
        // anything nested is covered by its own directory module below.
        let mut top_level: Vec<&str> = self
            .files
            .iter()
            .filter(|f| f.path.components().count() == 1)
            .filter_map(|f| f.path.file_stem())
            .collect();
        top_level.sort_unstable();
        top_level.dedup();

        for module in top_level {
            out.push_str(&format!("pub mod {module};\n"));
        }
        if self.files.iter().any(|f| f.path.starts_with("contracts")) {
            out.push_str("pub mod contracts;\n");
        }

        GeneratedFile {
            path: "mod.rs".into(),
            contents: out,
        }
    }
}

/// Write an output set beneath `root`, creating directories as needed.
///
/// Files are only rewritten when their contents change, so `superquery
/// codegen` does not invalidate every downstream build artifact on a no-op run.
pub fn write_output_set(root: &Utf8Path, set: &OutputSet) -> CodegenResult<Vec<Utf8PathBuf>> {
    let mut written = Vec::new();

    for file in &set.files {
        let path = root.join(&file.path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| CodegenError::Write {
                path: parent.to_owned(),
                source,
            })?;
        }

        if std::fs::read_to_string(&path).is_ok_and(|existing| existing == file.contents) {
            continue;
        }

        std::fs::write(&path, &file.contents).map_err(|source| CodegenError::Write {
            path: path.clone(),
            source,
        })?;
        written.push(path);
    }

    Ok(written)
}
