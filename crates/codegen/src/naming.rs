//! Turning schema names into Rust names.
//!
//! Kept in one place because the same mapping has to be applied identically by
//! entity codegen, the store metadata and the macros. A name that round-trips
//! differently in two of those produces code that compiles and then fails to
//! find its own table.

use heck::{ToSnakeCase, ToUpperCamelCase};

/// Rust identifiers that cannot be used as-is for a field.
const RESERVED: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "union",
    "unsafe", "use", "where", "while", "yield",
];

/// Keywords that cannot be raw identifiers either; these get a trailing `_`.
const NOT_RAW: &[&str] = &["crate", "self", "Self", "super"];

/// The Rust struct name for an entity, e.g. `Transfer` -> `Transfer`.
pub fn entity_struct(entity: &str) -> String {
    entity.to_upper_camel_case()
}

/// The Rust field name for a schema field, e.g. `blockNumber` -> `block_number`.
///
/// Reserved words are escaped with a raw identifier rather than mangled, so
/// the generated field still reads like the schema field.
pub fn field_ident(field: &str) -> String {
    let snake = field.to_snake_case();
    if NOT_RAW.contains(&snake.as_str()) {
        format!("{snake}_")
    } else if RESERVED.contains(&snake.as_str()) {
        format!("r#{snake}")
    } else {
        snake
    }
}

/// The Rust variant name for a schema enum value, e.g. `BUY` -> `Buy`.
pub fn enum_variant(value: &str) -> String {
    // Only `Self` can collide: every other keyword is lowercase.
    let camel = value.to_lowercase().to_upper_camel_case();
    if NOT_RAW.contains(&camel.as_str()) {
        format!("{camel}_")
    } else {
        camel
    }
}

/// The schema name a generated field would map back to without help.
///
/// When this differs from the schema's own spelling, codegen must emit
/// `#[superquery(rename = "...")]` so the derive writes the right key.
pub fn implied_schema_name(rust_ident: &str) -> &str {
    rust_ident.strip_prefix("r#").unwrap_or(rust_ident)
}

/// The module name for an entity's generated code.
pub fn module_name(entity: &str) -> String {
    entity.to_snake_case()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_names_become_camel_case_structs() {
        assert_eq!(entity_struct("Transfer"), "Transfer");
        assert_eq!(entity_struct("token_holder"), "TokenHolder");
    }

    #[test]
    fn field_names_become_snake_case() {
        assert_eq!(field_ident("blockNumber"), "block_number");
        assert_eq!(field_ident("id"), "id");
    }

    #[test]
    fn enum_values_become_camel_case_variants() {
        assert_eq!(enum_variant("BUY"), "Buy");
        assert_eq!(enum_variant("LIMIT_ORDER"), "LimitOrder");
        assert_eq!(enum_variant("self"), "Self_");
    }

    #[test]
    fn reserved_words_become_raw_identifiers() {
        assert_eq!(field_ident("type"), "r#type");
        assert_eq!(field_ident("match"), "r#match");
    }

    #[test]
    fn keywords_that_cannot_be_raw_get_a_trailing_underscore() {
        assert_eq!(field_ident("self"), "self_");
        assert_eq!(field_ident("super"), "super_");
        assert_eq!(field_ident("crate"), "crate_");
    }
}
