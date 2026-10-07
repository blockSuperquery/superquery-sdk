//! The payloads a mapping handler receives.
//!
//! [`EvmBlock`], [`EvmTransaction`] and [`RawEvmLog`] cross the mapping ABI,
//! so their serde representation is part of `docs/spec/mapping-abi-v1.md`.
//! [`EvmLog<T>`] does not: the guest builds it from a `RawEvmLog` by decoding
//! the topics and data against the handler's event type. That keeps the
//! node free of every project's contract ABIs.

use alloy_primitives::{Address, B256, Bytes, U256};
use alloy_sol_types::SolEvent;
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

/// An event log exactly as the host delivers it to an `evm/LogHandler`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawEvmLog {
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
}

/// An event log with its decoded parameters.
///
/// `T` is the generated struct for the event, produced by `superquery codegen`
/// from the contract ABI.
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl<T: SolEvent> EvmLog<T> {
    /// Decode a raw log as event `T`.
    ///
    /// Fails if topic0 is not `T`'s signature hash or the topics and data do
    /// not ABI-decode as `T`. Either means the host routed a log to the wrong
    /// handler, which the mapping cannot recover from.
    pub fn decode(raw: RawEvmLog) -> Result<Self, LogDecodeError> {
        let params = T::decode_raw_log_validate(raw.topics.iter().copied(), &raw.data).map_err(
            |source| LogDecodeError {
                event: T::SIGNATURE,
                message: source.to_string(),
            },
        )?;
        Ok(Self {
            address: raw.address,
            log_index: raw.log_index,
            topics: raw.topics,
            data: raw.data,
            transaction_hash: raw.transaction_hash,
            block: raw.block,
            params,
        })
    }
}

/// A raw log did not decode as the handler's event.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("log does not decode as `{event}`: {message}")]
pub struct LogDecodeError {
    /// The event signature the handler expected.
    pub event: &'static str,
    /// The decoder's explanation.
    pub message: String,
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{address, b256};

    alloy_sol_types::sol! {
        #[derive(Debug, PartialEq, Eq)]
        event Transfer(address indexed from, address indexed to, uint256 value);
        #[derive(Debug, PartialEq, Eq)]
        event Approval(address indexed owner, address indexed spender, uint256 value);
    }

    fn block() -> EvmBlock {
        EvmBlock {
            number: 1,
            hash: B256::ZERO,
            parent_hash: B256::ZERO,
            timestamp: 0,
            gas_used: U256::ZERO,
            gas_limit: U256::ZERO,
        }
    }

    fn transfer_log() -> RawEvmLog {
        let from = address!("0x00000000000000000000000000000000000000aa");
        let to = address!("0x00000000000000000000000000000000000000bb");
        RawEvmLog {
            address: Address::ZERO,
            log_index: 3,
            topics: vec![Transfer::SIGNATURE_HASH, from.into_word(), to.into_word()],
            data: U256::from(1_000_000u64).to_be_bytes::<32>().to_vec().into(),
            transaction_hash: b256!(
                "0x1111111111111111111111111111111111111111111111111111111111111111"
            ),
            block: block(),
        }
    }

    #[test]
    fn a_raw_log_decodes_into_typed_event_parameters() {
        let log = EvmLog::<Transfer>::decode(transfer_log()).unwrap();
        assert_eq!(
            log.params.from,
            address!("0x00000000000000000000000000000000000000aa")
        );
        assert_eq!(
            log.params.to,
            address!("0x00000000000000000000000000000000000000bb")
        );
        assert_eq!(log.params.value, U256::from(1_000_000u64));
        assert_eq!(log.log_index, 3);
    }

    #[test]
    fn a_log_for_a_different_event_is_rejected_with_the_expected_signature() {
        let err = EvmLog::<Approval>::decode(transfer_log()).unwrap_err();
        assert_eq!(err.event, "Approval(address,address,uint256)");
    }

    #[test]
    fn the_log_id_is_the_transaction_hash_and_log_index() {
        let log = EvmLog::<Transfer>::decode(transfer_log()).unwrap();
        assert_eq!(log.id(), format!("0x{}-3", "11".repeat(32)));
    }

    #[test]
    fn the_raw_log_wire_format_is_camel_case_hex() {
        let json = serde_json::to_value(transfer_log()).unwrap();
        assert!(json.get("logIndex").is_some(), "{json}");
        assert!(
            json.get("transactionHash")
                .unwrap()
                .as_str()
                .unwrap()
                .starts_with("0x")
        );
        let back: RawEvmLog = serde_json::from_value(json).unwrap();
        assert_eq!(back, transfer_log());
    }
}
