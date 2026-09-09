//! The command surface.
//!
//! v0.1 is deliberately six commands. Publishing and multi-chain support come
//! after the EVM lifecycle is stable — see §3 of the implementation guide.

mod build;
mod codegen;
mod doctor;
mod init;
mod test;
mod validate;

use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};

/// Define, generate, build and validate SuperQuery projects.
#[derive(Debug, Parser)]
#[command(name = "superquery", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Path to `project.yaml`. Defaults to searching upward from the cwd.
    #[arg(long, short = 'm', global = true, value_name = "PATH")]
    manifest: Option<Utf8PathBuf>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Create a new indexer project from a template.
    Init(init::Args),
    /// Check the manifest, schema and assets without touching the network.
    Validate(validate::Args),
    /// Generate Rust entities and contract bindings.
    Codegen(codegen::Args),
    /// Compile the mapping and produce a `dist/` bundle.
    Build(build::Args),
    /// Run the project's mapping tests.
    Test(test::Args),
    /// Diagnose the local toolchain and project setup.
    Doctor(doctor::Args),
}

impl Cli {
    /// Dispatch to the selected command.
    pub fn run(self) -> miette::Result<()> {
        let manifest = self.manifest;
        match self.command {
            Command::Init(args) => init::run(args),
            Command::Validate(args) => validate::run(args, manifest),
            Command::Codegen(args) => codegen::run(args, manifest),
            Command::Build(args) => build::run(args, manifest),
            Command::Test(args) => test::run(args, manifest),
            Command::Doctor(args) => doctor::run(args, manifest),
        }
    }
}

/// Load the project a command should act on.
fn project(manifest: Option<Utf8PathBuf>) -> miette::Result<crate::context::Project> {
    match manifest {
        Some(path) => crate::context::Project::load(&path),
        None => crate::context::Project::discover(&crate::context::cwd()?),
    }
}
