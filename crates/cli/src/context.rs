//! Locating the project a command should operate on.

use camino::{Utf8Path, Utf8PathBuf};
use superquery_chain_api::Registry;
use superquery_evm::EvmIntegration;
use superquery_manifest::ProjectManifest;

/// A loaded project: its manifest and where it lives.
pub struct Project {
    /// Absolute path to `project.yaml`.
    pub manifest_path: Utf8PathBuf,
    /// The directory the manifest lives in — the root for relative paths.
    pub root: Utf8PathBuf,
    /// The parsed manifest.
    pub manifest: ProjectManifest,
}

impl Project {
    /// Load the project containing `dir`, searching upward for a manifest.
    pub fn discover(dir: &Utf8Path) -> miette::Result<Self> {
        let manifest_path = superquery_manifest::reader::discover(dir)?;
        Self::load(&manifest_path)
    }

    /// Load an exact manifest path.
    pub fn load(manifest_path: &Utf8Path) -> miette::Result<Self> {
        let manifest = superquery_manifest::from_path(manifest_path)?;
        let root = manifest_path
            .parent()
            .unwrap_or(Utf8Path::new("."))
            .to_owned();
        Ok(Self {
            manifest_path: manifest_path.to_owned(),
            root,
            manifest,
        })
    }

    /// Resolve a manifest-relative path against the project root.
    pub fn resolve(&self, relative: &Utf8Path) -> Utf8PathBuf {
        self.root.join(relative)
    }

    /// Where `superquery codegen` writes.
    pub fn generated_dir(&self) -> Utf8PathBuf {
        self.root.join("src").join("generated")
    }

    /// Where `superquery build` writes.
    pub fn dist_dir(&self) -> Utf8PathBuf {
        self.root.join("dist")
    }
}

/// The chain integrations linked into this build.
pub fn registry() -> Registry {
    let mut registry = Registry::new();
    registry.register(Box::new(EvmIntegration));
    registry
}

/// The current working directory, as UTF-8.
pub fn cwd() -> miette::Result<Utf8PathBuf> {
    let dir = std::env::current_dir()
        .map_err(|e| miette::miette!("could not read the current directory: {e}"))?;
    Utf8PathBuf::from_path_buf(dir)
        .map_err(|p| miette::miette!("working directory is not valid UTF-8: {}", p.display()))
}
