//! `superquery validate` — the fast, offline correctness check.
//!
//! Fast and offline is the point: this runs on every save in an editor loop,
//! so it must never need Postgres, RPC or a compiler.

use camino::Utf8PathBuf;
use clap::Args as ClapArgs;
use superquery_manifest::Severity;

/// Arguments for `superquery validate`.
#[derive(Debug, ClapArgs)]
pub struct Args {
    /// Also check that declared RPC endpoints are reachable.
    #[arg(long)]
    pub network: bool,
}

/// Run the command.
pub fn run(args: Args, manifest: Option<Utf8PathBuf>) -> miette::Result<()> {
    let project = super::project(manifest)?;
    println!("checking {}", project.manifest_path);

    let mut report = superquery_manifest::validate(&project.manifest, Some(&project.root));

    // Family-specific checks run on top of the generic ones.
    let registry = crate::context::registry();
    if let Ok(integration) = registry.get(project.manifest.network.family) {
        let chain = integration.validate(&project.manifest);
        for (field, message) in chain.errors {
            report.diagnostics.push(superquery_manifest::Diagnostic {
                field,
                message,
                help: None,
                severity: Severity::Error,
            });
        }
        for (field, message) in chain.warnings {
            report.diagnostics.push(superquery_manifest::Diagnostic {
                field,
                message,
                help: None,
                severity: Severity::Warning,
            });
        }
    }

    // The schema is parsed here rather than in the manifest crate: a manifest
    // can be valid while the schema it points at is not.
    let schema_path = project.resolve(&project.manifest.schema.file);
    if schema_path.is_file() {
        let ir = superquery_schema::parse_file(&schema_path)?;
        superquery_schema::validate(&ir)?;
        println!(
            "  schema  {} entit{}",
            ir.entities.len(),
            if ir.entities.len() == 1 { "y" } else { "ies" }
        );
    }

    for diagnostic in &report.diagnostics {
        let marker = match diagnostic.severity {
            Severity::Error => "error",
            Severity::Warning => "warn ",
        };
        println!("  {marker}  {}: {}", diagnostic.field, diagnostic.message);
        if let Some(help) = &diagnostic.help {
            println!("         help: {help}");
        }
    }

    if args.network {
        // TODO(milestone-8): probe `network.endpoint` with an eth_chainId call
        // and compare the answer against `network.chainId`.
        println!("  note   --network probing is not implemented yet");
    }

    // The findings are already printed above in a readable form, so fail with
    // a summary rather than re-rendering every diagnostic.
    let errors = report.errors().count();
    if errors > 0 {
        return Err(miette::miette!(
            "{errors} error(s) — see above; run `superquery doctor` if the setup itself looks wrong"
        ));
    }

    println!("ok");
    Ok(())
}
