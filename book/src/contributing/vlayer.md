# Contributing to vlayer

## Prerequisites

To start working with this repository, you will need to install following software:

- [Rust](https://www.rust-lang.org/tools/install) compiler
- Rust risc-0 [toolchain](https://dev.risczero.com/api/zkvm/quickstart)
- [Foundry](https://book.getfoundry.sh/getting-started/installation)

## Building

To build vlayer project, firs, navigate to the `rust` directory and type:

```sh
cargo build
```

Before you build solidity smart contracts, make sure that dependencies are up to date:
```sh
git submodule update --init --recursive
```

Next, navigate to `contracts` and type:
```sh
forge build
```

## Running

To deploy contract, first install `jq`:

```sh
brew install jq
```

Run anvil in the background:
```sh
anvil
```

Deploy the contract by going to its directory (e.g. `examples/simple`) and running `../../bash/vlayer-deploy.sh`.
If `VLAYER_CONTRACT_ADDRESS` is displayed, contract was deployed successfully.

Finally, run:

```sh
RUST_LOG=info RISC0_DEV_MODE=1 cargo run
```

For guides about the project structure, check out [architecture appendix](/appendix/architecture.md).

