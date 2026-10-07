//! Turning a handler payload into the handler's argument.
//!
//! `#[handler]` calls [`HandlerInput::from_payload`] on the bytes the host
//! passed in. A failure here is the host's fault (it routed the wrong item to
//! this handler), so it surfaces as [`Error::Payload`] and handler status `2`,
//! distinct from an error the mapping itself returned.

use crate::error::{Error, Result};

/// Implemented by every type a `#[handler]` function may take.
pub trait HandlerInput: Sized {
    /// Decode the handler payload.
    fn from_payload(payload: &[u8]) -> Result<Self>;
}

/// Decode a payload whose JSON shape is the type's own serde form.
pub(crate) fn from_json<T: serde::de::DeserializeOwned>(payload: &[u8]) -> Result<T> {
    serde_json::from_slice(payload).map_err(|e| Error::Payload(e.to_string()))
}

#[cfg(feature = "evm")]
mod evm {
    use alloy_sol_types::SolEvent;
    use superquery_evm::{EvmBlock, EvmLog, EvmTransaction, RawEvmLog};

    use super::{HandlerInput, from_json};
    use crate::error::{Error, Result};

    impl<T: SolEvent> HandlerInput for EvmLog<T> {
        fn from_payload(payload: &[u8]) -> Result<Self> {
            let raw: RawEvmLog = from_json(payload)?;
            EvmLog::decode(raw).map_err(|e| Error::Payload(e.to_string()))
        }
    }

    impl HandlerInput for RawEvmLog {
        fn from_payload(payload: &[u8]) -> Result<Self> {
            from_json(payload)
        }
    }

    impl HandlerInput for EvmBlock {
        fn from_payload(payload: &[u8]) -> Result<Self> {
            from_json(payload)
        }
    }

    impl HandlerInput for EvmTransaction {
        fn from_payload(payload: &[u8]) -> Result<Self> {
            from_json(payload)
        }
    }
}

#[cfg(all(test, feature = "evm"))]
mod tests {
    use super::*;
    use superquery_evm::{EvmBlock, EvmLog};

    alloy_sol_types::sol! {
        #[derive(Debug, PartialEq, Eq)]
        event Transfer(address indexed from, address indexed to, uint256 value);
    }

    const BLOCK: &str = r#"{"number":5,"hash":"0x0000000000000000000000000000000000000000000000000000000000000001","parentHash":"0x0000000000000000000000000000000000000000000000000000000000000000","timestamp":1700000000,"gasUsed":"0x0","gasLimit":"0x1c9c380"}"#;

    #[test]
    fn a_block_payload_decodes_from_its_spec_json() {
        let block = EvmBlock::from_payload(BLOCK.as_bytes()).unwrap();
        assert_eq!((block.number, block.timestamp), (5, 1_700_000_000));
    }

    #[test]
    fn a_log_payload_for_the_wrong_event_is_a_payload_error() {
        let payload = format!(
            r#"{{"address":"0x0000000000000000000000000000000000000000","logIndex":0,
                "topics":["0x{}"],"data":"0x","transactionHash":"0x{}","block":{BLOCK}}}"#,
            "ab".repeat(32),
            "00".repeat(32),
        );
        let err = EvmLog::<Transfer>::from_payload(payload.as_bytes()).unwrap_err();
        assert!(matches!(err, Error::Payload(_)), "{err}");
    }

    #[test]
    fn malformed_json_is_a_payload_error() {
        assert!(matches!(
            EvmBlock::from_payload(b"{"),
            Err(Error::Payload(_))
        ));
    }
}
