//! The GraphQL entity schema and its canonical intermediate representation.
//!
//! A project's `schema.graphql` is parsed exactly once, into [`SchemaIr`].
//! Three consumers read that IR and nothing else:
//!
//! 1. the SDK's entity codegen (`superquery-codegen`),
//! 2. the node's table and migration planning,
//! 3. the query service's GraphQL schema generation.
//!
//! Keeping one parser and one IR is what stops the platform from growing three
//! subtly incompatible ideas of what a schema means. If you find yourself
//! reaching for `graphql_parser` outside this crate, add to the IR instead.
//!
//! Milestone 3 of `docs/IMPLEMENTATION_PLAN.md`; specified in
//! `docs/spec/schema-v1.md`.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod canonical;
pub mod error;
pub mod index;
pub mod ir;
pub mod parser;
pub mod relation;
pub mod validate;

pub use canonical::{canonical_json, schema_hash};
pub use error::{SchemaError, SchemaResult};
pub use index::{IndexDefinition, IndexKind};
pub use ir::{EntityDefinition, EnumDefinition, FieldDefinition, FieldType, SchemaIr};
pub use parser::{parse, parse_file};
pub use relation::{Relation, RelationKind};
pub use validate::validate;
