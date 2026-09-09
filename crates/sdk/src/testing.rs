//! Unit-testing helpers for mappings.
//!
//! Scaffold for Milestone 10: the point is that a developer can assert on
//! store contents without Postgres or an RPC endpoint. The in-memory store
//! below stands in for the host during `cargo test`.

use std::collections::BTreeMap;
use std::sync::Mutex;

use superquery_types::{Entity as UntypedEntity, EntityKey};

/// An in-memory stand-in for the node's entity store.
#[derive(Debug, Default)]
pub struct TestStore {
    entities: Mutex<BTreeMap<EntityKey, UntypedEntity>>,
}

impl TestStore {
    /// An empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace an entity.
    pub fn set(&self, key: EntityKey, entity: UntypedEntity) {
        self.entities
            .lock()
            .expect("test store lock")
            .insert(key, entity);
    }

    /// Read an entity back.
    pub fn get(&self, key: &EntityKey) -> Option<UntypedEntity> {
        self.entities
            .lock()
            .expect("test store lock")
            .get(key)
            .cloned()
    }

    /// Remove an entity.
    pub fn remove(&self, key: &EntityKey) -> Option<UntypedEntity> {
        self.entities.lock().expect("test store lock").remove(key)
    }

    /// How many entities are stored.
    pub fn len(&self) -> usize {
        self.entities.lock().expect("test store lock").len()
    }

    /// Whether the store is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use superquery_types::{EntityId, Value};

    #[test]
    fn stores_and_reads_back_an_entity() {
        let store = TestStore::new();
        let key = EntityKey::new("Transfer", EntityId::new("0x1-0").unwrap());

        let mut entity = UntypedEntity::new();
        entity.set("id", "0x1-0");
        entity.set("value", Value::BigInt("100".into()));

        store.set(key.clone(), entity.clone());
        assert_eq!(store.get(&key), Some(entity));
        assert_eq!(store.len(), 1);

        store.remove(&key);
        assert!(store.is_empty());
    }
}
