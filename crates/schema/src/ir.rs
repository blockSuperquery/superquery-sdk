//! The canonical schema intermediate representation.
//!
//! This is a plain data structure with no behaviour beyond lookups: it is
//! serialised to `dist/schema.ir.json` and read by tools that are not this
//! crate, so it must stay boring and stable.

use serde::{Deserialize, Serialize};
use superquery_types::ScalarKind;

use crate::index::IndexDefinition;
use crate::relation::Relation;

/// A whole project schema, resolved.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaIr {
    /// Entities, sorted by name so the IR is order-independent.
    pub entities: Vec<EntityDefinition>,
    /// Enum types declared in the schema, sorted by name.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enums: Vec<EnumDefinition>,
}

impl SchemaIr {
    /// Look up an entity by its schema name.
    pub fn entity(&self, name: &str) -> Option<&EntityDefinition> {
        self.entities.iter().find(|e| e.name == name)
    }

    /// Look up an enum by its schema name.
    pub fn enum_type(&self, name: &str) -> Option<&EnumDefinition> {
        self.enums.iter().find(|e| e.name == name)
    }

    /// Whether a type name refers to something declared in this schema.
    pub fn defines(&self, name: &str) -> bool {
        self.entity(name).is_some() || self.enum_type(name).is_some()
    }
}

/// One `type X @entity { .. }` declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityDefinition {
    /// The type name as written in the schema, e.g. `Transfer`.
    pub name: String,
    /// Doc comment / description attached to the type, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Fields in declaration order — developers expect generated structs to
    /// match the order they wrote.
    pub fields: Vec<FieldDefinition>,
    /// Indexes, both explicit (`@index`) and implied (relations, `id`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indexes: Vec<IndexDefinition>,
    /// Whether the entity keeps historical versions (`@entity(immutable: true)`
    /// opts out of the write-behind history the node maintains).
    #[serde(default)]
    pub immutable: bool,
}

impl EntityDefinition {
    /// Look up a field by name.
    pub fn field(&self, name: &str) -> Option<&FieldDefinition> {
        self.fields.iter().find(|f| f.name == name)
    }

    /// The `id` field. Every entity is required to have one.
    pub fn id_field(&self) -> Option<&FieldDefinition> {
        self.field("id")
    }
}

/// One field of an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefinition {
    /// The field name as written in the schema.
    pub name: String,
    /// Doc comment / description, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The resolved type of this field.
    #[serde(rename = "type")]
    pub field_type: FieldType,
    /// `false` when the schema wrote `T!`.
    pub nullable: bool,
    /// Set when the field is a relation to another entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<Relation>,
}

/// A resolved field type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FieldType {
    /// A built-in scalar.
    Scalar {
        /// Which scalar.
        scalar: ScalarKind,
    },
    /// A reference to another entity in this schema.
    Entity {
        /// The referenced entity's name.
        entity: String,
    },
    /// A reference to an enum declared in this schema.
    Enum {
        /// The enum's name.
        name: String,
    },
    /// A list of the inner type.
    List {
        /// Element type.
        inner: Box<FieldType>,
        /// Whether elements may be null.
        nullable_elements: bool,
    },
}

impl FieldType {
    /// The innermost non-list type.
    pub fn base(&self) -> &FieldType {
        match self {
            FieldType::List { inner, .. } => inner.base(),
            other => other,
        }
    }

    /// Whether this type is a list at the outermost level.
    pub const fn is_list(&self) -> bool {
        matches!(self, FieldType::List { .. })
    }

    /// The name of the entity this type ultimately refers to, if any.
    pub fn referenced_entity(&self) -> Option<&str> {
        match self.base() {
            FieldType::Entity { entity } => Some(entity),
            _ => None,
        }
    }
}

/// One `enum X { .. }` declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumDefinition {
    /// The enum's name.
    pub name: String,
    /// Variants in declaration order.
    pub values: Vec<String>,
}
