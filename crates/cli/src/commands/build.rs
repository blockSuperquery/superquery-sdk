//! `superquery build` — compile the mapping and bundle `dist/`.
//!
//! Scaffold for Milestone 9. The artifact contract is specified in
//! `docs/spec/build-artifact-v1.md`; the compile-and-hash pipeline is not
//! written yet.
//!
//! What it will do, in order: run codegen, `cargo build --release --target
//! wasm32-wasip1`, copy the manifest/schema/IR/assets into `dist/`, then write
//! `build.json` with the three hashes the node verifies.

use camino::Utf8PathBuf;
use clap::Args as ClapArgs;

/// Arguments for `superquery build`.
#[derive(Debug, ClapArgs)]
pub struct Args {
    /// Compile target for the mapping.
    #[arg(long, default_value = "wasm32-wasip1")]
    pub target: String,

    /// Skip the codegen step, assuming `src/generated` is current.
    #[arg(long)]
    pub skip_codegen: bool,
}

/// Run the command.
pub fn run(_args: Args, manifest: Option<Utf8PathBuf>) -> miette::Result<()> {
    let project = super::project(manifest)?;
    Err(miette::miette!(
        "`superquery build` is not implemented yet (Milestone 9); \
         would bundle {} into {}",
        project.manifest.name,
        project.dist_dir()
    ))
}
