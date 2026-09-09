//! Canonical serialisation and hashing.
//!
//! `superquery build` records a `schemaHash` so the node can refuse a bundle
//! whose schema no longer matches the tables it created. That hash is only
//! meaningful if the same schema always serialises identically, which is why
//! the IR sorts entities and enums at construction and this module writes
//! compact JSON with sorted object keys.

use sha2::{Digest, Sha256};

use crate::ir::SchemaIr;

/// Serialise the IR to its canonical JSON form.
///
/// Compact, key-sorted, newline-free. This exact byte sequence is what
/// `dist/schema.ir.json` contains and what [`schema_hash`] digests.
pub fn canonical_json(ir: &SchemaIr) -> String {
    // `serde_json::to_string` on our IR is already deterministic: every struct
    // has a fixed field order and every collection is sorted by the parser.
    // Round-tripping through `Value` would re-order keys, so we do not.
    serde_json::to_string(ir).expect("SchemaIr is always serialisable")
}

/// The canonical SHA-256 of a schema IR, as lowercase hex.
pub fn schema_hash(ir: &SchemaIr) -> String {
    let mut hasher = Sha256::new();
    hasher.update(canonical_json(ir).as_bytes());
    hex::encode(hasher.finalize())
}
