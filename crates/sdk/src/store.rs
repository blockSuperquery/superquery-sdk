//! The entity store, as a mapping sees it.
//!
//! Scaffold: the trait shape is settled, the host wiring is not. `save()`
//! currently round-trips through [`crate::host::store`], whose implementation
//! is a stub until Milestone 6.

use superquery_types::{Entity as UntypedEntity, EntityId, EntityKey};

use crate::error::Result;

/// Implemented by every generated entity struct.
///
/// `superquery codegen` emits the impl; a mapping author never writes one.
pub trait Entity: Sized {
    /// The entity's schema name, e.g. `Transfer`.
    const NAME: &'static str;

    /// This entity's primary key.
    fn id(&self) -> EntityId;

    /// Lower to the untyped form that crosses the mapping ABI.
    fn to_untyped(&self) -> UntypedEntity;

    /// Rebuild from the untyped form.
    fn from_untyped(entity: &UntypedEntity) -> Result<Self>;

    /// This entity's store address.
    fn key(&self) -> EntityKey {
        EntityKey::new(Self::NAME, self.id())
    }
}

/// Store operations available to a mapping.
///
/// Async because the host call may suspend; the node's runtime drives it. The
/// mapping never sees a connection, a transaction or a table.
#[allow(async_fn_in_trait)]
pub trait Store {
    /// Write this entity, inserting or replacing.
    async fn save(&self) -> Result;

    /// Delete this entity.
    async fn remove(&self) -> Result;
}

impl<T: Entity> Store for T {
    async fn save(&self) -> Result {
        crate::host::store::set(&self.key(), &self.to_untyped())
    }

    async fn remove(&self) -> Result {
        crate::host::store::remove(&self.key())
    }
}

/// Load an entity by id.
pub async fn get<T: Entity>(id: &EntityId) -> Result<Option<T>> {
    let key = EntityKey::new(T::NAME, id.clone());
    match crate::host::store::get(&key)? {
        Some(untyped) => T::from_untyped(&untyped).map(Some),
        None => Ok(None),
    }
}
