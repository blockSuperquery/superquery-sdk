//! Host calls for logging: `sq_log`.
//!
//! A mapping cannot write to stdout — the node owns the process's output. Logs
//! go through the host so they carry the block and handler that produced them.

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

/// Emit a log line through the host.
///
/// Scaffold: writes nothing until the extern block lands (Milestone 6).
pub fn log(_level: Level, _message: &str) {}
