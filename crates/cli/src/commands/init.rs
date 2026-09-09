//! `superquery init` — scaffold a new project.
//!
//! Scaffold for Milestone 8. The template layout is settled (`templates/evm`);
//! the copy-and-substitute step is not written yet.

use clap::Args as ClapArgs;
use superquery_types::ChainFamily;

/// Arguments for `superquery init`.
#[derive(Debug, ClapArgs)]
pub struct Args {
    /// Directory to create, and the project's name.
    pub name: String,

    /// Which chain family to scaffold for.
    #[arg(long, default_value = "evm")]
    pub chain: String,
}

/// Run the command.
pub fn run(args: Args) -> miette::Result<()> {
    let family = ChainFamily::parse(&args.chain).map_err(|e| miette::miette!("{e}"))?;

    if !family.is_implemented() {
        return Err(miette::miette!(
            "`{family}` templates are not available yet — only `evm` is implemented"
        ));
    }

    // TODO(milestone-8): copy `templates/evm`, substituting the project name
    // into Cargo.toml and project.yaml, then print the next steps.
    Err(miette::miette!(
        "`superquery init` is not implemented yet (Milestone 8); \
         copy `templates/{family}` manually for now"
    ))
}
