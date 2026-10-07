//! Host calls for logging: `sq_log`.
//!
//! A mapping cannot write to stdout — the node owns the process's output. Logs
//! go through the host so they carry the block and handler that produced them.

use crate::host::raw;

/// Import name for the log call.
pub const IMPORT_LOG: &str = "sq_log";

/// Severity of a mapping log line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Level {
    /// Verbose detail, off by default.
    Debug = 0,
    /// Normal progress.
    Info = 1,
    /// Something suspicious that did not stop indexing.
    Warn = 2,
    /// Something that did.
    Error = 3,
}

impl Level {
    /// Interpret a raw level from the wire.
    pub const fn from_raw(raw: u32) -> Option<Self> {
        Some(match raw {
            0 => Self::Debug,
            1 => Self::Info,
            2 => Self::Warn,
            3 => Self::Error,
            _ => return None,
        })
    }
}

/// Emit a log line through the host.
pub fn log(level: Level, message: &str) {
    raw::log(level as u32, message);
}

/// Log at [`Level::Debug`].
pub fn debug(message: &str) {
    log(Level::Debug, message);
}

/// Log at [`Level::Info`].
pub fn info(message: &str) {
    log(Level::Info, message);
}

/// Log at [`Level::Warn`].
pub fn warn(message: &str) {
    log(Level::Warn, message);
}

/// Log at [`Level::Error`].
pub fn error(message: &str) {
    log(Level::Error, message);
}
