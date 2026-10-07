# Schema v1

`schema.graphql` and the canonical IR. Implemented by `crates/schema`.

Status: **v1** — this document and `crates/schema` agree. Every rejection rule
below has a fixture under `crates/schema/tests/fixtures/invalid/`.

## Entities

```graphql
type Transfer @entity {
  id: ID!
  from: String!
  value: BigInt!
  block: Block!            # owned relation, stored as the target id
  memo: String             # nullable
}

type Block @entity {
  id: ID!
  transfers: [Transfer!]! @derivedFrom(field: "block")
}
```

Every entity requires `id: ID!`. It is the store's primary key, so it can be
neither nullable nor another type.

Every object type must carry `@entity`. Enums are declared with plain GraphQL
`enum` syntax and stored as their variant name.

## Scalars

| GraphQL | Rust (generated) | `Value` variant | Encoding |
|---|---|---|---|
| `ID` | `String` | `String` | — |
| `String` | `String` | `String` | — |
| `Boolean` | `bool` | `Boolean` | — |
| `Int` | `i32` | `Int` | 32-bit signed |
| `BigInt` | `BigInt` | `BigInt` | base-10 string |
| `Float` | `f64` | `Float` | IEEE-754 |
| `BigDecimal` | `BigDecimal` | `BigDecimal` | decimal string |
| `Bytes` | `Bytes` | `Bytes` | `0x`-prefixed hex |
| `Date` | `Timestamp` | `Date` | ms since epoch |
| `Json` | `Json` | `Json` | JSON document |

Arbitrary-precision numbers travel as strings so no host/guest pair has to
agree on a bignum memory layout.

## Directives

| Directive | On | Meaning |
|---|---|---|
| `@entity` | type | Store this type. Required. |
| `@entity(immutable: true)` | type | Opt out of historical versioning. |
| `@derivedFrom(field: "x")` | field | Computed by reverse lookup; stores nothing. |
| `@index` | field | Request a non-unique index. |
| `@unique` | field | Request a unique index. |

## Rejection rules

Each error names a location (`Entity` or `Entity.field`) and, where there is
one, a concrete fix.

| Rule | Location |
|---|---|
| object type without `@entity` | `Type` |
| entity without an `id` field | `Entity` |
| `id` that is not `ID!` | `Entity.id` |
| reference to an undeclared type | `Entity.field` |
| type or enum name declared twice | `Type` |
| type named like a built-in scalar | `Type` |
| field declared twice in one entity | `Entity.field` |
| `@derivedFrom` without `field:` | `Entity.field` |
| `@derivedFrom` on a non-entity field | `Entity.field` |
| a list of entities without `@derivedFrom` | `Entity.field` |
| `@index`/`@unique` on a `@derivedFrom` field | `Entity.field` |
| `@derivedFrom(field:)` naming a missing field | `Entity.field` |
| `@derivedFrom(field:)` naming a field that does not point back | `Entity.field` |

A list of entities must be derived because v1 has nowhere to store an owned
list: store the relation on the child and derive the list on the parent, or add
a join entity for many-to-many.

Syntax errors carry a source span over the offending token.

## Indexes

The IR carries every index the schema implies, so the node does not re-derive
them: the primary key on `id`, a foreign key per owned relation, and whatever
`@index`/`@unique` asked for.

## IR shape

`dist/schema.ir.json` is the IR serialised as below. Scalars use their GraphQL
spelling; type kinds and index kinds are camelCase.

```json
{
  "entities": [
    {
      "name": "Transfer",
      "description": "One ERC-20 Transfer event.",
      "fields": [
        { "name": "id", "type": { "kind": "scalar", "scalar": "ID" }, "nullable": false },
        { "name": "sender", "type": { "kind": "entity", "entity": "Account" }, "nullable": false,
          "relation": { "target": "Account", "kind": "owned" } },
        { "name": "tags", "type": { "kind": "list",
          "inner": { "kind": "scalar", "scalar": "String" }, "nullableElements": true },
          "nullable": true }
      ],
      "indexes": [
        { "fields": ["id"], "kind": "primaryKey", "unique": true },
        { "fields": ["sender"], "kind": "foreignKey", "unique": false }
      ],
      "immutable": false
    }
  ],
  "enums": [{ "name": "Side", "values": ["BUY", "SELL"] }]
}
```

Optional members (`description`, `relation`, `derivedFrom`, empty `indexes`,
empty `enums`) are omitted rather than written as `null`.

## Canonical form

Entities and enums are sorted by name at parse time; fields keep declaration
order, because generated structs should match what the developer wrote.
`canonical_json` emits compact JSON and `schema_hash` is its lowercase-hex
SHA-256. That hash goes in `build.json`, so any change to the IR's serialisation
is a breaking change to the build format. The template schema's canonical form
and hash are pinned as snapshots in `crates/schema/tests/snapshots/` so that
change cannot happen by accident.

Declaration order of types does not affect the hash; field order does, because
it is the generated struct's layout.

## Consumers

The same IR feeds SDK entity codegen, the node's table/migration planning, and
the query service's GraphQL generation. Three consumers, one parser.
