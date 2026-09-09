# Schema v1

`schema.graphql` and the canonical IR. Implemented by `crates/schema`.

Status: **draft**.

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

## Indexes

The IR carries every index the schema implies, so the node does not re-derive
them: the primary key on `id`, a foreign key per owned relation, and whatever
`@index`/`@unique` asked for.

## Canonical form

Entities and enums are sorted by name at parse time; fields keep declaration
order, because generated structs should match what the developer wrote.
`canonical_json` emits compact JSON and `schema_hash` is its SHA-256. That hash
goes in `build.json`, so any change to the IR's serialisation is a breaking
change to the build format.

## Consumers

The same IR feeds SDK entity codegen, the node's table/migration planning, and
the query service's GraphQL generation. Three consumers, one parser.
