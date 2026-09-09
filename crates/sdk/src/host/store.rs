//! Host calls for entity storage: `sq_store_*`.

use superquery_types::{Entity, EntityKey};

use crate::error::{Error, Result};

/// Import name for the write call.
pub const IMPORT_SET: &str = "sq_store_set";
/// Import name for the read call.
pub const IMPORT_GET: &str = "sq_store_get";
/// Import name for the delete call.
pub const IMPORT_REMOVE: &str = "sq_store_remove";

/// Write an entity through the host.
pub fn set(_key: &EntityKey, _entity: &Entity) -> Result {
    Err(unimplemented(IMPORT_SET))
}

/// Read an entity through the host.
pub fn get(_key: &EntityKey) -> Result<Option<Entity>> {
    Err(unimplemented(IMPORT_GET))
}

/// Delete an entity through the host.
pub fn remove(_key: &EntityKey) -> Result {
    Err(unimplemented(IMPORT_REMOVE))
}

/// Placeholder until the extern block lands in Milestone 6.
///
/// Returning an error rather than panicking keeps the failure legible if a
/// mapping is run against a host that predates the call.
fn unimplemented(call: &'static str) -> Error {
    Error::Host {
        call,
        message: "host call not yet implemented in this SDK build (Milestone 6)".to_owned(),
    }
}
