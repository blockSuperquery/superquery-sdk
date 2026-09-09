//! `superquery doctor` — diagnose the local setup.
//!
//! Partially wired: toolchain checks work, project checks are stubs. The point
//! of this command is that setup problems produce a checklist, not a
//! confusing failure three commands later.

use camino::Utf8PathBuf;
use clap::Args as ClapArgs;

/// The wasm target a mapping compiles to.
const WASM_TARGET: &str = "wasm32-wasip1";

/// Arguments for `superquery doctor`.
#[derive(Debug, ClapArgs)]
pub struct Args {
    /// Also check RPC and database connectivity.
    #[arg(long)]
    pub network: bool,
}

/// Run the command.
pub fn run(args: Args, manifest: Option<Utf8PathBuf>) -> miette::Result<()> {
    let mut failures = 0;

    check("rust toolchain", rust_version(), &mut failures);
    check("wasm target", wasm_target_installed(), &mut failures);

    match super::project(manifest) {
        Ok(project) => {
            check(
                "manifest",
                Ok(format!(
                    "{} ({})",
                    project.manifest.name, project.manifest_path
                )),
                &mut failures,
            );
            let schema = project.resolve(&project.manifest.schema.file);
            let schema_result = if schema.is_file() {
                Ok(schema.to_string())
            } else {
                Err(format!("`{schema}` does not exist"))
            };
            check("schema", schema_result, &mut failures);
        }
        Err(_) => println!("  skip   project — not inside a SuperQuery project"),
    }

    if args.network {
        // TODO(milestone-8): probe RPC endpoints and the node/query services.
        println!("  skip   network probing is not implemented yet");
    }

    if failures > 0 {
        return Err(miette::miette!("{failures} check(s) failed"));
    }
    println!("all checks passed");
    Ok(())
}

fn check(name: &str, result: Result<String, String>, failures: &mut usize) {
    match result {
        Ok(detail) => println!("  ok     {name}: {detail}"),
        Err(reason) => {
            *failures += 1;
            println!("  FAIL   {name}: {reason}");
        }
    }
}

fn rust_version() -> Result<String, String> {
    run_tool("rustc", &["--version"]).map(|s| s.trim().to_owned())
}

fn wasm_target_installed() -> Result<String, String> {
    let installed = run_tool("rustup", &["target", "list", "--installed"])?;
    if installed.lines().any(|line| line.trim() == WASM_TARGET) {
        Ok(WASM_TARGET.to_owned())
    } else {
        Err(format!(
            "`{WASM_TARGET}` not installed — run `rustup target add {WASM_TARGET}`"
        ))
    }
}

fn run_tool(program: &str, args: &[&str]) -> Result<String, String> {
    let output = std::process::Command::new(program)
        .args(args)
        .output()
        .map_err(|e| format!("could not run `{program}`: {e}"))?;

    if !output.status.success() {
        return Err(format!("`{program}` exited with {}", output.status));
    }
    String::from_utf8(output.stdout).map_err(|e| format!("`{program}` output was not UTF-8: {e}"))
}
