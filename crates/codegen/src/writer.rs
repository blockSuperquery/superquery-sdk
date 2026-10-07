//! Collecting generated files and putting them on disk.
//!
//! Writing is separated from generating so codegen can be tested without a
//! filesystem, and so the CLI can diff or dry-run before touching a project.

use std::collections::{BTreeMap, BTreeSet};

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

    /// Add the `mod.rs` files that tie the set into one module tree, then
    /// sort by path so the set is canonical.
    ///
    /// Every directory gets a `mod.rs` declaring its files and
    /// subdirectories, so `mod generated;` in a mapping resolves no matter
    /// which chain integrations contributed files.
    pub fn finish(mut self) -> Self {
        let tree = self.module_tree();
        self.files.retain(|f| f.path.file_name() != Some("mod.rs"));
        self.files.extend(tree);
        self.files.sort_by(|a, b| a.path.cmp(&b.path));
        self
    }

    fn module_tree(&self) -> Vec<GeneratedFile> {
        // directory -> child module names; BTree for deterministic output.
        let mut dirs: BTreeMap<Utf8PathBuf, BTreeSet<String>> = BTreeMap::new();
        dirs.entry(Utf8PathBuf::new()).or_default();

        for file in self
            .files
            .iter()
            .filter(|f| f.path.extension() == Some("rs"))
        {
            let mut child = file.path.clone();
            let mut module = child.file_stem().unwrap_or_default().to_owned();
            while let Some(parent) = child.parent() {
                if module != "mod" {
                    dirs.entry(parent.to_owned()).or_default().insert(module);
                }
                module = parent.file_name().unwrap_or_default().to_owned();
                child = parent.to_owned();
                if child.as_str().is_empty() {
                    break;
                }
            }
        }

        dirs.into_iter()
            .map(|(dir, modules)| {
                let mut out = String::from(crate::GENERATED_HEADER);
                if dir.as_str().is_empty() {
                    out.push_str(
                        "//! Generated code. Re-run `superquery codegen` after changing\n",
                    );
                    out.push_str("//! `schema.graphql` or any ABI.\n\n");
                } else {
                    out.push('\n');
                }
                for module in modules {
                    out.push_str(&format!("pub mod {module};\n"));
                }
                GeneratedFile {
                    path: dir.join("mod.rs"),
                    contents: out,
                }
            })
            .collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    fn file(path: &str) -> GeneratedFile {
        GeneratedFile {
            path: path.into(),
            contents: String::new(),
        }
    }

    fn contents<'a>(set: &'a OutputSet, path: &str) -> &'a str {
        &set.files
            .iter()
            .find(|f| f.path == path)
            .expect(path)
            .contents
    }

    #[test]
    fn every_directory_gets_a_mod_rs_declaring_its_children() {
        let mut set = OutputSet::new();
        set.push(file("schema_metadata.rs"))
            .push(file("entities.rs"))
            .push(file("contracts/erc20.rs"))
            .push(file("contracts/pool.rs"));
        let set = set.finish();

        let root = contents(&set, "mod.rs");
        assert!(
            root.contains("pub mod contracts;\npub mod entities;\npub mod schema_metadata;\n"),
            "{root}"
        );
        let contracts = contents(&set, "contracts/mod.rs");
        assert!(
            contracts.ends_with("pub mod erc20;\npub mod pool;\n"),
            "{contracts}"
        );
    }

    #[test]
    fn an_empty_set_still_has_a_root_module() {
        let set = OutputSet::new().finish();
        assert_eq!(set.files.len(), 1);
        assert_eq!(set.files[0].path, "mod.rs");
    }

    #[test]
    fn finishing_twice_is_idempotent() {
        let mut set = OutputSet::new();
        set.push(file("entities.rs"))
            .push(file("contracts/erc20.rs"));
        let once = set.finish();
        assert_eq!(once.clone().finish(), once);
    }
}
