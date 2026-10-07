//! Host calls for entity storage: `sq_store_*`.
//!
//! Owns the JSON encoding of keys and entities and the translation of store
//! statuses into errors; the bytes themselves move through `raw`.

use superquery_types::{Entity, EntityKey};

use crate::error::{Error, Result};
use crate::host::abi::StoreStatus;
use crate::host::raw;

/// Import name for the write call.
pub const IMPORT_SET: &str = "sq_store_set";
/// Import name for the read call.
pub const IMPORT_GET: &str = "sq_store_get";
/// Import name for the delete call.
pub const IMPORT_REMOVE: &str = "sq_store_remove";

/// Write an entity through the host.
pub fn set(key: &EntityKey, entity: &Entity) -> Result {
    let entity_json = encode(IMPORT_SET, key, entity)?;
    let status = raw::store_set(&encode(IMPORT_SET, key, key)?, &entity_json)?;
    check(IMPORT_SET, key, status)
}

/// Read an entity through the host.
pub fn get(key: &EntityKey) -> Result<Option<Entity>> {
    let Some(bytes) = raw::store_get(&encode(IMPORT_GET, key, key)?)? else {
        return Ok(None);
    };
    serde_json::from_slice(&bytes)
        .map(Some)
        .map_err(|e| Error::Entity {
            operation: "decode",
            entity: key.entity.clone(),
            message: format!("host returned malformed JSON: {e}"),
        })
}

/// Delete an entity through the host.
pub fn remove(key: &EntityKey) -> Result {
    let status = raw::store_remove(&encode(IMPORT_REMOVE, key, key)?)?;
    check(IMPORT_REMOVE, key, status)
}

fn encode(call: &'static str, key: &EntityKey, value: &impl serde::Serialize) -> Result<Vec<u8>> {
    serde_json::to_vec(value).map_err(|e| Error::Entity {
        operation: "encode",
        entity: key.entity.clone(),
        message: format!("{call}: {e}"),
    })
}

fn check(call: &'static str, key: &EntityKey, raw_status: u32) -> Result {
    match StoreStatus::from_raw(raw_status) {
        Some(StoreStatus::Ok) => Ok(()),
        Some(status) => Err(Error::Host {
            call,
            message: format!("{key}: {}", status.describe()),
        }),
        None => Err(Error::Host {
            call,
            message: format!("{key}: unknown status {raw_status} from a newer host"),
        }),
    }
}
