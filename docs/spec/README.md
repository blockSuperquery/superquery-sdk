# Specifications

These four documents are the contract between this repository and the rest of
the platform. `superquery-node` and `superquery-query` should be implementable
against them alone, without reading this repo's source.

| Spec | Status | Consumed by |
|---|---|---|
| [`manifest-v1.md`](manifest-v1.md) | draft | node, CLI |
| [`schema-v1.md`](schema-v1.md) | draft | node, query, codegen |
| [`mapping-abi-v1.md`](mapping-abi-v1.md) | stub | node runtime, SDK guest |
| [`build-artifact-v1.md`](build-artifact-v1.md) | stub | node |

A spec is *draft* when the shape is implemented but the document lags the code,
and *stub* when the document is a placeholder. Milestone 0 is complete when all
four are accurate enough that a node developer never has to read `crates/`.
