//! The host on every target that is not `wasm32`.
//!
//! A [`HostBackend`] installed on the current thread receives exactly the
//! bytes the WASM imports would, so a mapping's unit tests run the same JSON
//! encoding and status handling as production. With nothing installed, every
//! call fails legibly instead of pretending to succeed.

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::{Error, Result};

/// The byte-level host interface, mirroring the `superquery` WASM imports.
pub trait HostBackend {
    /// `sq_store_set`: returns a store status.
    fn store_set(&self, key: &[u8], entity: &[u8]) -> u32;
    /// `sq_store_get`: the entity JSON, or `None` if absent.
    fn store_get(&self, key: &[u8]) -> Option<Vec<u8>>;
    /// `sq_store_remove`: returns a store status.
    fn store_remove(&self, key: &[u8]) -> u32;
    /// `sq_log`.
    fn log(&self, level: u32, message: &str);
    /// `sq_chain_current_block`: the block pointer JSON, if there is a block.
    fn current_block(&self) -> Option<Vec<u8>>;
    /// `sq_handler_error`.
    fn handler_error(&self, message: &str);
}

thread_local! {
    static BACKEND: RefCell<Option<Rc<dyn HostBackend>>> = const { RefCell::new(None) };
}

/// Install `backend` for the current thread until the guard drops.
///
/// Thread-local because `cargo test` runs tests on parallel threads, and two
/// tests must never see each other's store.
pub fn install(backend: Rc<dyn HostBackend>) -> BackendGuard {
    let previous = BACKEND.with(|slot| slot.borrow_mut().replace(backend));
    BackendGuard { previous }
}

/// Restores the previously installed backend when dropped.
#[must_use = "the backend is uninstalled when the guard drops"]
pub struct BackendGuard {
    previous: Option<Rc<dyn HostBackend>>,
}

impl Drop for BackendGuard {
    fn drop(&mut self) {
        let previous = self.previous.take();
        BACKEND.with(|slot| *slot.borrow_mut() = previous);
    }
}

fn with_backend<T>(call: &'static str, f: impl FnOnce(&dyn HostBackend) -> T) -> Result<T> {
    // Clone the handle out so a backend may itself call back into the host.
    let backend = BACKEND.with(|slot| slot.borrow().clone());
    match backend {
        Some(backend) => Ok(f(backend.as_ref())),
        None => Err(Error::Host {
            call,
            message: "no host attached; outside WASM, install one with \
                      `superquery_sdk::testing::TestHost`"
                .to_owned(),
        }),
    }
}

pub(crate) fn store_set(key: &[u8], entity: &[u8]) -> Result<u32> {
    with_backend("sq_store_set", |b| b.store_set(key, entity))
}

pub(crate) fn store_get(key: &[u8]) -> Result<Option<Vec<u8>>> {
    with_backend("sq_store_get", |b| b.store_get(key))
}

pub(crate) fn store_remove(key: &[u8]) -> Result<u32> {
    with_backend("sq_store_remove", |b| b.store_remove(key))
}

pub(crate) fn log(level: u32, message: &str) {
    // Logging without a host is not worth failing a test over; fall back to
    // stderr so the line is not lost.
    if with_backend("sq_log", |b| b.log(level, message)).is_err() {
        eprintln!("[superquery log {level}] {message}");
    }
}

pub(crate) fn current_block() -> Result<Option<Vec<u8>>> {
    with_backend("sq_chain_current_block", |b| b.current_block())
}

pub(crate) fn handler_error(message: &str) {
    let _ = with_backend("sq_handler_error", |b| b.handler_error(message));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    struct Counting(Cell<u32>);

    impl HostBackend for Counting {
        fn store_set(&self, _: &[u8], _: &[u8]) -> u32 {
            self.0.set(self.0.get() + 1);
            0
        }
        fn store_get(&self, _: &[u8]) -> Option<Vec<u8>> {
            None
        }
        fn store_remove(&self, _: &[u8]) -> u32 {
            0
        }
        fn log(&self, _: u32, _: &str) {}
        fn current_block(&self) -> Option<Vec<u8>> {
            None
        }
        fn handler_error(&self, _: &str) {}
    }

    #[test]
    fn calls_without_a_backend_fail_with_a_named_host_call() {
        let err = store_set(b"k", b"e").unwrap_err();
        assert!(
            matches!(
                err,
                Error::Host {
                    call: "sq_store_set",
                    ..
                }
            ),
            "{err}"
        );
    }

    #[test]
    fn an_installed_backend_receives_calls_until_its_guard_drops() {
        let backend = Rc::new(Counting(Cell::new(0)));
        {
            let _guard = install(backend.clone());
            assert_eq!(store_set(b"k", b"e").unwrap(), 0);
            assert_eq!(store_set(b"k", b"e").unwrap(), 0);
        }
        assert_eq!(backend.0.get(), 2);
        assert!(store_set(b"k", b"e").is_err(), "guard drop uninstalls");
    }

    #[test]
    fn nested_installs_restore_the_outer_backend() {
        let outer = Rc::new(Counting(Cell::new(0)));
        let inner = Rc::new(Counting(Cell::new(0)));
        let _outer = install(outer.clone());
        {
            let _inner = install(inner.clone());
            store_set(b"k", b"e").unwrap();
        }
        store_set(b"k", b"e").unwrap();
        assert_eq!((outer.0.get(), inner.0.get()), (1, 1));
    }
}
