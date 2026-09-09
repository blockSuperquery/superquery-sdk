//! `superquery test` — run mapping tests.
//!
//! Scaffold for Milestone 10. This will shell out to `cargo test` with the
//! testing feature enabled, so mappings are tested as ordinary Rust against
//! `superquery_sdk::testing::TestStore` — no RPC, no Postgres.

use camino::Utf8PathBuf;
use clap::Args as ClapArgs;

/// Arguments for `superquery test`.
#[derive(Debug, ClapArgs)]
pub struct Args {
    /// Only run tests whose name contains this string.
    #[arg(value_name = "FILTER")]
    pub filter: Option<String>,
}

/// Run the command.
pub fn run(_args: Args, manifest: Option<Utf8PathBuf>) -> miette::Result<()> {
    let _project = super::project(manifest)?;
    Err(miette::miette!(
        "`superquery test` is not implemented yet (Milestone 10); run `cargo test` for now"
    ))
}
