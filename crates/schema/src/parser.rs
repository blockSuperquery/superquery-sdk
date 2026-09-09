//! `schema.graphql` -> [`SchemaIr`].
//!
//! Two passes. The first collects the names every type declaration
//! introduces, because a field can reference an entity declared further down
//! the file. The second resolves each field against those names.

use camino::Utf8Path;
use graphql_parser::schema::{
    Definition, Directive, Document, Field as GqlField, ObjectType, Type as GqlType,
    TypeDefinition, Value as GqlValue,
};
use miette::NamedSource;
use std::collections::BTreeSet;
use superquery_types::ScalarKind;

use crate::error::{SchemaError, SchemaResult};
use crate::index::IndexDefinition;
use crate::ir::{EntityDefinition, EnumDefinition, FieldDefinition, FieldType, SchemaIr};
use crate::relation::{Relation, RelationKind};

/// The directive that marks a type as a stored entity.
const ENTITY_DIRECTIVE: &str = "entity";
/// Marks a field as computed from the other side of a relation.
const DERIVED_FROM_DIRECTIVE: &str = "derivedFrom";
/// Requests an index on a field.
const INDEX_DIRECTIVE: &str = "index";
/// Requests a unique index on a field.
const UNIQUE_DIRECTIVE: &str = "unique";

/// Read and parse a schema file.
pub fn parse_file(path: &Utf8Path) -> SchemaResult<SchemaIr> {
    let source = std::fs::read_to_string(path).map_err(|source| SchemaError::Unreadable {
        path: path.to_owned(),
        source,
    })?;
    parse(&source, path.as_str())
}

/// Parse a schema from source. `name` labels the source in diagnostics.
pub fn parse(source: &str, name: &str) -> SchemaResult<SchemaIr> {
    let doc: Document<'_, String> =
        graphql_parser::parse_schema(source).map_err(|err| SchemaError::Syntax {
            message: err.to_string(),
            src: NamedSource::new(name, source.to_owned()).with_language("graphql"),
            span: None,
        })?;

    // Pass 1: what names does this document introduce?
    let mut entity_names = BTreeSet::new();
    let mut enums = Vec::new();
    for def in &doc.definitions {
        match def {
            Definition::TypeDefinition(TypeDefinition::Object(obj)) if is_entity(obj) => {
                entity_names.insert(obj.name.clone());
            }
            Definition::TypeDefinition(TypeDefinition::Enum(e)) => {
                enums.push(EnumDefinition {
                    name: e.name.clone(),
                    values: e.values.iter().map(|v| v.name.clone()).collect(),
                });
            }
            _ => {}
        }
    }
    let enum_names: BTreeSet<_> = enums.iter().map(|e| e.name.clone()).collect();

    // Pass 2: resolve each entity's fields.
    let mut entities = Vec::new();
    for def in &doc.definitions {
        let Definition::TypeDefinition(TypeDefinition::Object(obj)) = def else {
            continue;
        };
        if !is_entity(obj) {
            // A plain object type is a modelling mistake worth naming rather
            // than silently dropping.
            return Err(SchemaError::invalid(
                obj.name.clone(),
                format!("type `{}` is missing the `@entity` directive", obj.name),
            )
            .with_help("add `@entity` to store this type, or remove it from the schema"));
        }
        entities.push(resolve_entity(obj, &entity_names, &enum_names)?);
    }

    entities.sort_by(|a, b| a.name.cmp(&b.name));
    enums.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(SchemaIr { entities, enums })
}

fn is_entity(obj: &ObjectType<'_, String>) -> bool {
    obj.directives.iter().any(|d| d.name == ENTITY_DIRECTIVE)
}

fn resolve_entity(
    obj: &ObjectType<'_, String>,
    entity_names: &BTreeSet<String>,
    enum_names: &BTreeSet<String>,
) -> SchemaResult<EntityDefinition> {
    let immutable = obj
        .directives
        .iter()
        .find(|d| d.name == ENTITY_DIRECTIVE)
        .and_then(|d| directive_bool(d, "immutable"))
        .unwrap_or(false);

    let mut fields = Vec::with_capacity(obj.fields.len());
    let mut indexes = vec![IndexDefinition::primary_key()];

    for field in &obj.fields {
        let location = format!("{}.{}", obj.name, field.name);
        let resolved = resolve_field(field, &location, entity_names, enum_names)?;

        // Owned relations get an index because the node will look them up by
        // foreign key on every derived-field resolution.
        if matches!(
            resolved.relation,
            Some(Relation {
                kind: RelationKind::Owned,
                ..
            })
        ) {
            indexes.push(IndexDefinition::foreign_key(&resolved.name));
        }

        if has_directive(&field.directives, UNIQUE_DIRECTIVE) {
            indexes.push(IndexDefinition::explicit(vec![resolved.name.clone()], true));
        } else if has_directive(&field.directives, INDEX_DIRECTIVE) {
            indexes.push(IndexDefinition::explicit(
                vec![resolved.name.clone()],
                false,
            ));
        }

        fields.push(resolved);
    }

    let entity = EntityDefinition {
        name: obj.name.clone(),
        description: obj.description.clone(),
        fields,
        indexes,
        immutable,
    };

    check_id_field(&entity)?;
    Ok(entity)
}

/// Every entity needs `id: ID!`: it is the store's primary key, and the node
/// has nowhere to put an entity without one.
fn check_id_field(entity: &EntityDefinition) -> SchemaResult<()> {
    let Some(id) = entity.id_field() else {
        return Err(SchemaError::invalid(
            entity.name.clone(),
            format!("entity `{}` has no `id` field", entity.name),
        )
        .with_help("every entity needs `id: ID!` — it is the primary key"));
    };

    let is_required_id = !id.nullable
        && matches!(
            id.field_type,
            FieldType::Scalar {
                scalar: ScalarKind::Id
            }
        );

    if !is_required_id {
        return Err(SchemaError::invalid(
            format!("{}.id", entity.name),
            "the `id` field must be declared `ID!`",
        )
        .with_help("`id` is the primary key, so it can be neither nullable nor another type"));
    }
    Ok(())
}

fn resolve_field(
    field: &GqlField<'_, String>,
    location: &str,
    entity_names: &BTreeSet<String>,
    enum_names: &BTreeSet<String>,
) -> SchemaResult<FieldDefinition> {
    let (field_type, nullable) =
        resolve_type(&field.field_type, location, entity_names, enum_names)?;

    let relation = match field_type.referenced_entity() {
        Some(target) => {
            let derived_from = field
                .directives
                .iter()
                .find(|d| d.name == DERIVED_FROM_DIRECTIVE)
                .map(|d| {
                    directive_str(d, "field").ok_or_else(|| {
                        SchemaError::invalid(
                            location.to_owned(),
                            "`@derivedFrom` requires a `field:` argument",
                        )
                        .with_help("write `@derivedFrom(field: \"owner\")`")
                    })
                })
                .transpose()?;

            Some(Relation {
                target: target.to_owned(),
                kind: if derived_from.is_some() {
                    RelationKind::Derived
                } else {
                    RelationKind::Owned
                },
                derived_from,
            })
        }
        None => {
            if has_directive(&field.directives, DERIVED_FROM_DIRECTIVE) {
                return Err(SchemaError::invalid(
                    location.to_owned(),
                    "`@derivedFrom` is only valid on a field that references an entity",
                ));
            }
            None
        }
    };

    Ok(FieldDefinition {
        name: field.name.clone(),
        description: field.description.clone(),
        field_type,
        nullable,
        relation,
    })
}

/// Resolve a GraphQL type to `(type, nullable)`.
///
/// GraphQL defaults to nullable and spells non-null with `!`, so nullability
/// is decided by the *outermost* wrapper.
fn resolve_type(
    ty: &GqlType<'_, String>,
    location: &str,
    entity_names: &BTreeSet<String>,
    enum_names: &BTreeSet<String>,
) -> SchemaResult<(FieldType, bool)> {
    match ty {
        GqlType::NonNullType(inner) => {
            let (resolved, _) = resolve_type(inner, location, entity_names, enum_names)?;
            Ok((resolved, false))
        }
        GqlType::ListType(inner) => {
            let (element, nullable_elements) =
                resolve_type(inner, location, entity_names, enum_names)?;
            Ok((
                FieldType::List {
                    inner: Box::new(element),
                    nullable_elements,
                },
                true,
            ))
        }
        GqlType::NamedType(name) => {
            let resolved = if let Some(scalar) = ScalarKind::from_graphql_name(name) {
                FieldType::Scalar { scalar }
            } else if entity_names.contains(name) {
                FieldType::Entity {
                    entity: name.clone(),
                }
            } else if enum_names.contains(name) {
                FieldType::Enum { name: name.clone() }
            } else {
                return Err(SchemaError::invalid(
                    location.to_owned(),
                    format!("unknown type `{name}`"),
                )
                .with_help(
                    "use a built-in scalar (ID, String, Boolean, Int, BigInt, Float, \
                     BigDecimal, Bytes, Date, Json), or declare the type as an `@entity` or `enum`",
                ));
            };
            Ok((resolved, true))
        }
    }
}

fn has_directive(directives: &[Directive<'_, String>], name: &str) -> bool {
    directives.iter().any(|d| d.name == name)
}

fn directive_str(directive: &Directive<'_, String>, arg: &str) -> Option<String> {
    directive
        .arguments
        .iter()
        .find(|(k, _)| k == arg)
        .and_then(|(_, v)| match v {
            GqlValue::String(s) => Some(s.clone()),
            GqlValue::Enum(s) => Some(s.clone()),
            _ => None,
        })
}

fn directive_bool(directive: &Directive<'_, String>, arg: &str) -> Option<bool> {
    directive
        .arguments
        .iter()
        .find(|(k, _)| k == arg)
        .and_then(|(_, v)| match v {
            GqlValue::Boolean(b) => Some(*b),
            _ => None,
        })
}
