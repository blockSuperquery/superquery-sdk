//! ABI version negotiation.
//!
//! The generated `#[handler]` wrapper exports [`ABI_VERSION_EXPORT`] so the
//! node can read a mapping's ABI version before calling anything else, and
//! refuse an incompatible bundle with a clear message instead of a trap.

use superquery_types::{MAPPING_ABI_VERSION, MappingAbiVersion};

/// The symbol a compiled mapping exports to declare its ABI version.
pub const ABI_VERSION_EXPORT: &str = "sq_mapping_abi_version";

/// The version this SDK build implements.
pub const fn version() -> MappingAbiVersion {
    MAPPING_ABI_VERSION
}
