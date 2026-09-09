//! Schema metadata generation.
//!
//! The generated mapping needs to describe its own entities at run time — the
//! host has to know which fields exist and what they are before it can accept
//! a `sq_store_set`. This emits that table as constants so the mapping carries
//! its schema with it and the node never has to re-read `schema.graphql`.

use superquery_schema::SchemaIr;

use crate::GENERATED_HEADER;
use crate::writer::GeneratedFile;

/// Generate `schema_metadata.rs`, embedding the canonical IR.
pub fn generate(ir: &SchemaIr) -> GeneratedFile {
    let mut out = String::from(GENERATED_HEADER);
    out.push_str("//! Schema metadata generated from `schema.graphql`.\n\n");
    out.push_str(
        "/// The canonical schema IR this mapping was built against.\n\
         ///\n\
         /// The node compares this against the schema in the bundle and refuses\n\
         /// a mapping whose view of the schema has drifted.\n",
    );
    out.push_str(&format!(
        "pub const SCHEMA_IR_JSON: &str = r##\"{}\"##;\n\n",
        superquery_schema::canonical_json(ir)
    ));
    out.push_str("/// SHA-256 of [`SCHEMA_IR_JSON`], as written to `build.json`.\n");
    out.push_str(&format!(
        "pub const SCHEMA_HASH: &str = \"{}\";\n\n",
        superquery_schema::schema_hash(ir)
    ));
    out.push_str("/// Every entity type name in this schema, sorted.\n");
    out.push_str("pub const ENTITY_NAMES: &[&str] = &[\n");
    for entity in &ir.entities {
        out.push_str(&format!("    \"{}\",\n", entity.name));
    }
    out.push_str("];\n");

    GeneratedFile {
        path: "schema_metadata.rs".into(),
        contents: out,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embeds_the_hash_and_entity_names() {
        let ir =
            superquery_schema::parse("type Transfer @entity { id: ID! }", "test.graphql").unwrap();
        let file = generate(&ir);
        assert!(file.contents.contains("\"Transfer\","), "{}", file.contents);
        assert!(
            file.contents.contains(&superquery_schema::schema_hash(&ir)),
            "{}",
            file.contents
        );
    }
}
