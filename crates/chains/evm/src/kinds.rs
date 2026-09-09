//! The `evm/*` datasource and handler kinds.

use superquery_chain_api::{KindRole, KindSpec};

/// Datasource kinds this integration accepts.
pub const DATA_SOURCE_KINDS: &[KindSpec] = &[KindSpec {
    kind: "evm/Runtime",
    role: KindRole::DataSource,
    summary: "a contract or address range on an EVM chain",
}];

/// Handler kinds this integration accepts.
pub const HANDLER_KINDS: &[KindSpec] = &[
    KindSpec {
        kind: "evm/LogHandler",
        role: KindRole::Handler,
        summary: "receives decoded event logs matching an `event:` filter",
    },
    KindSpec {
        kind: "evm/TransactionHandler",
        role: KindRole::Handler,
        summary: "receives transactions matching a `function:` filter",
    },
    KindSpec {
        kind: "evm/BlockHandler",
        role: KindRole::Handler,
        summary: "receives every block, or every Nth block with `modulo:`",
    },
];

/// Every kind, datasource and handler alike.
pub fn all() -> impl Iterator<Item = &'static KindSpec> {
    DATA_SOURCE_KINDS.iter().chain(HANDLER_KINDS)
}
