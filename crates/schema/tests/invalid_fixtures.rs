//! Every schema rejection rule, proved by a fixture.
//!
//! Each `tests/fixtures/invalid/*.graphql` file opens with
//! `# expect: <location>` and `# message: <fragment>`. The schema must fail
//! parsing or whole-schema validation with an `Invalid` error at exactly that
//! location, whose message contains the fragment.

use camino::{Utf8Path, Utf8PathBuf};
use superquery_schema::{SchemaError, SchemaResult, parse_file, validate};

fn header<'a>(source: &'a str, key: &str) -> &'a str {
    source
        .lines()
        .take_while(|line| line.starts_with('#'))
        .find_map(|line| line.strip_prefix('#')?.trim().strip_prefix(key))
        .map(str::trim)
        .unwrap_or_else(|| panic!("fixture is missing `# {key}`"))
}

fn parse_and_validate(path: &Utf8Path) -> SchemaResult<()> {
    validate(&parse_file(path)?)
}

#[test]
fn each_invalid_schema_is_rejected_at_its_declared_location() {
    let root = Utf8Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/invalid");
    let mut paths: Vec<Utf8PathBuf> = std::fs::read_dir(&root)
        .unwrap()
        .map(|e| Utf8PathBuf::try_from(e.unwrap().path()).unwrap())
        .collect();
    paths.sort();
    assert!(paths.len() >= 15, "fixtures went missing from {root}");

    for path in paths {
        let source = std::fs::read_to_string(&path).unwrap();
        let (want_location, want_message) =
            (header(&source, "expect:"), header(&source, "message:"));

        match parse_and_validate(&path) {
            Err(SchemaError::Invalid {
                location, message, ..
            }) => {
                assert_eq!(location, want_location, "{path}: {message}");
                assert!(
                    message.contains(want_message),
                    "{path}: expected `{want_message}` in `{message}`"
                );
            }
            other => panic!("{path}: expected an Invalid error, got {other:?}"),
        }
    }
}
