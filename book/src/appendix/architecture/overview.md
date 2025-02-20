# Architecture overview

vlayer execution spans across three environments, each written in respective technologies and consisting of related components:
- browser (js)
    - **javascript SDK** - thin wrapper around the vlayer JSON-RPC API
    - **browser plugin** - used for notarization of TLS Connections
- server infrastructure (rust)
    - [**prover server**](./prover.md) - exposing vlayer functionality via vlayer JSON-RPC API
    - [**chain proof cache**](./chain_proof.md) - http server used as a cache for proofs of inclusion of a block in a chain
    - **TLS Notary server** - used to notarize TLS connections
    - **DNS Notary server** - used to notarize DKIM DNS records
    - **workers** - used to perform actual proving
- blockchain (Solidity)
    - [**on-chain smart contracts**](./solidity.md) - used to verify proofs

![Schema](/images/architecture/overview.png)

All the above components can be found in the [monorepo](https://github.com/vlayer-xyz/vlayer). It also contains [sources](https://github.com/vlayer-xyz/vlayer/tree/main/book) of this book.
