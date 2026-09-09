export const links = {
  organization: 'https://github.com/blockSuperquery',
  sdk: 'https://github.com/blockSuperquery/superquery-sdk',
  node: 'https://github.com/blockSuperquery/superquery-node',
  query: 'https://github.com/blockSuperquery/superquery-query'
}

export const repositories = [
  { title: 'Define with the SDK', description: 'Validate your manifest, turn GraphQL entities into Rust types, and generate contract bindings.', icon: 'i-lucide-terminal', to: '/sdk', label: '01 / DEVELOPER TOOLING' },
  { title: 'Index with the node', description: 'The indexing engine brings together chain adapters, WASM mappings, and PostgreSQL storage.', icon: 'i-lucide-blocks', to: links.node, target: '_blank', label: '02 / INDEXING ENGINE' },
  { title: 'Serve with GraphQL', description: 'Read indexed entities with typed filters, cursor pagination, relations, and query limits.', icon: 'i-lucide-database', to: links.query, target: '_blank', label: '03 / QUERY SERVICE' }
]

export const quickstart = `git clone https://github.com/blockSuperquery/superquery-sdk.git
cd superquery-sdk
cargo run --locked -p superquery-cli -- validate -m templates/evm/project.yaml
cargo run --locked -p superquery-cli -- codegen --dry-run -m templates/evm/project.yaml`

export const transferSchema = `type Transfer @entity {
  id: ID!
  from: String!
  to: String!
  value: BigInt!
  blockNumber: BigInt!
  timestamp: Date!
  transactionHash: Bytes!
}`

export const transferQuery = `{
  transfers(first: 10) {
    nodes {
      id
      from
      to
      value
    }
    pageInfo { hasNextPage endCursor }
  }
}`

export const commands = [
  { command: 'validate', state: 'Available', description: 'Check the manifest, schema, assets, and handler filters.' },
  { command: 'codegen', state: 'Available', description: 'Generate Rust entities, schema metadata, and EVM contract bindings.' },
  { command: 'doctor', state: 'Available', description: 'Inspect your toolchain and project prerequisites.' },
  { command: 'init', state: 'Planned', description: 'Scaffold a project from a starter template. Milestone 8.' },
  { command: 'build', state: 'Planned', description: 'Compile WASM mappings and assemble a versioned artifact. Milestone 9.' },
  { command: 'test', state: 'Planned', description: 'Run mapping fixtures through the CLI. Milestone 10.' }
]
