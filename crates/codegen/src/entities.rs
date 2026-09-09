//! Entity struct generation.
//!
//! Produces one `src/generated/entities.rs` containing a struct per entity,
//! plus the `Entity` trait impl that lets `.save()` work.

use superquery_schema::{EntityDefinition, FieldDefinition, FieldType, SchemaIr};
use superquery_types::ScalarKind;

use crate::error::{CodegenError, CodegenResult};
use crate::naming::{entity_struct, field_ident};
use crate::{GENERATED_HEADER, writer::GeneratedFile};

/// Generate `entities.rs` for a whole schema.
pub fn generate(ir: &SchemaIr) -> CodegenResult<GeneratedFile> {
    let mut out = String::from(GENERATED_HEADER);
    out.push_str("//! Entity types generated from `schema.graphql`.\n\n");
    out.push_str("use superquery_sdk::prelude::*;\n\n");

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
    out.push_str(&format!("    pub {}: {ty},\n", field_ident(&field.name)));
    Ok(out)
}

/// Map an IR type to the Rust type a mapping author writes against.
fn rust_type(ty: &FieldType, location: &str) -> CodegenResult<String> {
    Ok(match ty {
        FieldType::Scalar { scalar } => scalar_type(*scalar, location)?.to_owned(),
        // A relation is stored as the target's id, not as a nested struct:
        // the store is flat, and loading the target is an explicit call.
        FieldType::Entity { .. } => "String".to_owned(),
        FieldType::Enum { name } => name.clone(),
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
