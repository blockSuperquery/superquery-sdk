//! `#[handler]` end to end: payload bytes in, entities in the store out.

use superquery_sdk::__private::HandlerStatus;
use superquery_sdk::prelude::*;
use superquery_sdk::testing::TestStore;

superquery_sdk::sol! {
    #[derive(Debug, PartialEq, Eq)]
    event Transfer(address indexed from, address indexed to, uint256 value);
}

#[derive(Clone, Debug, PartialEq, SuperQueryEntity)]
#[superquery(entity = "Transfer")]
pub struct TransferEntity {
    pub id: String,
    pub from: String,
    pub value: BigInt,
    #[superquery(rename = "blockNumber")]
    pub block_number: BigInt,
}

#[handler]
pub async fn handle_transfer(event: EvmLog<Transfer>) -> Result<()> {
    if event.params.value.is_zero() {
        return Err(Error::mapping("zero-value transfer"));
    }
    TransferEntity {
        id: event.id(),
        from: event.params.from.to_string(),
        value: BigInt::new(event.params.value.to_string())?,
        block_number: BigInt::from(event.block_number()),
    }
    .save()
    .await
}

#[handler]
pub fn count_blocks(block: EvmBlock) -> Result<()> {
    superquery_sdk::host::log::info(&format!("block {}", block.number));
    Ok(())
}

const BLOCK: &str = r#"{"number":21000000,"hash":"0x0000000000000000000000000000000000000000000000000000000000000001","parentHash":"0x0000000000000000000000000000000000000000000000000000000000000000","timestamp":1730000000,"gasUsed":"0x0","gasLimit":"0x1c9c380"}"#;

fn transfer_payload(value_hex: &str) -> Vec<u8> {
    let topic0 = format!(
        "{:#x}",
        <Transfer as superquery_sdk::sol_types::SolEvent>::SIGNATURE_HASH
    );
    format!(
        r#"{{"address":"0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48","logIndex":7,
            "topics":["{topic0}",
              "0x000000000000000000000000000000000000000000000000000000000000aaaa",
              "0x000000000000000000000000000000000000000000000000000000000000bbbb"],
            "data":"0x{value_hex:0>64}",
            "transactionHash":"0x{}","block":{BLOCK}}}"#,
        "cd".repeat(32)
    )
    .into_bytes()
}

#[test]
fn a_log_payload_runs_the_handler_and_saves_the_entity() {
    let store = TestStore::new();
    let _host = store.install();

    let status = __sq_dispatch_handle_transfer(&transfer_payload("f4240"));
    assert_eq!(status, HandlerStatus::Ok, "{:?}", store.handler_errors());

    let id = format!("0x{}-7", "cd".repeat(32));
    let saved = store.entity::<TransferEntity>(&id).expect("saved");
    assert_eq!(saved.value, BigInt::from(1_000_000u64));
    assert_eq!(saved.block_number, BigInt::from(21_000_000u64));
    // Address::to_string is EIP-55 checksummed; the bytes are what matter.
    assert!(
        saved
            .from
            .eq_ignore_ascii_case("0x000000000000000000000000000000000000aaaa")
    );
}

#[test]
fn a_mapping_error_reaches_the_host_with_its_message() {
    let store = TestStore::new();
    let _host = store.install();

    let status = __sq_dispatch_handle_transfer(&transfer_payload("0"));
    assert_eq!(status, HandlerStatus::Failed);
    assert_eq!(store.handler_errors(), ["zero-value transfer"]);
    assert!(store.is_empty());
}

#[test]
fn a_block_payload_sent_to_a_log_handler_is_a_bad_payload() {
    let store = TestStore::new();
    let _host = store.install();
    assert_eq!(
        __sq_dispatch_handle_transfer(BLOCK.as_bytes()),
        HandlerStatus::BadPayload
    );
}

#[test]
fn sync_handlers_are_dispatched_too() {
    let store = TestStore::new();
    let _host = store.install();
    assert_eq!(
        __sq_dispatch_count_blocks(BLOCK.as_bytes()),
        HandlerStatus::Ok
    );
    assert_eq!(store.logs()[0].1, "block 21000000");
}

#[test]
fn the_handler_itself_stays_callable_as_written() {
    let store = TestStore::new();
    let _host = store.install();
    let block: EvmBlock = serde_json::from_str(BLOCK).unwrap();
    assert!(count_blocks(block).is_ok());
}
