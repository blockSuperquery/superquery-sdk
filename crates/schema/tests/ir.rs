//! What a valid schema resolves to, and how stably it hashes.

use superquery_schema::{
    FieldType, IndexDefinition, IndexKind, RelationKind, SchemaIr, canonical_json, parse,
    schema_hash, validate,
};
use superquery_types::ScalarKind;

const MARKETPLACE: &str = r#"
enum Side {
  BUY
  SELL
}

"""
A trading account.
"""
type Account @entity {
  id: ID!
  "Display name, if the owner set one."
  name: String
  handle: String! @unique
  orders: [Order!]! @derivedFrom(field: "owner")
}

type Order @entity(immutable: true) {
  id: ID!
  owner: Account!
  side: Side!
  price: BigDecimal! @index
  tags: [String]
  fills: [BigInt!]!
}
"#;

fn ir(source: &str) -> SchemaIr {
    let ir = parse(source, "test.graphql").expect("fixture parses");
    validate(&ir).expect("fixture validates");
    ir
}

#[test]
fn entities_and_enums_are_sorted_but_fields_keep_declaration_order() {
    let ir = ir(MARKETPLACE);
    let entities: Vec<_> = ir.entities.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(entities, ["Account", "Order"]);

    let order_fields: Vec<_> = ir
        .entity("Order")
        .unwrap()
        .fields
        .iter()
        .map(|f| f.name.as_str())
        .collect();
    assert_eq!(
        order_fields,
        ["id", "owner", "side", "price", "tags", "fills"]
    );

    assert_eq!(ir.enum_type("Side").unwrap().values, ["BUY", "SELL"]);
}

#[test]
fn descriptions_and_the_immutable_flag_reach_the_ir() {
    let ir = ir(MARKETPLACE);
    let account = ir.entity("Account").unwrap();
    assert_eq!(account.description.as_deref(), Some("A trading account."));
    assert_eq!(
        account.field("name").unwrap().description.as_deref(),
        Some("Display name, if the owner set one.")
    );
    assert!(!account.immutable);
    assert!(ir.entity("Order").unwrap().immutable);
}

#[test]
fn owned_and_derived_relations_resolve_to_opposite_kinds() {
    let ir = ir(MARKETPLACE);

    let owner = ir.entity("Order").unwrap().field("owner").unwrap();
    let owned = owner.relation.as_ref().unwrap();
    assert_eq!(
        (owned.target.as_str(), owned.kind),
        ("Account", RelationKind::Owned)
    );
    assert_eq!(owned.derived_from, None);

    let orders = ir.entity("Account").unwrap().field("orders").unwrap();
    let derived = orders.relation.as_ref().unwrap();
    assert_eq!(
        (derived.target.as_str(), derived.kind),
        ("Order", RelationKind::Derived)
    );
    assert_eq!(derived.derived_from.as_deref(), Some("owner"));
}

#[test]
fn nullability_is_tracked_for_the_field_and_for_list_elements() {
    let ir = ir(MARKETPLACE);
    let order = ir.entity("Order").unwrap();

    let tags = order.field("tags").unwrap();
    assert!(tags.nullable);
    assert_eq!(
        tags.field_type,
        FieldType::List {
            inner: Box::new(FieldType::Scalar {
                scalar: ScalarKind::String
            }),
            nullable_elements: true,
        }
    );

    let fills = order.field("fills").unwrap();
    assert!(!fills.nullable);
    assert!(matches!(
        fills.field_type,
        FieldType::List {
            nullable_elements: false,
            ..
        }
    ));

    assert_eq!(
        order.field("side").unwrap().field_type,
        FieldType::Enum {
            name: "Side".into()
        }
    );
}

#[test]
fn indexes_cover_the_primary_key_owned_relations_and_explicit_requests() {
    let ir = ir(MARKETPLACE);

    assert_eq!(
        ir.entity("Order").unwrap().indexes,
        [
            IndexDefinition::primary_key(),
            IndexDefinition::foreign_key("owner"),
            IndexDefinition::explicit(vec!["price".into()], false),
        ]
    );

    // A derived relation stores nothing, so it implies no index; `@unique`
    // implies a unique one.
    let account = &ir.entity("Account").unwrap().indexes;
    assert_eq!(account.len(), 2);
    assert_eq!(
        (account[1].kind, account[1].unique),
        (IndexKind::Explicit, true)
    );
}

#[test]
fn declaration_order_does_not_change_the_hash() {
    let forwards = "type A @entity { id: ID! }\ntype B @entity { id: ID! }\nenum E { X }";
    let backwards = "enum E { X }\ntype B @entity { id: ID! }\ntype A @entity { id: ID! }";
    assert_eq!(schema_hash(&ir(forwards)), schema_hash(&ir(backwards)));
}

#[test]
fn field_order_does_change_the_hash() {
    // Field order is part of the developer's intent (it is the struct
    // layout), so reordering fields is a real schema change.
    let a = "type A @entity { id: ID! x: Int y: Int }";
    let b = "type A @entity { id: ID! y: Int x: Int }";
    assert_ne!(schema_hash(&ir(a)), schema_hash(&ir(b)));
}

#[test]
fn canonical_json_round_trips_to_the_same_ir() {
    let ir = ir(MARKETPLACE);
    let back: SchemaIr = serde_json::from_str(&canonical_json(&ir)).unwrap();
    assert_eq!(back, ir);
}

#[test]
fn the_template_schema_has_a_pinned_canonical_form() {
    // This snapshot is the build format. If it changes, every deployed
    // bundle's `schemaHash` stops matching, so update it only on purpose.
    let source = include_str!("../../../templates/evm/schema.graphql");
    let ir = ir(source);
    insta::assert_snapshot!("template_canonical_json", canonical_json(&ir));
    insta::assert_snapshot!("template_schema_hash", schema_hash(&ir));
}
