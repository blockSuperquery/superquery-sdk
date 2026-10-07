//! `superquery init` — scaffold a new project.
//!
//! The template comes from the family's `ChainIntegration`, so this command
//! knows nothing about any chain: it validates the name, refuses to clobber
//! anything, writes the files and substitutes the project name.

use camino::{Utf8Path, Utf8PathBuf};
use clap::Args as ClapArgs;
use superquery_chain_api::ProjectTemplate;
use superquery_types::{ChainFamily, ProjectId};

/// Arguments for `superquery init`.
#[derive(Debug, ClapArgs)]
pub struct Args {
    /// Directory to create. Its last component becomes the project name.
    pub path: Utf8PathBuf,

    /// Which chain family to scaffold for.
    #[arg(long, default_value = "evm")]
    pub chain: String,

    /// Depend on a local SDK checkout instead of the git release, e.g. while
    /// developing the SDK itself.
    #[arg(long, value_name = "PATH")]
    pub sdk_path: Option<Utf8PathBuf>,
}

/// Run the command.
pub fn run(args: Args) -> miette::Result<()> {
    let family = ChainFamily::parse(&args.chain).map_err(|e| miette::miette!("{e}"))?;
    let registry = crate::context::registry();
    let template = registry
        .get(family)
        .ok()
        .and_then(|integration| integration.template())
        .ok_or_else(|| miette::miette!("no `{family}` project template is available yet"))?;

    let name = scaffold(template, &args.path, args.sdk_path.as_deref())?;

    println!("created `{name}` ({family}) in {}", args.path);
    println!();
    println!("next:");
    println!("  cd {}", args.path);
    println!("  superquery validate");
    println!("  superquery codegen");
    println!("  cargo check");
    Ok(())
}

/// Write `template` into `dir`, returning the project name used.
fn scaffold(
    template: &ProjectTemplate,
    dir: &Utf8Path,
    sdk_path: Option<&Utf8Path>,
) -> miette::Result<String> {
    let name = dir
        .file_name()
        .ok_or_else(|| miette::miette!("`{dir}` does not name a directory"))?;
    ProjectId::new(name).map_err(|e| {
        miette::miette!(
            help = "the name becomes a directory, a database schema and a URL segment, \
                    so it is restricted to lowercase letters, digits and `-`",
            "{e}"
        )
    })?;

    if dir.exists() {
        let mut entries = dir
            .read_dir_utf8()
            .map_err(|e| miette::miette!("could not read `{dir}`: {e}"))?;
        if entries.next().is_some() {
            return Err(miette::miette!(
                help = "pick a new directory name, or empty this one first",
                "`{dir}` already exists and is not empty"
            ));
        }
    }

    for file in template.files {
        let path = dir.join(file.path);
        let contents = personalise(
            file.path,
            file.contents,
            template.placeholder_name,
            name,
            sdk_path,
        );
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| miette::miette!("could not create `{parent}`: {e}"))?;
        }
        std::fs::write(&path, contents)
            .map_err(|e| miette::miette!("could not write `{path}`: {e}"))?;
    }
    Ok(name.to_owned())
}

/// Substitute the project name, and optionally the SDK dependency, into the
/// two files that declare them. Only whole `name` entries are touched, so the
/// placeholder appearing in prose is left alone.
fn personalise(
    path: &str,
    contents: &str,
    placeholder: &str,
    name: &str,
    sdk_path: Option<&Utf8Path>,
) -> String {
    let lines = contents.split_inclusive('\n').map(|line| {
        let body = line.trim_end_matches('\n');
        let newline = &line[body.len()..];
        let replaced = match path {
            "project.yaml" if body == format!("name: {placeholder}") => format!("name: {name}"),
            "Cargo.toml" if body == format!("name = \"{placeholder}\"") => {
                format!("name = \"{name}\"")
            }
            "Cargo.toml" if body.starts_with("superquery-sdk =") => match sdk_path {
                Some(sdk) => format!("superquery-sdk = {{ path = \"{sdk}\" }}"),
                None => body.to_owned(),
            },
            _ => body.to_owned(),
        };
        replaced + newline
    });
    lines.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn template() -> &'static ProjectTemplate {
        crate::context::registry()
            .get(ChainFamily::Evm)
            .unwrap()
            .template()
            .unwrap()
    }

    fn tempdir() -> (tempfile::TempDir, Utf8PathBuf) {
        let tmp = tempfile::tempdir().unwrap();
        let root = Utf8PathBuf::try_from(tmp.path().to_path_buf()).unwrap();
        (tmp, root)
    }

    #[test]
    fn writes_every_template_file_with_the_project_name_substituted() {
        let (_tmp, root) = tempdir();
        let dir = root.join("my-indexer");
        scaffold(template(), &dir, None).unwrap();

        for file in template().files {
            assert!(dir.join(file.path).is_file(), "missing {}", file.path);
        }
        let manifest = std::fs::read_to_string(dir.join("project.yaml")).unwrap();
        let cargo = std::fs::read_to_string(dir.join("Cargo.toml")).unwrap();
        assert!(manifest.contains("\nname: my-indexer\n"), "{manifest}");
        assert!(cargo.contains("\nname = \"my-indexer\"\n"), "{cargo}");
        assert!(!manifest.contains("name: erc20-transfers"));
    }

    #[test]
    fn the_scaffolded_manifest_validates_cleanly() {
        let (_tmp, root) = tempdir();
        let dir = root.join("fresh");
        scaffold(template(), &dir, None).unwrap();

        let manifest = superquery_manifest::from_path(&dir.join("project.yaml")).unwrap();
        let report = superquery_manifest::validate(&manifest, Some(&dir));
        assert!(!report.has_errors(), "{:#?}", report.diagnostics);
    }

    #[test]
    fn sdk_path_replaces_only_the_dependency_line() {
        let (_tmp, root) = tempdir();
        let dir = root.join("local");
        scaffold(template(), &dir, Some(Utf8Path::new("/src/sdk/crates/sdk"))).unwrap();
        let cargo = std::fs::read_to_string(dir.join("Cargo.toml")).unwrap();
        assert!(
            cargo.contains("superquery-sdk = { path = \"/src/sdk/crates/sdk\" }\n"),
            "{cargo}"
        );
        assert!(!cargo.contains("git ="), "{cargo}");
    }

    #[test]
    fn an_empty_existing_directory_is_fine() {
        let (_tmp, root) = tempdir();
        let dir = root.join("empty");
        std::fs::create_dir(&dir).unwrap();
        scaffold(template(), &dir, None).unwrap();
    }

    #[test]
    fn refuses_to_write_into_a_non_empty_directory() {
        let (_tmp, root) = tempdir();
        let dir = root.join("taken");
        std::fs::create_dir(&dir).unwrap();
        std::fs::write(dir.join("keep.txt"), "mine").unwrap();

        let err = scaffold(template(), &dir, None).unwrap_err();
        assert!(err.to_string().contains("not empty"), "{err}");
        assert_eq!(
            std::fs::read_to_string(dir.join("keep.txt")).unwrap(),
            "mine"
        );
        assert!(!dir.join("project.yaml").exists());
    }

    #[test]
    fn rejects_a_name_that_cannot_be_a_project_id() {
        let (_tmp, root) = tempdir();
        let err = scaffold(template(), &root.join("My_Indexer"), None).unwrap_err();
        assert!(err.to_string().contains("invalid project name"), "{err}");
        assert!(!root.join("My_Indexer").exists());
    }
}
