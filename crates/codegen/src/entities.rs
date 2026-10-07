//! Entity struct generation.
//!
//! Produces one `src/generated/entities.rs` containing a Rust enum per schema
//! enum and a struct per entity. The structs derive `SuperQueryEntity`, which
//! supplies the `Entity` impl that lets `.save()` work; this module's job is
//! to hand the derive the schema's own names.

use std::collections::BTreeSet;

use superquery_schema::{EntityDefinition, EnumDefinition, FieldDefinition, FieldType, SchemaIr};
use superquery_types::ScalarKind;

use crate::error::{CodegenError, CodegenResult};
use crate::naming::{entity_struct, enum_variant, field_ident, implied_schema_name};
use crate::{GENERATED_HEADER, writer::GeneratedFile};

/// Generate `entities.rs` for a whole schema.
pub fn generate(ir: &SchemaIr) -> CodegenResult<GeneratedFile> {
    let mut out = String::from(GENERATED_HEADER);
    out.push_str("//! Entity types generated from `schema.graphql`.\n\n");
    out.push_str("#![allow(dead_code)]\n\n");
    out.push_str("use superquery_sdk::prelude::*;\n");
    if !ir.enums.is_empty() {
        out.push_str("use superquery_sdk::types::{FromValue, FromValueError, ToValue, Value};\n");
    }
    out.push('\n');

    for definition in &ir.enums {
        out.push_str(&render_enum(definition)?);
        out.push('\n');
    }

    for entity in &ir.entities {
        out.push_str(&render_entity(entity)?);
        out.push('\n');
    }

    Ok(GeneratedFile {
        path: "entities.rs".into(),
        contents: out,
    })
}

fn render_entity(entity: &EntityDefinition) -> CodegenResult<String> {
    let struct_name = entity_struct(&entity.name);
    let mut out = String::new();

    if let Some(doc) = &entity.description {
        for line in doc.lines() {
            out.push_str(&format!("/// {line}\n"));
        }
    }

    out.push_str("#[derive(Clone, Debug, PartialEq, SuperQueryEntity)]\n");
    out.push_str(&format!("#[superquery(entity = \"{}\")]\n", entity.name));
    out.push_str(&format!("pub struct {struct_name} {{\n"));

    for field in &entity.fields {
        out.push_str(&render_field(entity, field)?);
    }

    out.push_str("}\n");
    Ok(out)
}

fn render_field(entity: &EntityDefinition, field: &FieldDefinition) -> CodegenResult<String> {
    let location = format!("{}.{}", entity.name, field.name);
    let ty = rust_type(&field.field_type, &location)?;
    let ty = if field.nullable {
        format!("Option<{ty}>")
    } else {
        ty
    };

    let mut out = String::new();
    if let Some(doc) = &field.description {
        for line in doc.lines() {
            out.push_str(&format!("    /// {line}\n"));
        }
    }
    let ident = field_ident(&field.name);
    if implied_schema_name(&ident) != field.name {
        out.push_str(&format!("    #[superquery(rename = \"{}\")]\n", field.name));
    }
    out.push_str(&format!("    pub {ident}: {ty},\n"));
    Ok(out)
}

/// A schema enum becomes a Rust enum stored as its schema spelling, so the
/// node and query service see `"BUY"`, never `"Buy"`.
fn render_enum(definition: &EnumDefinition) -> CodegenResult<String> {
    let name = entity_struct(&definition.name);
    let variants: Vec<(String, &str)> = definition
        .values
        .iter()
        .map(|value| (enum_variant(value), value.as_str()))
        .collect();

    let mut seen = BTreeSet::new();
    for (variant, value) in &variants {
        if !seen.insert(variant) {
            return Err(CodegenError::Unsupported {
                location: format!("{}.{value}", definition.name),
                message: format!("two values both become the Rust variant `{variant}`"),
            });
        }
    }

    let mut out =
        format!("#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\npub enum {name} {{\n");
    for (variant, value) in &variants {
        out.push_str(&format!("    /// `{value}`\n    {variant},\n"));
    }
    out.push_str("}\n\n");

    out.push_str(&format!("impl {name} {{\n"));
    out.push_str("    /// The value's spelling in `schema.graphql`.\n");
    out.push_str("    pub const fn as_str(self) -> &'static str {\n        match self {\n");
    for (variant, value) in &variants {
        out.push_str(&format!("            Self::{variant} => \"{value}\",\n"));
    }
    out.push_str("        }\n    }\n}\n\n");

    out.push_str(&format!(
        "impl ToValue for {name} {{\n    fn to_value(&self) -> Value {{\n        \
         Value::String(self.as_str().to_owned())\n    }}\n}}\n\n"
    ));

    out.push_str(&format!(
        "impl FromValue for {name} {{\n    fn from_value(value: &Value) -> Result<Self, FromValueError> {{\n        \
         match value.as_str() {{\n"
    ));
    for (variant, value) in &variants {
        out.push_str(&format!(
            "            Some(\"{value}\") => Ok(Self::{variant}),\n"
        ));
    }
    out.push_str(&format!(
        "            _ => Err(FromValueError::Invalid(format!(\n                \
         \"{{value:?}} is not a {} value\"\n            ))),\n        }}\n    }}\n}}\n",
        definition.name
    ));
    Ok(out)
}

/// Map an IR type to the Rust type a mapping author writes against.
fn rust_type(ty: &FieldType, location: &str) -> CodegenResult<String> {
    Ok(match ty {
        FieldType::Scalar { scalar } => scalar_type(*scalar, location)?.to_owned(),
        // A relation is stored as the target's id, not as a nested struct:
        // the store is flat, and loading the target is an explicit call.
        FieldType::Entity { .. } => "String".to_owned(),
        FieldType::Enum { name } => entity_struct(name),
        FieldType::List {
            inner,
            nullable_elements,
        } => {
            let element = rust_type(inner, location)?;
            let element = if *nullable_elements {
                format!("Option<{element}>")
            } else {
                element
            };
            format!("Vec<{element}>")
        }
    })
}

/// The Rust type for each schema scalar.
///
/// `BigInt`/`BigDecimal` are SDK newtypes rather than raw strings so mapping
/// code gets arithmetic and the store gets an unambiguous encoding.
///
/// `ScalarKind` is `#[non_exhaustive]`, so a scalar added upstream without a
/// mapping here is a clear error rather than a silently wrong type.
fn scalar_type(scalar: ScalarKind, location: &str) -> CodegenResult<&'static str> {
    Ok(match scalar {
        ScalarKind::Id | ScalarKind::String => "String",
        ScalarKind::Boolean => "bool",
        ScalarKind::Int => "i32",
        ScalarKind::BigInt => "BigInt",
        ScalarKind::Float => "f64",
        ScalarKind::BigDecimal => "BigDecimal",
        ScalarKind::Bytes => "Bytes",
        ScalarKind::Date => "Timestamp",
        ScalarKind::Json => "Json",
        other => {
            return Err(CodegenError::Unsupported {
                location: location.to_owned(),
                message: format!("scalar `{other}` has no Rust representation in this SDK build"),
            });
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use superquery_schema::parse;

    fn ir(src: &str) -> SchemaIr {
        parse(src, "test.graphql").expect("fixture parses")
    }

    #[test]
    fn generates_a_struct_per_entity() {
        let ir = ir(r#"
            type Transfer @entity {
              id: ID!
              from: String!
              value: BigInt!
            }
        "#);
        let file = generate(&ir).unwrap();
        assert!(
            file.contents.contains("pub struct Transfer {"),
            "{}",
            file.contents
        );
        assert!(
            file.contents.contains("pub value: BigInt,"),
            "{}",
            file.contents
        );
    }

    #[test]
    fn nullable_fields_become_option() {
        let ir = ir(r#"
            type Transfer @entity {
              id: ID!
              memo: String
            }
        "#);
        let file = generate(&ir).unwrap();
        assert!(
            file.contents.contains("pub memo: Option<String>,"),
            "{}",
            file.contents
        );
    }

    #[test]
    fn relations_are_generated_as_the_target_id() {
        let ir = ir(r#"
            type Account @entity { id: ID! }
            type Transfer @entity {
              id: ID!
              sender: Account!
            }
        "#);
        let file = generate(&ir).unwrap();
        assert!(
            file.contents.contains("pub sender: String,"),
            "{}",
            file.contents
        );
    }

    #[test]
    fn fields_whose_rust_name_differs_carry_the_schema_name() {
        let ir = ir(r#"
            type Transfer @entity {
              id: ID!
              blockNumber: BigInt!
              type: String!
              self: String
            }
        "#);
        let out = generate(&ir).unwrap().contents;
        assert!(
            out.contains(
                "    #[superquery(rename = \"blockNumber\")]\n    pub block_number: BigInt,"
            ),
            "{out}"
        );
        assert!(!out.contains("rename = \"id\""), "{out}");
        assert!(
            !out.contains("rename = \"type\""),
            "r#type maps back on its own: {out}"
        );
        assert!(out.contains("rename = \"self\")]\n    pub self_:"), "{out}");
    }

    #[test]
    fn enums_are_generated_and_stored_as_their_schema_spelling() {
        let ir = ir(r#"
            enum Side { BUY SELL_SHORT }
            type Order @entity { id: ID! side: Side! }
        "#);
        let out = generate(&ir).unwrap().contents;
        assert!(out.contains("pub enum Side {"), "{out}");
        assert!(out.contains("SellShort,"), "{out}");
        assert!(out.contains("Self::SellShort => \"SELL_SHORT\""), "{out}");
        assert!(out.contains("pub side: Side,"), "{out}");
    }

    #[test]
    fn enum_values_that_collide_in_rust_are_rejected() {
        let ir = ir("enum E { A_B a_b } type T @entity { id: ID! e: E }");
        assert!(matches!(
            generate(&ir),
            Err(CodegenError::Unsupported { .. })
        ));
    }

    #[test]
    fn the_template_schema_generates_pinned_output() {
        let ir = ir(include_str!("../../../templates/evm/schema.graphql"));
        insta::assert_snapshot!("template_entities", generate(&ir).unwrap().contents);
    }

    #[test]
    fn output_is_byte_for_byte_stable_across_runs() {
        let src = r#"
            type Transfer @entity {
              id: ID!
              blockNumber: BigInt!
            }
        "#;
        assert_eq!(generate(&ir(src)).unwrap(), generate(&ir(src)).unwrap());
    }
}
