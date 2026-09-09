//! Entity relations.

use serde::{Deserialize, Serialize};

/// A relation between two entities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relation {
    /// The entity on the other end.
    pub target: String,
    /// Which direction this relation points.
    pub kind: RelationKind,
    /// For derived relations, the field on the target that points back here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derived_from: Option<String>,
}

/// The shape of a relation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RelationKind {
    /// This entity stores the foreign key: `owner: Account!`.
    ///
    /// Becomes a stored column plus an index.
    Owned,
    /// The relation is computed from the other side: `@derivedFrom(field: "owner")`.
    ///
    /// Stores nothing; the query service resolves it by reverse lookup.
    Derived,
}
