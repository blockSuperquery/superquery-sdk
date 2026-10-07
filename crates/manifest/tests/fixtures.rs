//! Fixture-driven validation tests.
//!
//! Each file under `tests/fixtures/` declares what it proves in a header
//! comment, so adding a validation rule means adding one YAML file:
//!
//! - `valid/*.yaml` must parse and validate with no findings at all.
//! - `invalid/*.yaml` carries `# expect: <error|warning> <field>` lines and
//!   must produce exactly those findings — no more, so each fixture pins one
//!   branch rather than passing because something else went wrong.
//! - `unparseable/*.yaml` carries `# expect-parse: <fragment>` and must fail
//!   to parse with a message containing it.

use camino::{Utf8Path, Utf8PathBuf};
use std::collections::BTreeSet;
use superquery_manifest::{Severity, from_path, validate};

fn fixtures(dir: &str) -> Vec<Utf8PathBuf> {
    let root = Utf8Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(dir);
    let mut files: Vec<_> = std::fs::read_dir(&root)
        .unwrap_or_else(|e| panic!("reading {root}: {e}"))
        .map(|entry| Utf8PathBuf::try_from(entry.unwrap().path()).unwrap())
        .filter(|path| path.extension() == Some("yaml"))
        .collect();
    files.sort();
    assert!(!files.is_empty(), "no fixtures in {root}");
    files
}

fn header_values<'a>(source: &'a str, key: &str) -> Vec<&'a str> {
    source
        .lines()
        .take_while(|line| line.starts_with('#'))
        .filter_map(|line| line.strip_prefix('#')?.trim().strip_prefix(key))
        .map(str::trim)
        .collect()
}

fn severity_name(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
    }
}

#[test]
fn valid_fixtures_produce_no_findings() {
    for path in fixtures("valid") {
        let manifest = from_path(&path).unwrap_or_else(|e| panic!("{path}: {e:?}"));
        let report = validate(&manifest, path.parent());
        assert!(
            report.diagnostics.is_empty(),
            "{path} should be clean, got {:#?}",
            report.diagnostics
        );
    }
}

#[test]
fn each_invalid_fixture_produces_exactly_its_declared_findings() {
    for path in fixtures("invalid") {
        let source = std::fs::read_to_string(&path).unwrap();
        let expected: BTreeSet<String> = header_values(&source, "expect:")
            .into_iter()
            .map(str::to_owned)
            .collect();
        assert!(!expected.is_empty(), "{path} declares no `# expect:` line");

        let manifest = from_path(&path).unwrap_or_else(|e| panic!("{path} must parse: {e:?}"));
        let report = validate(&manifest, path.parent());
        let actual: BTreeSet<String> = report
            .diagnostics
            .iter()
            .map(|d| format!("{} {}", severity_name(d.severity), d.field))
            .collect();

        assert_eq!(actual, expected, "{path}: {:#?}", report.diagnostics);
    }
}

#[test]
fn every_blocking_finding_explains_how_to_fix_it_or_why() {
    // A field path plus a message is the floor; this pins that no error-level
    // diagnostic ships with an empty message.
    for path in fixtures("invalid") {
        let manifest = from_path(&path).unwrap();
        for diagnostic in validate(&manifest, path.parent()).errors() {
            assert!(
                !diagnostic.message.trim().is_empty(),
                "{path}: {diagnostic:?}"
            );
        }
    }
}

#[test]
fn unparseable_fixtures_fail_with_a_message_naming_the_problem() {
    for path in fixtures("unparseable") {
        let source = std::fs::read_to_string(&path).unwrap();
        let [fragment] = header_values(&source, "expect-parse:")[..] else {
            panic!("{path} must declare exactly one `# expect-parse:` line");
        };

        let err = from_path(&path).expect_err(path.as_str());
        let message = err.to_string();
        assert!(
            message.contains(fragment),
            "{path}: expected `{fragment}` in `{message}`"
        );
    }
}

#[test]
fn skipping_filesystem_checks_ignores_missing_files() {
    let path = Utf8Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/invalid/missing-schema.yaml");
    let manifest = from_path(&path).unwrap();
    assert!(!validate(&manifest, None).has_errors());
}
