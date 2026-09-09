//! Index metadata.
//!
//! The SDK does not create indexes — the node does. What the SDK owns is the
//! decision about *which* indexes a schema implies, so that decision is made
//! once, in the IR, rather than re-derived by every consumer.

use serde::{Deserialize, Serialize};

/// One index on an entity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexDefinition {
    /// Fields covered, in order.
    pub fields: Vec<String>,
    /// Why this index exists.
    pub kind: IndexKind,
    /// Whether the index enforces uniqueness.
    #[serde(default)]
    pub unique: bool,
}

impl IndexDefinition {
    /// The primary key index implied by `id: ID!`.
    pub fn primary_key() -> Self {
        Self {
            fields: vec!["id".to_owned()],
            kind: IndexKind::PrimaryKey,
            unique: true,
        }
    }

    /// The index implied by an owned relation field.
    pub fn foreign_key(field: impl Into<String>) -> Self {
        Self {
            fields: vec![field.into()],
            kind: IndexKind::ForeignKey,
            unique: false,
        }
    }

    /// An index the developer asked for with `@index`.
    pub fn explicit(fields: Vec<String>, unique: bool) -> Self {
        Self {
            fields,
            kind: IndexKind::Explicit,
            unique,
        }
    }
}

/// Why an index is in the IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IndexKind {
    /// Implied by the entity's `id` field.
    PrimaryKey,
    /// Implied by a relation field that stores a foreign key.
    ForeignKey,
    /// Requested by the developer via `@index`.
    Explicit,
}
