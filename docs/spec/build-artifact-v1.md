# Build artifact v1

What `superquery build` produces and `superquery-node` consumes.

Status: **stub** — write this before implementing Milestone 9.

## Bundle

```text
dist/
|-- manifest.json      the manifest, normalised to JSON
|-- schema.graphql     the schema as written
|-- schema.ir.json     the canonical IR
|-- mapping.wasm       the compiled mapping
|-- assets/            ABIs and other declared assets
`-- build.json         the manifest of the bundle itself
```

```json
{
  "formatVersion": 1,
  "mappingAbiVersion": 1,
  "manifestHash": "...",
  "schemaHash": "...",
  "mappingHash": "..."
}
```

## Rules

**The bundle is self-contained.** The node never reads the developer's source
tree. That is what makes Docker deployment, artifact hashing and future
decentralized deployment tractable.

**The bundle is read-only.** The node does not write into `dist/`.

**Builds are reproducible.** Same inputs, byte-identical output, including the
hashes. Test it by building twice and comparing — a build that is 99%
reproducible is not reproducible.

## Still to decide

- Whether hashes cover the file bytes or the canonical form of their contents.
- Whether `dist/` is a directory, a tarball, or both.
- How the node reports a hash mismatch, and whether it ever proceeds anyway.
