//! Milestone 8's acceptance, run against the real binary:
//!
//! ```text
//! superquery init demo --chain evm
//! cd demo
//! superquery validate
//! superquery codegen
//! cargo check
//! ```

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const SUPERQUERY: &str = env!("CARGO_BIN_EXE_superquery");

fn sdk_crate() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../sdk")
        .canonicalize()
        .unwrap()
}

fn run(cmd: &mut Command) -> Output {
    let output = cmd.output().expect("command runs");
    assert!(
        output.status.success(),
        "{cmd:?} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    output
}

fn superquery(dir: &Path, args: &[&str]) -> String {
    let output = run(Command::new(SUPERQUERY).args(args).current_dir(dir));
    String::from_utf8(output.stdout).unwrap()
}

/// Scaffold `demo` against this checkout's SDK and generate its code.
fn generated_project(root: &Path) -> PathBuf {
    let sdk = sdk_crate();
    superquery(
        root,
        &[
            "init",
            "demo",
            "--chain",
            "evm",
            "--sdk-path",
            sdk.to_str().unwrap(),
        ],
    );
    let project = root.join("demo");

    assert!(
        superquery(&project, &["validate"])
            .trim_end()
            .ends_with("ok")
    );
    let codegen = superquery(&project, &["codegen"]);
    assert!(codegen.contains("entities.rs"), "{codegen}");
    project
}

#[test]
fn init_validate_codegen_works_from_inside_the_new_project() {
    let tmp = tempfile::tempdir().unwrap();
    let project = generated_project(tmp.path());

    for file in [
        "mod.rs",
        "entities.rs",
        "schema_metadata.rs",
        "contracts/mod.rs",
        "contracts/erc20.rs",
    ] {
        assert!(
            project.join("src/generated").join(file).is_file(),
            "missing {file}"
        );
    }
}

#[test]
fn a_second_codegen_run_rewrites_nothing() {
    let tmp = tempfile::tempdir().unwrap();
    let project = generated_project(tmp.path());
    let before = std::fs::read_to_string(project.join("src/generated/entities.rs")).unwrap();

    assert_eq!(superquery(&project, &["codegen"]).trim(), "up to date");
    let after = std::fs::read_to_string(project.join("src/generated/entities.rs")).unwrap();
    assert_eq!(before, after);
}

/// Compiles alloy and the SDK into a fresh target dir, so it is too slow for
/// every `cargo test`; CI runs it with `--ignored`.
#[test]
#[ignore = "slow: compiles the generated project; run with --ignored"]
fn the_generated_project_passes_cargo_check() {
    let tmp = tempfile::tempdir().unwrap();
    let project = generated_project(tmp.path());

    // Share the workspace target dir so dependencies are compiled once.
    let target = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/lifecycle");
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    run(Command::new(cargo)
        .args(["check", "--quiet"])
        .env("CARGO_TARGET_DIR", target)
        .current_dir(&project));
}
