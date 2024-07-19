# Architecture overview

vlayer execution spans across three environments, each written in respective technology and consisting of related components:
- browser (js)
    - **javascript SDK** - thin wrapper around the [vlayer JSON-RPC API](/appendix/api.md)
    - **browser plugin** - used for notarization of TLS Connections    
- blockchain (solidity)
    - **on-chain smart contracts** - used to verify proofs [[docs](./solidity.md)]
- server infrastructure (rust)
    - **prover server** - exposing vlayer functionality via [vlayer JSON-RPC API](/appendix/api.md)  [[docs](./prover.md)]
    - **headers proof cache** - http server used as a cache for proofs of inclusion of a block in a chain
    - **notary server** - used to notarize TLS connections
    - **workers** - used to perform actual proving

![Schema](/images/architecture/overview.png)

All the above components can be found in the [monorepo](https://github.com/vlayer-xyz/vlayer). It also contains [sources](https://github.com/vlayer-xyz/vlayer/tree/main/book) of this book.
