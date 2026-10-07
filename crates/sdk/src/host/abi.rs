//! ABI version negotiation, status codes and buffer packing.
//!
//! Everything here is a number the node also hard-codes, so each constant is
//! named after its row in `docs/spec/mapping-abi-v1.md`.

use superquery_types::{MAPPING_ABI_VERSION, MappingAbiVersion};

/// The WASM import module every host function lives in.
pub const IMPORT_MODULE: &str = "superquery";

/// The symbol a compiled mapping exports to declare its ABI version.
pub const ABI_VERSION_EXPORT: &str = "sq_mapping_abi_version";

/// The guest allocator export the host uses to pass buffers in.
pub const ALLOC_EXPORT: &str = "sq_alloc";

/// The guest deallocator export.
pub const FREE_EXPORT: &str = "sq_free";

/// Prefix of every generated handler export: `sq_handle_<function name>`.
pub const HANDLER_EXPORT_PREFIX: &str = "sq_handle_";

/// The version this SDK build implements.
pub const fn version() -> MappingAbiVersion {
    MAPPING_ABI_VERSION
}

/// Status returned by a `sq_handle_*` export.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum HandlerStatus {
    /// The handler succeeded.
    Ok = 0,
    /// The handler returned an error; its message went to `sq_handler_error`.
    Failed = 1,
    /// The payload did not decode into the handler's input type.
    BadPayload = 2,
}

/// Status returned by `sq_store_set` and `sq_store_remove`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum StoreStatus {
    /// The write was applied.
    Ok = 0,
    /// The entity type is not in the bundle's schema.
    UnknownEntity = 1,
    /// A field is unknown, missing, wrongly null or has the wrong variant.
    SchemaMismatch = 2,
    /// The key or entity was not valid JSON of the expected shape.
    InvalidPayload = 3,
}

impl StoreStatus {
    /// Interpret a raw status. `None` means the host sent a code this SDK
    /// does not know, which only a newer host could do.
    pub const fn from_raw(raw: u32) -> Option<Self> {
        Some(match raw {
            0 => Self::Ok,
            1 => Self::UnknownEntity,
            2 => Self::SchemaMismatch,
            3 => Self::InvalidPayload,
            _ => return None,
        })
    }

    /// What the status means, phrased for an error message.
    pub const fn describe(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::UnknownEntity => "the entity type is not in the bundle's schema",
            Self::SchemaMismatch => "the entity does not match its schema definition",
            Self::InvalidPayload => "the host could not decode the request",
        }
    }
}

/// Record the error message for the handler call in progress
/// (`sq_handler_error`).
///
/// Generated `#[handler]` exports call this; mapping code returns `Err`.
#[doc(hidden)]
pub fn report_handler_error(message: &str) {
    crate::host::raw::handler_error(message);
}

/// Pack a guest buffer into the single `u64` some imports return.
pub const fn pack(ptr: u32, len: u32) -> u64 {
    ((ptr as u64) << 32) | len as u64
}

/// Split a packed buffer into `(ptr, len)`. `None` for the "nothing" value.
pub const fn unpack(packed: u64) -> Option<(u32, u32)> {
    if packed == 0 {
        return None;
    }
    Some(((packed >> 32) as u32, packed as u32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packing_round_trips_and_zero_means_nothing() {
        assert_eq!(unpack(pack(0x1000, 42)), Some((0x1000, 42)));
        assert_eq!(unpack(pack(u32::MAX, u32::MAX)), Some((u32::MAX, u32::MAX)));
        assert_eq!(unpack(0), None);
    }

    #[test]
    fn store_statuses_round_trip_and_reject_unknown_codes() {
        for status in [
            StoreStatus::Ok,
            StoreStatus::UnknownEntity,
            StoreStatus::SchemaMismatch,
            StoreStatus::InvalidPayload,
        ] {
            assert_eq!(StoreStatus::from_raw(status as u32), Some(status));
        }
        assert_eq!(StoreStatus::from_raw(4), None);
    }
}
