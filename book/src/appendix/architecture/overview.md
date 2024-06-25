# Architecture overview

vlayer consist of four main components:
- **prover server** exposing functionality via [vlayer JSON-RPC API](/appendix/api.md)  [[docs](./prover.md)]
- **javascript SDK** - thin wrapper around the [vlayer JSON-RPC API](/appendix/api.md)
- **on-chain smart contracts** used to verify proofs [[docs](./solidity.md)]
- **browser plugin** used for notarization of TLS Connections

All above elements can be found in a [monorepo](https://github.com/vlayer-xyz/vlayer). It also contains [sources](https://github.com/vlayer-xyz/vlayer/tree/main/book) of this book.
