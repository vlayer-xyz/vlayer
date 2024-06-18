# Contributing


## Prerequisites

You will need to install following software to start working with repository:

- [Rust](https://www.rust-lang.org/tools/install) compiler
- Rust risc-0 [toolchain](https://dev.risczero.com/api/zkvm/quickstart)

## Building

To build project navigate to `rust` directory and type:

```sh
cargo build
```

## Running

To deploy contract first install `jq`:

```sh
brew install jq
```

Deploy contract by going to its directory (e.g. `examples/simple`) and run `../../bash/vlayer-build.sh`.
If `VLAYER_CONTRACT_ADDRESS` is displayed, contract was deployed successfully.

Finally run:

```sh
RUST_LOG=info RISC0_DEV_MODE=1 cargo run
```
