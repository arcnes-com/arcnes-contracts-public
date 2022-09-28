# arcnes-contracts-public

Arcnes public contracts

## Commands

`yarn build` - build all contracts \
`yarn lint` - run linter \
`yarn test` - run unit tests \
`yarn test:unit` - run unit tests

## Deploy contract

1. `near create-account CONTRACT_ACCOUNT --masterAccount YOUR_ACCOUNT_HERE --initialBalance 10`
2. `near deploy --wasmFile artifacts/CONTRACT_BINARY.wasm --accountId CONTRACT_ACCOUNT`
