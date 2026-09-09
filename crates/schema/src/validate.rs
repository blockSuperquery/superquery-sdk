//! Post-parse schema checks.
//!
//! The parser already rejects anything it cannot resolve. What is left are the
//! cross-entity rules: a `@derivedFrom` must name a field that exists on the
//! target and actually points back here.

use crate::error::{SchemaError, SchemaResult};
use crate::ir::SchemaIr;
use crate::relation::{Relation, RelationKind};

/// Check whole-schema invariants that a single entity cannot see.
pub fn validate(ir: &SchemaIr) -> SchemaResult<()> {
    for entity in &ir.entities {
        for field in &entity.fields {
            let Some(Relation {
                target,
                kind,
                derived_from,
            }) = &field.relation
            else {
                continue;
            };
            let location = format!("{}.{}", entity.name, field.name);

            let Some(target_entity) = ir.entity(target) else {
                return Err(SchemaError::invalid(
                    location,
                    format!("`{target}` is not an entity in this schema"),
                ));
            };

            if *kind != RelationKind::Derived {
                continue;
            }
            let Some(back_field_name) = derived_from else {
                continue;
            };

            let Some(back_field) = target_entity.field(back_field_name) else {
                return Err(SchemaError::invalid(
                    location,
                    format!("`{target}` has no field `{back_field_name}`"),
                )
                .with_help("`@derivedFrom(field:)` must name a field on the target entity"));
            };

            if back_field.field_type.referenced_entity() != Some(entity.name.as_str()) {
                return Err(SchemaError::invalid(
                    location,
                    format!(
                        "`{target}.{back_field_name}` does not point back at `{}`",
                        entity.name
                    ),
                )
                .with_help("a derived field is resolved by reverse lookup, so the named field must reference this entity"));
            }
        }
    }
    Ok(())
}
