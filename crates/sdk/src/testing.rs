//! Unit-testing helpers for mappings.
//!
//! [`TestStore`] is a complete in-memory host. Installed on the current
//! thread, it receives the same JSON bytes the node would over the WASM
//! boundary, so `entity.save()` in a test runs the production encoding — no
//! Postgres, no RPC.
//!
//! ```ignore
//! let store = TestStore::new();
//! let _host = store.install();
//! block_on(handle_transfer(fixture))?;
//! assert_eq!(store.entity::<Transfer>("0xabc-0").unwrap().value, BigInt::from(100u64));
//! ```

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use superquery_types::{BlockPtr, Entity as UntypedEntity, EntityId, EntityKey};

#[cfg(not(target_arch = "wasm32"))]
use crate::host::abi::StoreStatus;
use crate::host::log::Level;
use crate::store::Entity;

pub use crate::executor::block_on;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::host::native::BackendGuard;

/// An in-memory stand-in for the node: entity store, logs and block context.
///
/// Cheap to clone; clones share state, so a test keeps one handle while the
/// installed copy serves the mapping.
#[derive(Debug, Clone, Default)]
pub struct TestStore {
    state: Rc<RefCell<State>>,
}

#[derive(Debug, Default)]
struct State {
    entities: BTreeMap<EntityKey, UntypedEntity>,
    logs: Vec<(Level, String)>,
    handler_errors: Vec<String>,
    block: Option<BlockPtr>,
}

impl TestStore {
    /// An empty store with no current block.
    pub fn new() -> Self {
        Self::default()
    }

    /// Serve host calls on the current thread until the guard drops.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn install(&self) -> BackendGuard {
        crate::host::native::install(Rc::new(self.clone()))
    }

    /// Set the block `sq_chain_current_block` reports.
    pub fn set_block(&self, block: BlockPtr) {
        self.state.borrow_mut().block = Some(block);
    }

    /// Insert or replace an entity directly, e.g. to seed state.
    pub fn set(&self, key: EntityKey, entity: UntypedEntity) {
        self.state.borrow_mut().entities.insert(key, entity);
    }

    /// Read an entity back in its untyped form.
    pub fn get(&self, key: &EntityKey) -> Option<UntypedEntity> {
        self.state.borrow().entities.get(key).cloned()
    }

    /// Read an entity back as its generated type.
    ///
    /// # Panics
    ///
    /// If `id` is empty or the stored entity no longer decodes as `T`; both
    /// are test bugs worth failing loudly on.
    pub fn entity<T: Entity>(&self, id: &str) -> Option<T> {
        let id = EntityId::new(id).expect("entity ids are non-empty");
        let untyped = self.get(&EntityKey::new(T::NAME, id))?;
        Some(T::from_untyped(&untyped).expect("stored entity decodes as its type"))
    }

    /// Remove an entity.
    pub fn remove(&self, key: &EntityKey) -> Option<UntypedEntity> {
        self.state.borrow_mut().entities.remove(key)
    }

    /// How many entities are stored.
    pub fn len(&self) -> usize {
        self.state.borrow().entities.len()
    }

    /// Whether the store is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Every line logged through `sq_log`, in order.
    pub fn logs(&self) -> Vec<(Level, String)> {
        self.state.borrow().logs.clone()
    }

    /// Every message reported through `sq_handler_error`, in order.
    pub fn handler_errors(&self) -> Vec<String> {
        self.state.borrow().handler_errors.clone()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl crate::host::native::HostBackend for TestStore {
    fn store_set(&self, key: &[u8], entity: &[u8]) -> u32 {
        match (serde_json::from_slice(key), serde_json::from_slice(entity)) {
            (Ok(key), Ok(entity)) => {
                self.set(key, entity);
                StoreStatus::Ok as u32
            }
            _ => StoreStatus::InvalidPayload as u32,
        }
    }

    fn store_get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let key: EntityKey = serde_json::from_slice(key).expect("the SDK encodes valid keys");
        let entity = self.get(&key)?;
        Some(serde_json::to_vec(&entity).expect("entities always serialise"))
    }

    fn store_remove(&self, key: &[u8]) -> u32 {
        match serde_json::from_slice(key) {
            Ok(key) => {
                self.remove(&key);
                StoreStatus::Ok as u32
            }
            Err(_) => StoreStatus::InvalidPayload as u32,
        }
    }

    fn log(&self, level: u32, message: &str) {
        let level = Level::from_raw(level).unwrap_or(Level::Error);
        self.state
            .borrow_mut()
            .logs
            .push((level, message.to_owned()));
    }

    fn current_block(&self) -> Option<Vec<u8>> {
        let block = self.state.borrow().block.clone()?;
        Some(serde_json::to_vec(&block).expect("block pointers always serialise"))
    }

    fn handler_error(&self, message: &str) {
        self.state
            .borrow_mut()
            .handler_errors
            .push(message.to_owned());
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::host::{chain, log};
    use crate::store::{self, Store};
    use crate::{Error, Result};
    use superquery_types::{BigInt, BlockHash, FromValue, ToValue, Value};

    #[derive(Debug, Clone, PartialEq)]
    struct Transfer {
        id: String,
        value: BigInt,
    }

    impl Entity for Transfer {
        const NAME: &'static str = "Transfer";

        fn id(&self) -> Result<EntityId> {
            crate::__private::entity_id(Self::NAME, &self.id)
        }

        fn to_untyped(&self) -> UntypedEntity {
            let mut e = UntypedEntity::new();
            e.set("id", self.id.to_value());
            e.set("value", self.value.to_value());
            e
        }

        fn from_untyped(e: &UntypedEntity) -> Result<Self> {
            let field = |name: &str| e.get(name).cloned().unwrap_or(Value::Null);
            let decode = |err: superquery_types::FromValueError| Error::mapping(err.to_string());
            Ok(Self {
                id: String::from_value(&field("id")).map_err(decode)?,
                value: BigInt::from_value(&field("value")).map_err(decode)?,
            })
        }
    }

    fn transfer() -> Transfer {
        Transfer {
            id: "0x1-0".into(),
            value: BigInt::from(100u64),
        }
    }

    #[test]
    fn save_get_and_remove_round_trip_through_the_json_boundary() {
        let store = TestStore::new();
        let _host = store.install();

        block_on(transfer().save()).unwrap();
        assert_eq!(store.entity::<Transfer>("0x1-0"), Some(transfer()));

        let loaded: Option<Transfer> =
            block_on(store::get(&EntityId::new("0x1-0").unwrap())).unwrap();
        assert_eq!(loaded, Some(transfer()));

        block_on(transfer().remove()).unwrap();
        assert!(store.is_empty());
    }

    #[test]
    fn reading_a_missing_entity_is_none_not_an_error() {
        let store = TestStore::new();
        let _host = store.install();
        let loaded: Option<Transfer> =
            block_on(store::get(&EntityId::new("nope").unwrap())).unwrap();
        assert_eq!(loaded, None);
    }

    #[test]
    fn saving_without_an_installed_host_fails_legibly() {
        let err = block_on(transfer().save()).unwrap_err();
        assert!(err.to_string().contains("no host attached"), "{err}");
    }

    #[test]
    fn logs_and_the_current_block_reach_the_test_host() {
        let store = TestStore::new();
        let _host = store.install();
        let block = BlockPtr::new(7, BlockHash::from_hex("0xaa").unwrap());
        store.set_block(block.clone());

        log::info("hello");
        assert_eq!(chain::current_block().unwrap(), block);
        assert_eq!(store.logs(), [(Level::Info, "hello".to_owned())]);
    }

    #[test]
    fn asking_for_the_block_outside_a_block_is_an_error() {
        let store = TestStore::new();
        let _host = store.install();
        assert!(chain::current_block().is_err());
    }
}
