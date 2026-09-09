//! The payloads a mapping handler receives.
//!
//! These types cross the mapping ABI, so their serde representation is part of
//! `docs/spec/mapping-abi-v1.md`. `EvmLog<T>` is generic over the decoded
//! event so a handler signature states exactly which event it wants.

use alloy_primitives::{Address, B256, Bytes, U256};
use serde::{Deserialize, Serialize};

/// A block header, as delivered to a `evm/BlockHandler`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvmBlock {
    /// Block height.
    pub number: u64,
    /// Block hash.
    pub hash: B256,
    /// Parent block hash.
    pub parent_hash: B256,
    /// Unix timestamp in seconds.
    pub timestamp: u64,
    /// Gas used by the block.
    pub gas_used: U256,
    /// Block gas limit.
    pub gas_limit: U256,
}

/// A transaction, as delivered to a `evm/TransactionHandler`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvmTransaction {
    /// Transaction hash.
    pub hash: B256,
    /// Index within the block.
    pub transaction_index: u64,
    /// Sender.
    pub from: Address,
    /// Recipient; `None` for contract creation.
    pub to: Option<Address>,
    /// Value transferred, in wei.
    pub value: U256,
    /// Calldata.
    pub input: Bytes,
    /// The block this transaction was mined in.
    pub block: EvmBlock,
}

/// An event log with its decoded parameters.
///
/// `T` is the generated struct for the event, produced by `superquery codegen`
/// from the contract ABI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvmLog<T> {
    /// The contract that emitted the log.
    pub address: Address,
    /// Index of this log within the block.
    pub log_index: u64,
    /// Raw topics, topic0 first.
    pub topics: Vec<B256>,
    /// Raw, undecoded data.
    pub data: Bytes,
    /// Hash of the transaction that produced this log.
    pub transaction_hash: B256,
    /// The block this log was emitted in.
    pub block: EvmBlock,
    /// The decoded event parameters.
    pub params: T,
}

impl<T> EvmLog<T> {
    /// A stable, unique id for this log, suitable as an entity `id`.
    ///
    /// `<transaction hash>-<log index>` is unique within a chain and stable
    /// across re-indexing, which is exactly what an entity key needs.
    pub fn id(&self) -> String {
        format!("{}-{}", self.transaction_hash, self.log_index)
    }

    /// The height this log was emitted at.
    pub const fn block_number(&self) -> u64 {
        self.block.number
    }
}
