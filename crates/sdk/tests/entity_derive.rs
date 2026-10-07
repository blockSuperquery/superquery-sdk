//! `#[derive(SuperQueryEntity)]` against the real store path.

use superquery_sdk::prelude::*;
use superquery_sdk::store::Entity;
use superquery_sdk::testing::{TestStore, block_on};
use superquery_types::{EntityId, EntityKey, Value};

#[derive(Clone, Debug, PartialEq, SuperQueryEntity)]
#[superquery(entity = "Transfer")]
pub struct TransferEntity {
    pub id: String,
    #[superquery(rename = "blockNumber")]
    pub block_number: BigInt,
    pub memo: Option<String>,
    pub tags: Vec<String>,
    pub r#type: String,
}

fn transfer(id: &str) -> TransferEntity {
    TransferEntity {
        id: id.to_owned(),
        block_number: BigInt::from(21_000_000u64),
        memo: None,
        tags: vec!["usdc".into()],
        r#type: "transfer".into(),
    }
}

#[test]
fn a_derived_entity_saves_under_its_schema_name_and_field_names() {
    let store = TestStore::new();
    let _host = store.install();

    block_on(transfer("0xabc-0").save()).unwrap();

    let key = EntityKey::new("Transfer", EntityId::new("0xabc-0").unwrap());
    let stored = store.get(&key).expect("saved under the schema name");
    assert_eq!(
        stored.get("blockNumber"),
        Some(&Value::BigInt("21000000".into()))
    );
    assert_eq!(stored.get("memo"), Some(&Value::Null));
    assert_eq!(stored.get("type"), Some(&Value::String("transfer".into())));
    assert_eq!(
        stored.get("block_number"),
        None,
        "Rust names never reach the wire"
    );
}

#[test]
fn a_derived_entity_round_trips_through_the_store() {
    let store = TestStore::new();
    let _host = store.install();

    block_on(transfer("0xabc-0").save()).unwrap();
    assert_eq!(
        store.entity::<TransferEntity>("0xabc-0"),
        Some(transfer("0xabc-0"))
    );
}

#[test]
fn an_empty_id_fails_the_save_instead_of_panicking() {
    let store = TestStore::new();
    let _host = store.install();

    let err = block_on(transfer("").save()).unwrap_err();
    assert!(err.to_string().contains("Transfer"), "{err}");
    assert!(store.is_empty());
}

#[test]
fn decoding_a_wrongly_typed_field_names_the_field() {
    let mut untyped = transfer("x").to_untyped();
    untyped.set("blockNumber", "not a bigint");
    let err = TransferEntity::from_untyped(&untyped).unwrap_err();
    assert!(err.to_string().contains("field `blockNumber`"), "{err}");
}
