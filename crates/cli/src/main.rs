//! The `superquery` binary.

mod commands;
mod context;

use clap::Parser;

use commands::Cli;

fn main() -> miette::Result<()> {
    miette::set_panic_hook();
    Cli::parse().run()
}
