//! The real boundary: WASM imports and exports.
//!
//! Compiled only for `wasm32`. This is the single place in the SDK that
//! touches raw pointers, which is why it alone is allowed `unsafe`.

use std::alloc::{Layout, alloc, dealloc};
use std::ptr::NonNull;

use crate::error::Result;
use crate::host::abi;

#[link(wasm_import_module = "superquery")]
unsafe extern "C" {
    fn sq_store_set(key_ptr: u32, key_len: u32, entity_ptr: u32, entity_len: u32) -> u32;
    fn sq_store_get(key_ptr: u32, key_len: u32) -> u64;
    fn sq_store_remove(key_ptr: u32, key_len: u32) -> u32;
    fn sq_log(level: u32, msg_ptr: u32, msg_len: u32);
    fn sq_chain_current_block() -> u64;
    fn sq_handler_error(msg_ptr: u32, msg_len: u32);
}

/// Declares the ABI version. The host calls this before anything else.
#[unsafe(no_mangle)]
pub extern "C" fn sq_mapping_abi_version() -> u32 {
    abi::version().get()
}

/// Allocate a buffer the host can write a payload into.
#[unsafe(no_mangle)]
pub extern "C" fn sq_alloc(len: u32) -> u32 {
    if len == 0 {
        return NonNull::<u8>::dangling().as_ptr() as u32;
    }
    let layout = Layout::array::<u8>(len as usize).expect("len fits in memory");
    // SAFETY: `layout` has a non-zero size, checked above.
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        std::alloc::handle_alloc_error(layout);
    }
    ptr as u32
}

/// Free a buffer from [`sq_alloc`].
#[unsafe(no_mangle)]
pub extern "C" fn sq_free(ptr: u32, len: u32) {
    if len == 0 {
        return;
    }
    let layout = Layout::array::<u8>(len as usize).expect("len fits in memory");
    // SAFETY: the ABI requires `ptr` came from `sq_alloc(len)`, which used
    // exactly this layout.
    unsafe { dealloc(ptr as *mut u8, layout) }
}

/// Take ownership of a buffer the host allocated with `sq_alloc` and filled.
///
/// # Safety
///
/// `(ptr, len)` must come from `sq_alloc(len)` and not be freed elsewhere.
pub unsafe fn take_buffer(ptr: u32, len: u32) -> Vec<u8> {
    if len == 0 {
        return Vec::new();
    }
    // SAFETY: `sq_alloc` allocated `len` bytes with the global allocator at
    // `Layout::array::<u8>(len)`, which is the layout a `Vec<u8>` of capacity
    // `len` uses, and the caller guarantees the host initialised them.
    unsafe { Vec::from_raw_parts(ptr as *mut u8, len as usize, len as usize) }
}

fn parts(bytes: &[u8]) -> (u32, u32) {
    (bytes.as_ptr() as u32, bytes.len() as u32)
}

fn take_packed(packed: u64) -> Option<Vec<u8>> {
    let (ptr, len) = abi::unpack(packed)?;
    // SAFETY: the ABI says a packed return was allocated with `sq_alloc` and
    // is now owned by the guest.
    Some(unsafe { take_buffer(ptr, len) })
}

pub(crate) fn store_set(key: &[u8], entity: &[u8]) -> Result<u32> {
    let ((kp, kl), (ep, el)) = (parts(key), parts(entity));
    // SAFETY: both buffers are live for the call; the host only reads them.
    Ok(unsafe { sq_store_set(kp, kl, ep, el) })
}

pub(crate) fn store_get(key: &[u8]) -> Result<Option<Vec<u8>>> {
    let (kp, kl) = parts(key);
    // SAFETY: the key is live for the call; the returned buffer is ours.
    Ok(take_packed(unsafe { sq_store_get(kp, kl) }))
}

pub(crate) fn store_remove(key: &[u8]) -> Result<u32> {
    let (kp, kl) = parts(key);
    // SAFETY: the key is live for the call.
    Ok(unsafe { sq_store_remove(kp, kl) })
}

pub(crate) fn log(level: u32, message: &str) {
    let (mp, ml) = parts(message.as_bytes());
    // SAFETY: the message is live for the call.
    unsafe { sq_log(level, mp, ml) }
}

pub(crate) fn current_block() -> Result<Option<Vec<u8>>> {
    // SAFETY: no arguments; the returned buffer is ours.
    Ok(take_packed(unsafe { sq_chain_current_block() }))
}

pub(crate) fn handler_error(message: &str) {
    let (mp, ml) = parts(message.as_bytes());
    // SAFETY: the message is live for the call.
    unsafe { sq_handler_error(mp, ml) }
}
