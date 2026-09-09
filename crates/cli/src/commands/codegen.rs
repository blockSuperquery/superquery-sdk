//! `superquery codegen` — schema and ABIs to Rust.
//!
//! Partially wired: entity and metadata generation run; contract bindings are
//! generated but the `sol!` output is not yet compiled against a real ABI
//! fixture (Milestone 5).

use camino::Utf8PathBuf;
use clap::Args as ClapArgs;
use superquery_codegen::OutputSet;

/// Arguments for `superquery codegen`.
#[derive(Debug, ClapArgs)]
pub struct Args {
    /// Print what would be written without touching the filesystem.
    #[arg(long)]
    pub dry_run: bool,
}

/// Run the command.
pub fn run(args: Args, manifest: Option<Utf8PathBuf>) -> miette::Result<()> {
    let project = super::project(manifest)?;
    let schema_path = project.resolve(&project.manifest.schema.file);
    let ir = superquery_schema::parse_file(&schema_path)?;
    superquery_schema::validate(&ir)?;

    let mut output = OutputSet::new();
    output.push(superquery_codegen::entities::generate(&ir).map_err(|e| miette::miette!("{e}"))?);
    output.push(superquery_codegen::metadata::generate(&ir));

    let registry = crate::context::registry();
    let integration = registry
        .get(project.manifest.network.family)
        .map_err(|e| miette::miette!("{e}"))?;

    for data_source in &project.manifest.data_sources {
        let modules = integration
            .codegen(&project.manifest, data_source, &project.root)
            .map_err(|e| miette::miette!("{e}"))?;
        for module in modules {
            for file in module.files {
                output.push(superquery_codegen::writer::GeneratedFile {
                    path: file.path.into(),
                    contents: file.contents,
                });
            }
        }
    }

    let output = output.finish();
    let out_dir = project.generated_dir();

    if args.dry_run {
        for file in &output.files {
            println!("would write {}", out_dir.join(&file.path));
        }
        return Ok(());
    }

    let written = superquery_codegen::write_output_set(&out_dir, &output)
        .map_err(|e| miette::miette!("{e}"))?;

    for path in &written {
        println!("wrote {path}");
    }
    if written.is_empty() {
        println!("up to date");
    }
    Ok(())
}
